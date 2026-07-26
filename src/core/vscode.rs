//! Deep-merge helper for `.vscode/settings.json`. Fully built and tested but
//! not yet called from `cli/init.rs` or anywhere else — no current command
//! writes VS Code settings during project setup.
#![allow(dead_code)]

use std::path::Path;

use anyhow::Result;
use serde_json::Value;

/// Deep-merge VSCode settings: preserves existing keys, adds new ones.
/// Arrays are replaced (not merged). Objects are deep-merged.
pub fn merge_settings(project_dir: &Path, new_settings: &Value) -> Result<()> {
    let vscode_dir = project_dir.join(".vscode");
    let settings_path = vscode_dir.join("settings.json");

    if settings_path.exists() {
        let existing_str = std::fs::read_to_string(&settings_path)?;
        let existing: Value = serde_json::from_str(&existing_str)
            .map_err(|e| anyhow::anyhow!("Invalid JSON in .vscode/settings.json: {e}"))?;

        let merged = deep_merge(existing, new_settings.clone());
        let output = serde_json::to_string_pretty(&merged)?;
        std::fs::write(&settings_path, output)?;
    } else {
        std::fs::create_dir_all(&vscode_dir)?;
        let output = serde_json::to_string_pretty(new_settings)?;
        std::fs::write(&settings_path, output)?;
    }

    Ok(())
}

fn deep_merge(base: Value, overlay: Value) -> Value {
    match (base, overlay) {
        (Value::Object(mut base_map), Value::Object(overlay_map)) => {
            for (key, overlay_val) in overlay_map {
                let merged = if let Some(base_val) = base_map.remove(&key) {
                    deep_merge(base_val, overlay_val)
                } else {
                    overlay_val
                };
                base_map.insert(key, merged);
            }
            Value::Object(base_map)
        }
        // Non-object values: overlay wins (arrays replaced, not merged)
        (_, overlay) => overlay,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    #[test]
    fn merge_into_empty_creates_file() {
        let dir = TempDir::new().unwrap();
        let settings = json!({"editor.fontSize": 14});
        merge_settings(dir.path(), &settings).unwrap();

        let path = dir.path().join(".vscode/settings.json");
        assert!(path.exists());
        let content: Value =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(content["editor.fontSize"], 14);
    }

    #[test]
    fn merge_preserves_existing_keys() {
        let dir = TempDir::new().unwrap();
        let vscode = dir.path().join(".vscode");
        std::fs::create_dir_all(&vscode).unwrap();
        std::fs::write(vscode.join("settings.json"), r#"{"existing": true}"#).unwrap();

        let new = json!({"added": "value"});
        merge_settings(dir.path(), &new).unwrap();

        let content: Value =
            serde_json::from_str(&std::fs::read_to_string(vscode.join("settings.json")).unwrap())
                .unwrap();
        assert_eq!(content["existing"], true);
        assert_eq!(content["added"], "value");
    }

    #[test]
    fn deep_merge_nested_objects() {
        let dir = TempDir::new().unwrap();
        let vscode = dir.path().join(".vscode");
        std::fs::create_dir_all(&vscode).unwrap();
        std::fs::write(
            vscode.join("settings.json"),
            r#"{"editor": {"fontSize": 14, "tabSize": 2}}"#,
        )
        .unwrap();

        let new = json!({"editor": {"wordWrap": "on"}});
        merge_settings(dir.path(), &new).unwrap();

        let content: Value =
            serde_json::from_str(&std::fs::read_to_string(vscode.join("settings.json")).unwrap())
                .unwrap();
        assert_eq!(content["editor"]["fontSize"], 14); // preserved
        assert_eq!(content["editor"]["tabSize"], 2); // preserved
        assert_eq!(content["editor"]["wordWrap"], "on"); // added
    }

    #[test]
    fn arrays_replaced_not_merged() {
        let dir = TempDir::new().unwrap();
        let vscode = dir.path().join(".vscode");
        std::fs::create_dir_all(&vscode).unwrap();
        std::fs::write(
            vscode.join("settings.json"),
            r#"{"files.exclude": ["*.log"]}"#,
        )
        .unwrap();

        let new = json!({"files.exclude": ["*.tmp", "*.bak"]});
        merge_settings(dir.path(), &new).unwrap();

        let content: Value =
            serde_json::from_str(&std::fs::read_to_string(vscode.join("settings.json")).unwrap())
                .unwrap();
        let arr = content["files.exclude"].as_array().unwrap();
        assert_eq!(arr.len(), 2); // replaced, not 3
    }

    #[test]
    fn non_json_existing_file_errors() {
        let dir = TempDir::new().unwrap();
        let vscode = dir.path().join(".vscode");
        std::fs::create_dir_all(&vscode).unwrap();
        std::fs::write(vscode.join("settings.json"), "NOT JSON {{{").unwrap();

        let result = merge_settings(dir.path(), &json!({}));
        assert!(result.is_err());
    }
}
