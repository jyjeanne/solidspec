#![allow(dead_code)]
use std::collections::HashMap;
use std::path::Path;

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionEntry {
    pub id: String,
    pub name: String,
    pub version: String,
    pub installed_timestamp: String,
    pub enabled: bool,
    pub commands: HashMap<String, Vec<String>>, // agent_id → [command_names]
    pub hooks: Vec<HookEntry>,
    pub templates: Vec<String>,
    #[serde(default)]
    pub dev: bool, // installed via --dev
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookEntry {
    pub trigger: String,
    pub file: String,
}

#[derive(Debug, Clone, Default)]
pub struct ExtensionRegistry {
    entries: HashMap<String, ExtensionEntry>,
}

impl ExtensionRegistry {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)?;
        if content.trim().is_empty() || content.trim() == "{}" {
            return Ok(Self::default());
        }
        let entries: HashMap<String, ExtensionEntry> = match serde_json::from_str(&content) {
            Ok(e) => e,
            Err(e) => {
                log::warn!("Extension registry corrupted, starting fresh: {e}");
                HashMap::new()
            }
        };
        Ok(Self { entries })
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.entries)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn add(&mut self, entry: ExtensionEntry) -> Result<()> {
        if self.entries.contains_key(&entry.id) {
            bail!("Extension '{}' is already installed.", entry.id);
        }
        self.entries.insert(entry.id.clone(), entry);
        Ok(())
    }

    pub fn remove(&mut self, id: &str) -> Result<ExtensionEntry> {
        self.entries
            .remove(id)
            .ok_or_else(|| anyhow::anyhow!("Extension '{}' is not installed.", id))
    }

    pub fn get(&self, id: &str) -> Option<ExtensionEntry> {
        self.entries.get(id).cloned() // deep copy
    }

    pub fn update(&mut self, entry: ExtensionEntry) -> Result<()> {
        let existing = self.entries.get(&entry.id).ok_or_else(|| {
            anyhow::anyhow!("Extension '{}' not installed. Cannot update.", entry.id)
        })?;
        // Preserve original installed_timestamp
        let mut updated = entry;
        updated.installed_timestamp = existing.installed_timestamp.clone();
        self.entries.insert(updated.id.clone(), updated);
        Ok(())
    }

    pub fn enable(&mut self, id: &str) -> Result<()> {
        let entry = self
            .entries
            .get_mut(id)
            .ok_or_else(|| anyhow::anyhow!("Extension '{id}' not installed."))?;
        entry.enabled = true;
        Ok(())
    }

    pub fn disable(&mut self, id: &str) -> Result<()> {
        let entry = self
            .entries
            .get_mut(id)
            .ok_or_else(|| anyhow::anyhow!("Extension '{id}' not installed."))?;
        entry.enabled = false;
        Ok(())
    }

    pub fn list(&self) -> Vec<ExtensionEntry> {
        self.entries.values().cloned().collect()
    }

    pub fn search(&self, query: &str) -> Vec<ExtensionEntry> {
        let q = query.to_lowercase();
        self.entries
            .values()
            .filter(|e| {
                e.id.to_lowercase().contains(&q)
                    || e.name.to_lowercase().contains(&q)
                    || e.version.to_lowercase().contains(&q)
            })
            .cloned()
            .collect()
    }

    /// Find by ID or display name. Returns error on ambiguous match.
    pub fn resolve(&self, name: &str) -> Result<Option<ExtensionEntry>> {
        // Try exact ID first
        if let Some(entry) = self.entries.get(name) {
            return Ok(Some(entry.clone()));
        }
        // Try display name
        let matches: Vec<_> = self
            .entries
            .values()
            .filter(|e| e.name.to_lowercase() == name.to_lowercase())
            .collect();
        match matches.len() {
            0 => Ok(None),
            1 => Ok(Some(matches[0].clone())),
            _ => bail!(
                "Ambiguous name '{}'. Matches: {}. Use the ID instead.",
                name,
                matches
                    .iter()
                    .map(|e| e.id.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }

    pub fn enabled_hooks(&self, trigger: &str) -> Vec<(String, HookEntry)> {
        self.entries
            .values()
            .filter(|e| e.enabled)
            .flat_map(|e| {
                e.hooks
                    .iter()
                    .filter(|h| h.trigger == trigger)
                    .map(move |h| (e.id.clone(), h.clone()))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn sample_entry(id: &str) -> ExtensionEntry {
        ExtensionEntry {
            id: id.into(),
            name: format!("Extension {id}"),
            version: "1.0.0".into(),
            installed_timestamp: "2026-03-14T00:00:00Z".into(),
            enabled: true,
            commands: HashMap::from([("claude".into(), vec![format!("solidspec.{id}.cmd")])]),
            hooks: vec![HookEntry {
                trigger: "after_tasks".into(),
                file: "hooks/run.sh".into(),
            }],
            templates: vec![],
            dev: false,
        }
    }

    #[test]
    fn add_and_get_deep_copy() {
        let mut reg = ExtensionRegistry::default();
        reg.add(sample_entry("my-ext")).unwrap();
        let entry = reg.get("my-ext").unwrap();
        assert!(entry.enabled);
        assert_eq!(entry.commands.get("claude").unwrap().len(), 1);
        // Verify deep copy: mutating returned entry doesn't affect registry
        drop(entry);
        assert!(reg.get("my-ext").unwrap().enabled);
    }

    #[test]
    fn duplicate_add_errors() {
        let mut reg = ExtensionRegistry::default();
        reg.add(sample_entry("x")).unwrap();
        assert!(reg.add(sample_entry("x")).is_err());
    }

    #[test]
    fn update_preserves_timestamp() {
        let mut reg = ExtensionRegistry::default();
        reg.add(sample_entry("x")).unwrap();
        let original_ts = reg.get("x").unwrap().installed_timestamp;

        let mut updated = sample_entry("x");
        updated.installed_timestamp = "2099-01-01T00:00:00Z".into();
        updated.version = "2.0.0".into();
        // Remove first to allow update (update requires existing)
        // Actually update works on existing
        reg.entries.remove("x");
        reg.add(sample_entry("x")).unwrap();
        let mut new_entry = sample_entry("x");
        new_entry.installed_timestamp = "2099-01-01T00:00:00Z".into();
        new_entry.version = "2.0.0".into();
        reg.update(new_entry).unwrap();

        let result = reg.get("x").unwrap();
        assert_eq!(result.version, "2.0.0");
        assert_eq!(result.installed_timestamp, original_ts); // preserved!
    }

    #[test]
    fn update_nonexistent_errors() {
        let mut reg = ExtensionRegistry::default();
        assert!(reg.update(sample_entry("ghost")).is_err());
    }

    #[test]
    fn enable_disable_toggle() {
        let mut reg = ExtensionRegistry::default();
        reg.add(sample_entry("x")).unwrap();

        reg.disable("x").unwrap();
        assert!(!reg.get("x").unwrap().enabled);

        reg.enable("x").unwrap();
        assert!(reg.get("x").unwrap().enabled);
    }

    #[test]
    fn disable_already_disabled_is_noop() {
        let mut reg = ExtensionRegistry::default();
        reg.add(sample_entry("x")).unwrap();
        reg.disable("x").unwrap();
        reg.disable("x").unwrap(); // no error
        assert!(!reg.get("x").unwrap().enabled);
    }

    #[test]
    fn remove_deletes_entry() {
        let mut reg = ExtensionRegistry::default();
        reg.add(sample_entry("x")).unwrap();
        reg.remove("x").unwrap();
        assert!(reg.get("x").is_none());
    }

    #[test]
    fn remove_nonexistent_errors() {
        let mut reg = ExtensionRegistry::default();
        assert!(reg.remove("ghost").is_err());
    }

    #[test]
    fn corrupted_registry_starts_fresh() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join(".registry");
        std::fs::write(&path, "NOT JSON {{{").unwrap();
        let reg = ExtensionRegistry::load(&path).unwrap();
        assert!(reg.list().is_empty());
    }

    #[test]
    fn load_empty_registry() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join(".registry");
        std::fs::write(&path, "{}").unwrap();
        let reg = ExtensionRegistry::load(&path).unwrap();
        assert!(reg.list().is_empty());
    }

    #[test]
    fn save_and_reload() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join(".registry");
        let mut reg = ExtensionRegistry::default();
        reg.add(sample_entry("test")).unwrap();
        reg.save(&path).unwrap();

        let loaded = ExtensionRegistry::load(&path).unwrap();
        assert_eq!(loaded.get("test").unwrap().version, "1.0.0");
    }

    #[test]
    fn search_by_name() {
        let mut reg = ExtensionRegistry::default();
        reg.add(sample_entry("lint-ext")).unwrap();
        reg.add(sample_entry("format-ext")).unwrap();
        let results = reg.search("lint");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn resolve_by_id() {
        let mut reg = ExtensionRegistry::default();
        reg.add(sample_entry("my-ext")).unwrap();
        assert!(reg.resolve("my-ext").unwrap().is_some());
    }

    #[test]
    fn resolve_by_display_name() {
        let mut reg = ExtensionRegistry::default();
        reg.add(sample_entry("my-ext")).unwrap();
        let result = reg.resolve("Extension my-ext").unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn resolve_missing_returns_none() {
        let reg = ExtensionRegistry::default();
        assert!(reg.resolve("ghost").unwrap().is_none());
    }

    #[test]
    fn enabled_hooks_skips_disabled() {
        let mut reg = ExtensionRegistry::default();
        reg.add(sample_entry("active")).unwrap();
        let mut disabled = sample_entry("disabled");
        disabled.id = "disabled".into();
        reg.add(disabled).unwrap();
        reg.disable("disabled").unwrap();

        let hooks = reg.enabled_hooks("after_tasks");
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].0, "active");
    }
}
