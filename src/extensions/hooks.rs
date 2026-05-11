use std::path::Path;
use std::process::Command;

use super::registry::ExtensionRegistry;

/// Execute all enabled hooks for a given trigger.
pub fn fire_hooks(trigger: &str, project_root: &Path, registry: &ExtensionRegistry) {
    let hooks = registry.enabled_hooks(trigger);
    for (ext_id, hook) in &hooks {
        let hook_path = project_root
            .join(".solidspec/extensions")
            .join(ext_id)
            .join(&hook.file);

        if !hook_path.exists() {
            log::warn!(
                "Hook file not found for extension '{ext_id}': {}",
                hook_path.display()
            );
            continue;
        }

        log::debug!("Firing hook '{}' for extension '{}'", trigger, ext_id);

        let result = if cfg!(windows) {
            let ext = hook_path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if ext == "ps1" {
                Command::new("powershell")
                    .args(["-ExecutionPolicy", "Bypass", "-File"])
                    .arg(&hook_path)
                    .env("EXTENSION_ID", ext_id)
                    .env("PROJECT_ROOT", project_root)
                    .env("HOOK_TRIGGER", trigger)
                    .output()
            } else {
                // Try sh (Git Bash / MSYS2), fall back to cmd
                Command::new("sh")
                    .arg(&hook_path)
                    .env("EXTENSION_ID", ext_id)
                    .env("PROJECT_ROOT", project_root)
                    .env("HOOK_TRIGGER", trigger)
                    .output()
                    .or_else(|_| {
                        Command::new("cmd")
                            .args(["/C"])
                            .arg(&hook_path)
                            .env("EXTENSION_ID", ext_id)
                            .env("PROJECT_ROOT", project_root)
                            .env("HOOK_TRIGGER", trigger)
                            .output()
                    })
            }
        } else {
            Command::new("sh")
                .arg(&hook_path)
                .env("EXTENSION_ID", ext_id)
                .env("PROJECT_ROOT", project_root)
                .env("HOOK_TRIGGER", trigger)
                .output()
        };

        match result {
            Ok(output) => {
                if output.status.success() {
                    log::debug!("Hook '{}' for '{}' succeeded", trigger, ext_id);
                } else {
                    log::warn!(
                        "Hook '{}' for '{}' failed (exit {})",
                        trigger,
                        ext_id,
                        output.status
                    );
                }
            }
            Err(e) => {
                log::warn!("Hook '{}' for '{}' failed to execute: {e}", trigger, ext_id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extensions::registry::{ExtensionEntry, HookEntry};
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    fn fire_hooks_skips_missing_file() {
        let dir = TempDir::new().unwrap();
        let mut reg = ExtensionRegistry::default();
        let entry = ExtensionEntry {
            id: "test".into(),
            name: "Test".into(),
            version: "1.0.0".into(),
            installed_timestamp: "2026-01-01T00:00:00Z".into(),
            enabled: true,
            commands: HashMap::new(),
            hooks: vec![HookEntry {
                trigger: "after_tasks".into(),
                file: "hooks/nonexistent.sh".into(),
            }],
            templates: vec![],
            dev: false,
        };
        reg.add(entry).unwrap();

        // Should not panic, just log warning
        fire_hooks("after_tasks", dir.path(), &reg);
    }

    #[test]
    fn fire_hooks_skips_disabled_extensions() {
        let dir = TempDir::new().unwrap();
        let mut reg = ExtensionRegistry::default();
        let entry = ExtensionEntry {
            id: "test".into(),
            name: "Test".into(),
            version: "1.0.0".into(),
            installed_timestamp: "2026-01-01T00:00:00Z".into(),
            enabled: false,
            commands: HashMap::new(),
            hooks: vec![HookEntry {
                trigger: "after_tasks".into(),
                file: "hooks/run.sh".into(),
            }],
            templates: vec![],
            dev: false,
        };
        reg.add(entry).unwrap();

        // No hooks should fire
        let hooks = reg.enabled_hooks("after_tasks");
        assert!(hooks.is_empty());
        fire_hooks("after_tasks", dir.path(), &reg);
    }
}
