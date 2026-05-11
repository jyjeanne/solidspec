use std::collections::HashMap;
use std::path::Path;

use anyhow::{Result, bail};

use super::manifest::ExtensionManifest;
use super::registry::{ExtensionEntry, ExtensionRegistry, HookEntry};

/// Install an extension from a local directory (--dev).
pub fn add_extension_dev(project_root: &Path, source_dir: &Path) -> Result<String> {
    let manifest_path = source_dir.join("extension.yml");
    if !manifest_path.exists() {
        bail!("No extension.yml found at {}", source_dir.display());
    }

    let manifest = ExtensionManifest::load(&manifest_path)?;
    let ext_id = manifest.extension.id.clone();

    let registry_path = project_root.join(".solidspec/extensions/.registry");
    let mut registry = ExtensionRegistry::load(&registry_path)?;

    // Copy to extensions dir
    let target_dir = project_root.join(".solidspec/extensions").join(&ext_id);
    if target_dir.exists() {
        bail!(
            "Extension directory already exists. Remove '{}' first.",
            ext_id
        );
    }
    copy_dir_safe(source_dir, &target_dir)?;

    // Build registry entry
    let entry = build_entry(&manifest, true);
    registry.add(entry)?;
    registry.save(&registry_path)?;

    Ok(ext_id)
}

/// Remove an installed extension.
pub fn remove_extension(project_root: &Path, ext_id: &str) -> Result<()> {
    let registry_path = project_root.join(".solidspec/extensions/.registry");
    let mut registry = ExtensionRegistry::load(&registry_path)?;

    registry.remove(ext_id)?;

    let ext_dir = project_root.join(".solidspec/extensions").join(ext_id);
    if ext_dir.exists() {
        std::fs::remove_dir_all(&ext_dir)?;
    }

    registry.save(&registry_path)?;
    Ok(())
}

/// Enable a disabled extension.
pub fn enable_extension(project_root: &Path, ext_id: &str) -> Result<()> {
    let registry_path = project_root.join(".solidspec/extensions/.registry");
    let mut registry = ExtensionRegistry::load(&registry_path)?;
    registry.enable(ext_id)?;
    registry.save(&registry_path)?;
    Ok(())
}

/// Disable an extension (unregister commands, keep files).
pub fn disable_extension(project_root: &Path, ext_id: &str) -> Result<()> {
    let registry_path = project_root.join(".solidspec/extensions/.registry");
    let mut registry = ExtensionRegistry::load(&registry_path)?;
    registry.disable(ext_id)?;
    registry.save(&registry_path)?;
    Ok(())
}

pub fn list_extensions(project_root: &Path) -> Result<Vec<ExtensionEntry>> {
    let registry_path = project_root.join(".solidspec/extensions/.registry");
    let registry = ExtensionRegistry::load(&registry_path)?;
    Ok(registry.list())
}

pub fn search_extensions(project_root: &Path, query: &str) -> Result<Vec<ExtensionEntry>> {
    let registry_path = project_root.join(".solidspec/extensions/.registry");
    let registry = ExtensionRegistry::load(&registry_path)?;
    Ok(registry.search(query))
}

pub fn info_extension(project_root: &Path, name: &str) -> Result<Option<ExtensionEntry>> {
    let registry_path = project_root.join(".solidspec/extensions/.registry");
    let registry = ExtensionRegistry::load(&registry_path)?;
    registry.resolve(name)
}

pub fn load_registry(project_root: &Path) -> Result<ExtensionRegistry> {
    let registry_path = project_root.join(".solidspec/extensions/.registry");
    ExtensionRegistry::load(&registry_path)
}

fn build_entry(manifest: &ExtensionManifest, dev: bool) -> ExtensionEntry {
    // Map hook command names to their file paths from provides.commands
    let cmd_file_map: HashMap<String, String> = manifest
        .provides
        .commands
        .iter()
        .map(|cmd| (cmd.name.clone(), cmd.file.clone()))
        .collect();

    let hooks: Vec<HookEntry> = manifest
        .hooks
        .iter()
        .filter_map(|(trigger, def)| match cmd_file_map.get(&def.command) {
            Some(file) => Some(HookEntry {
                trigger: trigger.clone(),
                file: file.clone(),
            }),
            None => {
                log::error!(
                    "Hook '{}' references unknown command '{}' — skipping",
                    trigger,
                    def.command
                );
                None
            }
        })
        .collect();

    ExtensionEntry {
        id: manifest.extension.id.clone(),
        name: manifest.extension.name.clone(),
        version: manifest.extension.version.clone(),
        installed_timestamp: chrono::Utc::now().to_rfc3339(),
        enabled: true,
        commands: HashMap::new(),
        hooks,
        templates: vec![],
        dev,
    }
}

fn copy_dir_safe(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();

        if name_str.starts_with('.') || name_str.contains("..") {
            continue;
        }

        let src_path = entry.path();
        let dst_path = dst.join(&file_name);

        let metadata = std::fs::symlink_metadata(&src_path)?;
        if metadata.file_type().is_symlink() {
            log::warn!("Skipping symlink: {}", src_path.display());
            continue;
        }

        if src_path.is_dir() {
            copy_dir_safe(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_project(dir: &Path) {
        std::fs::create_dir_all(dir.join(".solidspec/extensions")).unwrap();
        std::fs::write(dir.join(".solidspec/extensions/.registry"), "{}").unwrap();
    }

    fn create_ext_source(dir: &Path, id: &str) -> std::path::PathBuf {
        let src = dir.join(format!("ext-source-{id}"));
        std::fs::create_dir_all(src.join("commands")).unwrap();
        std::fs::write(
            src.join("extension.yml"),
            format!(
                r#"
schema_version: "1.0"
extension:
  id: {id}
  name: "Extension {id}"
  version: "1.0.0"
  description: "Test"
provides:
  commands:
    - name: solidspec.{id}.cmd
      file: commands/cmd.md
      description: "Test command"
"#
            ),
        )
        .unwrap();
        std::fs::write(src.join("commands/cmd.md"), "# Test command").unwrap();
        src
    }

    #[test]
    fn add_dev_installs_extension() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src = create_ext_source(dir.path(), "my-ext");

        let id = add_extension_dev(dir.path(), &src).unwrap();
        assert_eq!(id, "my-ext");
        assert!(
            dir.path()
                .join(".solidspec/extensions/my-ext/extension.yml")
                .exists()
        );
    }

    #[test]
    fn add_without_manifest_errors() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let empty = dir.path().join("empty-source");
        std::fs::create_dir_all(&empty).unwrap();

        assert!(add_extension_dev(dir.path(), &empty).is_err());
    }

    #[test]
    fn dev_flag_set_in_registry() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src = create_ext_source(dir.path(), "dev-ext");
        add_extension_dev(dir.path(), &src).unwrap();

        let reg = load_registry(dir.path()).unwrap();
        assert!(reg.get("dev-ext").unwrap().dev);
    }

    #[test]
    fn remove_cleans_up() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src = create_ext_source(dir.path(), "rm-ext");
        add_extension_dev(dir.path(), &src).unwrap();

        remove_extension(dir.path(), "rm-ext").unwrap();
        assert!(!dir.path().join(".solidspec/extensions/rm-ext").exists());
        let reg = load_registry(dir.path()).unwrap();
        assert!(reg.get("rm-ext").is_none());
    }

    #[test]
    fn remove_nonexistent_errors() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        assert!(remove_extension(dir.path(), "ghost").is_err());
    }

    #[test]
    fn enable_disable_toggle() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src = create_ext_source(dir.path(), "toggle");
        add_extension_dev(dir.path(), &src).unwrap();

        disable_extension(dir.path(), "toggle").unwrap();
        let reg = load_registry(dir.path()).unwrap();
        assert!(!reg.get("toggle").unwrap().enabled);
        assert!(dir.path().join(".solidspec/extensions/toggle").exists()); // files still on disk

        enable_extension(dir.path(), "toggle").unwrap();
        let reg = load_registry(dir.path()).unwrap();
        assert!(reg.get("toggle").unwrap().enabled);
    }

    #[test]
    fn disable_already_disabled_noop() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src = create_ext_source(dir.path(), "dis");
        add_extension_dev(dir.path(), &src).unwrap();

        disable_extension(dir.path(), "dis").unwrap();
        disable_extension(dir.path(), "dis").unwrap(); // no error
    }

    #[test]
    fn list_shows_all() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src_a = create_ext_source(dir.path(), "ext-a");
        let src_b = create_ext_source(dir.path(), "ext-b");
        add_extension_dev(dir.path(), &src_a).unwrap();
        add_extension_dev(dir.path(), &src_b).unwrap();

        let list = list_extensions(dir.path()).unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn search_filters() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src = create_ext_source(dir.path(), "lint-ext");
        add_extension_dev(dir.path(), &src).unwrap();

        let results = search_extensions(dir.path(), "lint").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn info_by_id() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src = create_ext_source(dir.path(), "info-ext");
        add_extension_dev(dir.path(), &src).unwrap();

        let entry = info_extension(dir.path(), "info-ext").unwrap().unwrap();
        assert_eq!(entry.id, "info-ext");
    }

    #[test]
    fn info_missing_returns_none() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        assert!(info_extension(dir.path(), "ghost").unwrap().is_none());
    }
}
