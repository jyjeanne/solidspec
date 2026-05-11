use std::path::{Path, PathBuf};

/// Resolution layers in priority order (highest first).
/// 1. overrides/ — project-specific tweaks
/// 2. presets/<id>/templates/ — sorted by priority (lower number = higher precedence)
/// 3. extensions/<id>/templates/ — extension-provided
/// 4. Embedded defaults (returned as None path — caller uses include_str!)
#[derive(Debug, Clone)]
pub struct ResolvedTemplate {
    pub path: Option<PathBuf>,
    pub source: TemplateSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateSource {
    Override,
    Preset(String),    // preset id
    Extension(String), // extension id
    EmbeddedDefault,
}

use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static RESOLVE_CACHE: RefCell<HashMap<(String, String), ResolvedTemplate>> =
        RefCell::new(HashMap::new());
}

/// Resolve a template name with caching — avoids redundant filesystem traversal
/// when the same template is requested multiple times in a process lifetime.
pub fn resolve_cached(
    template_name: &str,
    project_root: &Path,
    preset_priorities: &[(String, u32)],
) -> ResolvedTemplate {
    let cache_key = (
        template_name.to_string(),
        project_root.to_string_lossy().to_string(),
    );
    RESOLVE_CACHE.with(|cache| {
        if let Some(cached) = cache.borrow().get(&cache_key) {
            log::debug!("Template cache hit: {template_name}");
            return cached.clone();
        }
        let result = resolve(template_name, project_root, preset_priorities);
        cache.borrow_mut().insert(cache_key, result.clone());
        result
    })
}

/// Resolve a template name through the 4-layer hierarchy.
/// Returns the path to the winning template file, or None for embedded default.
pub fn resolve(
    template_name: &str,
    project_root: &Path,
    preset_priorities: &[(String, u32)], // (preset_id, priority) sorted by priority
) -> ResolvedTemplate {
    let solidspec_dir = project_root.join(".solidspec");

    // Layer 1: overrides/
    let override_path = solidspec_dir
        .join("templates/overrides")
        .join(template_name);
    if override_path.exists() {
        return ResolvedTemplate {
            path: Some(override_path),
            source: TemplateSource::Override,
        };
    }

    // Layer 2: presets (sorted by priority — lower number = higher precedence)
    for (preset_id, _priority) in preset_priorities {
        let preset_path = solidspec_dir
            .join("presets")
            .join(preset_id)
            .join("templates")
            .join(template_name);
        if preset_path.exists() {
            return ResolvedTemplate {
                path: Some(preset_path),
                source: TemplateSource::Preset(preset_id.clone()),
            };
        }
    }

    // Layer 3: extensions
    let extensions_dir = solidspec_dir.join("extensions");
    if extensions_dir.exists()
        && let Ok(entries) = std::fs::read_dir(&extensions_dir)
    {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with('.') {
                    continue; // skip .registry, .cache
                }
                let ext_path = entry.path().join("templates").join(template_name);
                if ext_path.exists() {
                    return ResolvedTemplate {
                        path: Some(ext_path),
                        source: TemplateSource::Extension(name),
                    };
                }
            }
        }
    }

    // Layer 4: embedded default
    ResolvedTemplate {
        path: None,
        source: TemplateSource::EmbeddedDefault,
    }
}

/// Load the resolved template content as a string (uses cache).
pub fn load_template(
    template_name: &str,
    project_root: &Path,
    preset_priorities: &[(String, u32)],
) -> std::io::Result<(String, TemplateSource)> {
    let resolved = resolve_cached(template_name, project_root, preset_priorities);

    match resolved.path {
        Some(path) => {
            let content = std::fs::read_to_string(&path)?;
            Ok((content, resolved.source))
        }
        None => {
            // Embedded default
            let content = match template_name {
                "spec-template.md" => super::embedded::SPEC_TEMPLATE,
                "plan-template.md" => super::embedded::PLAN_TEMPLATE,
                "tasks-template.md" => super::embedded::TASKS_TEMPLATE,
                "checklist-template.md" => super::embedded::CHECKLIST_TEMPLATE,
                "constitution-template.md" => super::embedded::CONSTITUTION_TEMPLATE,
                "agent-file-template.md" => super::embedded::AGENT_FILE_TEMPLATE,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("Unknown template: {template_name}"),
                    ));
                }
            };
            Ok((content.to_string(), TemplateSource::EmbeddedDefault))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_project(dir: &Path) {
        std::fs::create_dir_all(dir.join(".solidspec/templates/overrides")).unwrap();
        std::fs::create_dir_all(dir.join(".solidspec/presets")).unwrap();
        std::fs::create_dir_all(dir.join(".solidspec/extensions")).unwrap();
    }

    #[test]
    fn override_present_wins() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        std::fs::write(
            dir.path()
                .join(".solidspec/templates/overrides/spec-template.md"),
            "OVERRIDE CONTENT",
        )
        .unwrap();

        let result = resolve("spec-template.md", dir.path(), &[]);
        assert_eq!(result.source, TemplateSource::Override);
        assert!(result.path.is_some());
    }

    #[test]
    fn preset_wins_when_no_override() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let preset_dir = dir.path().join(".solidspec/presets/my-preset/templates");
        std::fs::create_dir_all(&preset_dir).unwrap();
        std::fs::write(preset_dir.join("spec-template.md"), "PRESET CONTENT").unwrap();

        let priorities = vec![("my-preset".to_string(), 10)];
        let result = resolve("spec-template.md", dir.path(), &priorities);
        assert_eq!(result.source, TemplateSource::Preset("my-preset".into()));
    }

    #[test]
    fn extension_wins_when_no_override_or_preset() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        let ext_dir = dir.path().join(".solidspec/extensions/my-ext/templates");
        std::fs::create_dir_all(&ext_dir).unwrap();
        std::fs::write(ext_dir.join("spec-template.md"), "EXT CONTENT").unwrap();

        let result = resolve("spec-template.md", dir.path(), &[]);
        assert_eq!(result.source, TemplateSource::Extension("my-ext".into()));
    }

    #[test]
    fn embedded_default_when_nothing_else() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());

        let result = resolve("spec-template.md", dir.path(), &[]);
        assert_eq!(result.source, TemplateSource::EmbeddedDefault);
        assert!(result.path.is_none());
    }

    #[test]
    fn override_beats_preset() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());

        // Both override and preset exist
        std::fs::write(
            dir.path()
                .join(".solidspec/templates/overrides/spec-template.md"),
            "OVERRIDE",
        )
        .unwrap();
        let preset_dir = dir.path().join(".solidspec/presets/my-preset/templates");
        std::fs::create_dir_all(&preset_dir).unwrap();
        std::fs::write(preset_dir.join("spec-template.md"), "PRESET").unwrap();

        let priorities = vec![("my-preset".to_string(), 10)];
        let result = resolve("spec-template.md", dir.path(), &priorities);
        assert_eq!(result.source, TemplateSource::Override);
    }

    #[test]
    fn lower_priority_number_preset_wins() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());

        // Two presets with different priorities
        for (id, _) in &[("high-priority", 1), ("low-priority", 10)] {
            let preset_dir = dir
                .path()
                .join(format!(".solidspec/presets/{id}/templates"));
            std::fs::create_dir_all(&preset_dir).unwrap();
            std::fs::write(
                preset_dir.join("spec-template.md"),
                format!("CONTENT FROM {id}"),
            )
            .unwrap();
        }

        // Sorted: lower number first
        let priorities = vec![
            ("high-priority".to_string(), 1),
            ("low-priority".to_string(), 10),
        ];
        let result = resolve("spec-template.md", dir.path(), &priorities);
        assert_eq!(
            result.source,
            TemplateSource::Preset("high-priority".into())
        );
    }

    #[test]
    fn empty_override_file_still_wins() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        // Empty file — still takes precedence (exists = wins)
        std::fs::write(
            dir.path()
                .join(".solidspec/templates/overrides/spec-template.md"),
            "",
        )
        .unwrap();

        let result = resolve("spec-template.md", dir.path(), &[]);
        assert_eq!(result.source, TemplateSource::Override);
    }

    #[test]
    fn load_template_from_override() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());
        std::fs::write(
            dir.path()
                .join(".solidspec/templates/overrides/spec-template.md"),
            "CUSTOM SPEC",
        )
        .unwrap();

        let (content, source) = load_template("spec-template.md", dir.path(), &[]).unwrap();
        assert_eq!(content, "CUSTOM SPEC");
        assert_eq!(source, TemplateSource::Override);
    }

    #[test]
    fn load_template_embedded_default() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());

        let (content, source) = load_template("spec-template.md", dir.path(), &[]).unwrap();
        assert!(!content.is_empty());
        assert_eq!(source, TemplateSource::EmbeddedDefault);
        assert!(content.contains("Feature Specification"));
    }

    #[test]
    fn load_unknown_template_returns_error() {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path());

        let result = load_template("nonexistent.md", dir.path(), &[]);
        assert!(result.is_err());
    }
}
