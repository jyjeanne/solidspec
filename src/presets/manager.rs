use std::path::Path;

use anyhow::{Result, bail};

use super::manifest::PresetManifest;
use super::registry::{PresetEntry, PresetRegistry};

/// Install a preset from a local directory.
pub fn add_preset(project_root: &Path, source_dir: &Path, priority: u32) -> Result<String> {
    let manifest_path = source_dir.join("preset.yml");
    if !manifest_path.exists() {
        bail!("No preset.yml found at {}", source_dir.display());
    }

    let manifest = PresetManifest::load(&manifest_path)?;
    let preset_id = manifest.preset.id.clone();

    // Load registry
    let registry_path = project_root.join(".solidspec/presets/.registry");
    let mut registry = PresetRegistry::load(&registry_path)?;

    // Create preset directory and copy files
    let target_dir = project_root.join(".solidspec/presets").join(&preset_id);
    if target_dir.exists() {
        bail!(
            "Preset directory already exists. Remove '{}' first.",
            preset_id
        );
    }

    copy_dir_recursive(source_dir, &target_dir)?;

    // Register in registry
    let entry = PresetEntry {
        id: preset_id.clone(),
        name: manifest.preset.name,
        version: manifest.preset.version,
        priority,
        description: manifest.preset.description,
        installed_at: chrono::Utc::now().to_rfc3339(),
    };
    registry.add(entry)?;
    registry.save(&registry_path)?;

    Ok(preset_id)
}

/// Remove an installed preset.
pub fn remove_preset(project_root: &Path, preset_id: &str) -> Result<()> {
    let registry_path = project_root.join(".solidspec/presets/.registry");
    let mut registry = PresetRegistry::load(&registry_path)?;

    registry.remove(preset_id)?;

    // Remove preset directory
    let preset_dir = project_root.join(".solidspec/presets").join(preset_id);
    if preset_dir.exists() {
        std::fs::remove_dir_all(&preset_dir)?;
    }

    registry.save(&registry_path)?;
    Ok(())
}

/// List installed presets.
pub fn list_presets(project_root: &Path) -> Result<Vec<PresetEntry>> {
    let registry_path = project_root.join(".solidspec/presets/.registry");
    let registry = PresetRegistry::load(&registry_path)?;
    Ok(registry.list())
}

/// Search presets by keyword.
pub fn search_presets(project_root: &Path, query: &str) -> Result<Vec<PresetEntry>> {
    let registry_path = project_root.join(".solidspec/presets/.registry");
    let registry = PresetRegistry::load(&registry_path)?;
    Ok(registry.search(query))
}

/// Get info about a specific preset.
pub fn info_preset(project_root: &Path, preset_id: &str) -> Result<Option<PresetEntry>> {
    let registry_path = project_root.join(".solidspec/presets/.registry");
    let registry = PresetRegistry::load(&registry_path)?;
    Ok(registry.get(preset_id))
}

/// Get sorted priorities for template resolution.
pub fn get_preset_priorities(project_root: &Path) -> Result<Vec<(String, u32)>> {
    let registry_path = project_root.join(".solidspec/presets/.registry");
    let registry = PresetRegistry::load(&registry_path)?;
    Ok(registry.sorted_priorities())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();

        // Skip hidden files, symlinks, and path traversal attempts
        if name_str.starts_with('.') || name_str.contains("..") {
            continue;
        }

        let src_path = entry.path();
        let dst_path = dst.join(&file_name);

        // Skip symlinks for security
        let metadata = std::fs::symlink_metadata(&src_path)?;
        if metadata.file_type().is_symlink() {
            log::warn!("Skipping symlink: {}", src_path.display());
            continue;
        }

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
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
        std::fs::create_dir_all(dir.join(".solidspec/presets")).unwrap();
        std::fs::write(dir.join(".solidspec/presets/.registry"), "{}").unwrap();
    }

    fn create_preset_source(dir: &Path, id: &str) -> std::path::PathBuf {
        let src = dir.join(format!("source-{id}"));
        std::fs::create_dir_all(src.join("templates")).unwrap();
        std::fs::write(
            src.join("preset.yml"),
            format!(
                "schema_version: '1.0'\npreset:\n  id: {id}\n  name: Preset {id}\n  version: '1.0.0'\n  description: 'Test'\nprovides:\n  templates:\n    - type: template\n      name: spec-template\n      file: templates/spec-template.md\n"
            ),
        ).unwrap();
        std::fs::write(
            src.join("templates/spec-template.md"),
            format!("CUSTOM FROM {id}"),
        )
        .unwrap();
        src
    }

    #[test]
    fn add_preset_copies_files_and_registers() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src = create_preset_source(dir.path(), "my-preset");

        let id = add_preset(dir.path(), &src, 10).unwrap();
        assert_eq!(id, "my-preset");

        // Files copied
        assert!(
            dir.path()
                .join(".solidspec/presets/my-preset/preset.yml")
                .exists()
        );
        assert!(
            dir.path()
                .join(".solidspec/presets/my-preset/templates/spec-template.md")
                .exists()
        );

        // Registry updated
        let reg = PresetRegistry::load(&dir.path().join(".solidspec/presets/.registry")).unwrap();
        let entry = reg.get("my-preset").unwrap();
        assert_eq!(entry.priority, 10);
    }

    #[test]
    fn add_same_preset_twice_errors() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src = create_preset_source(dir.path(), "dupe");

        add_preset(dir.path(), &src, 10).unwrap();
        assert!(add_preset(dir.path(), &src, 5).is_err());
    }

    #[test]
    fn remove_deletes_files_and_registry() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src = create_preset_source(dir.path(), "to-remove");
        add_preset(dir.path(), &src, 10).unwrap();

        remove_preset(dir.path(), "to-remove").unwrap();
        assert!(!dir.path().join(".solidspec/presets/to-remove").exists());

        let reg = PresetRegistry::load(&dir.path().join(".solidspec/presets/.registry")).unwrap();
        assert!(reg.get("to-remove").is_none());
    }

    #[test]
    fn remove_nonexistent_errors() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        assert!(remove_preset(dir.path(), "ghost").is_err());
    }

    #[test]
    fn list_returns_sorted_by_priority() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src_a = create_preset_source(dir.path(), "preset-a");
        let src_b = create_preset_source(dir.path(), "preset-b");

        add_preset(dir.path(), &src_a, 10).unwrap();
        add_preset(dir.path(), &src_b, 1).unwrap();

        let list = list_presets(dir.path()).unwrap();
        assert_eq!(list[0].id, "preset-b"); // priority 1 first
        assert_eq!(list[1].id, "preset-a"); // priority 10 second
    }

    #[test]
    fn search_filters_by_keyword() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src = create_preset_source(dir.path(), "testing-preset");
        add_preset(dir.path(), &src, 1).unwrap();

        let results = search_presets(dir.path(), "testing").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn info_returns_entry() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src = create_preset_source(dir.path(), "info-test");
        add_preset(dir.path(), &src, 5).unwrap();

        let entry = info_preset(dir.path(), "info-test").unwrap().unwrap();
        assert_eq!(entry.priority, 5);
    }

    #[test]
    fn info_missing_returns_none() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        assert!(info_preset(dir.path(), "ghost").unwrap().is_none());
    }

    #[test]
    fn priorities_for_resolver() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let src_a = create_preset_source(dir.path(), "preset-a");
        let src_b = create_preset_source(dir.path(), "preset-b");
        add_preset(dir.path(), &src_a, 10).unwrap();
        add_preset(dir.path(), &src_b, 1).unwrap();

        let priorities = get_preset_priorities(dir.path()).unwrap();
        assert_eq!(priorities[0].0, "preset-b");
        assert_eq!(priorities[0].1, 1);
    }
}
