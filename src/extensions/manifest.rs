use std::sync::LazyLock;

use anyhow::{Result, bail};
use regex::Regex;
use serde::{Deserialize, Serialize};

static EXT_ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(ID_PATTERN).expect("invalid extension id regex"));
static EXT_CMD_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(CMD_PATTERN).expect("invalid extension cmd regex"));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub schema_version: String,
    pub extension: ExtensionInfo,
    #[serde(default)]
    pub requires: ExtensionRequires,
    #[serde(default)]
    pub provides: ExtensionProvides,
    #[serde(default)]
    pub hooks: std::collections::HashMap<String, HookDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtensionRequires {
    #[serde(default)]
    pub solidspec_version: String,
    #[serde(default)]
    pub tools: Vec<ToolRequirement>,
    #[serde(default)]
    pub extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequirement {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default = "default_true")]
    pub required: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtensionProvides {
    #[serde(default)]
    pub commands: Vec<ExtensionCommand>,
    #[serde(default)]
    pub config: Vec<ExtensionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionCommand {
    pub name: String,
    pub file: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionConfig {
    pub name: String,
    #[serde(default)]
    pub template: String,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookDef {
    pub command: String,
    #[serde(default)]
    pub optional: bool,
}

const VALID_HOOKS: &[&str] = &[
    "after_init",
    "after_add",
    "after_remove",
    "after_tasks",
    "before_implement",
    "after_implement",
];
const ID_PATTERN: &str = r"^[a-z0-9-]+$";
const CMD_PATTERN: &str = r"^solidspec\.[a-z0-9-]+\.[a-z0-9-]+$";

impl ExtensionManifest {
    pub fn load(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content)
    }

    pub fn parse(content: &str) -> Result<Self> {
        let manifest: Self = serde_yaml::from_str(content)?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != "1.0" {
            bail!(
                "Unsupported schema_version '{}'. Expected '1.0'.",
                self.schema_version
            );
        }

        if !EXT_ID_RE.is_match(&self.extension.id) {
            bail!(
                "Invalid extension ID '{}'. Must match {ID_PATTERN}.",
                self.extension.id
            );
        }

        if semver::Version::parse(&self.extension.version).is_err() {
            bail!(
                "Invalid version '{}'. Must be valid semver.",
                self.extension.version
            );
        }

        if self.extension.description.len() > 200 {
            bail!(
                "Description too long ({} chars). Max 200.",
                self.extension.description.len()
            );
        }

        if self.provides.commands.is_empty() {
            bail!("Extension must provide at least one command.");
        }

        for cmd in &self.provides.commands {
            if !EXT_CMD_RE.is_match(&cmd.name) {
                bail!(
                    "Invalid command name '{}'. Must match {CMD_PATTERN} (e.g., solidspec.my-ext.validate).",
                    cmd.name
                );
            }
        }

        let declared_commands: Vec<&str> = self
            .provides
            .commands
            .iter()
            .map(|c| c.name.as_str())
            .collect();

        for (hook_name, hook_def) in &self.hooks {
            if !VALID_HOOKS.contains(&hook_name.as_str()) {
                bail!(
                    "Invalid hook trigger '{}'. Must be one of: {}",
                    hook_name,
                    VALID_HOOKS.join(", ")
                );
            }
            if !declared_commands.contains(&hook_def.command.as_str()) {
                bail!(
                    "Hook '{}' references undeclared command '{}'. It must match a command in provides.commands.",
                    hook_name,
                    hook_def.command
                );
            }
        }

        if !self.requires.solidspec_version.is_empty()
            && semver::VersionReq::parse(&self.requires.solidspec_version).is_err()
        {
            bail!(
                "Invalid version specifier '{}'.",
                self.requires.solidspec_version
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_MANIFEST: &str = r#"
schema_version: "1.0"
extension:
  id: my-ext
  name: My Extension
  version: "1.0.0"
  description: "Test extension"
requires:
  solidspec_version: ">=0.1.0"
provides:
  commands:
    - name: solidspec.my-ext.validate
      file: commands/validate.md
      description: "Validate something"
hooks:
  after_tasks:
    command: solidspec.my-ext.validate
    optional: true
"#;

    #[test]
    fn parse_valid_manifest() {
        let m = ExtensionManifest::parse(VALID_MANIFEST).unwrap();
        assert_eq!(m.extension.id, "my-ext");
        assert_eq!(m.provides.commands.len(), 1);
        assert!(m.hooks.contains_key("after_tasks"));
    }

    #[test]
    fn missing_required_fields_errors() {
        let yaml = "schema_version: '1.0'\nextension:\n  id: x\n  name: X\n  version: '1.0.0'\n";
        let err = ExtensionManifest::parse(yaml).unwrap_err().to_string();
        assert!(err.contains("at least one command"));
    }

    #[test]
    fn invalid_version_specifier_errors() {
        let yaml = r#"
schema_version: "1.0"
extension:
  id: x
  name: X
  version: "1.0.0"
requires:
  solidspec_version: "bad"
provides:
  commands:
    - name: solidspec.x.y
      file: x.md
"#;
        assert!(ExtensionManifest::parse(yaml).is_err());
    }

    #[test]
    fn invalid_hook_trigger_errors() {
        let yaml = r#"
schema_version: "1.0"
extension:
  id: x
  name: X
  version: "1.0.0"
provides:
  commands:
    - name: solidspec.x.y
      file: x.md
hooks:
  invalid_hook:
    command: solidspec.x.y
"#;
        let err = ExtensionManifest::parse(yaml).unwrap_err().to_string();
        assert!(err.contains("Invalid hook trigger"));
    }

    #[test]
    fn valid_hooks_accepted() {
        for hook in &[
            "after_init",
            "after_tasks",
            "before_implement",
            "after_implement",
        ] {
            let yaml = format!(
                r#"
schema_version: "1.0"
extension:
  id: x
  name: X
  version: "1.0.0"
provides:
  commands:
    - name: solidspec.x.y
      file: x.md
hooks:
  {hook}:
    command: solidspec.x.y
"#
            );
            assert!(
                ExtensionManifest::parse(&yaml).is_ok(),
                "Hook {hook} should be valid"
            );
        }
    }

    #[test]
    fn invalid_extension_id_errors() {
        let yaml = "schema_version: '1.0'\nextension:\n  id: MyExt\n  name: X\n  version: '1.0.0'\nprovides:\n  commands:\n    - name: solidspec.x.y\n      file: x.md\n";
        let err = ExtensionManifest::parse(yaml).unwrap_err().to_string();
        assert!(err.contains("Invalid extension ID"));
    }

    #[test]
    fn invalid_command_name_errors() {
        let yaml = "schema_version: '1.0'\nextension:\n  id: x\n  name: X\n  version: '1.0.0'\nprovides:\n  commands:\n    - name: bad-name\n      file: x.md\n";
        let err = ExtensionManifest::parse(yaml).unwrap_err().to_string();
        assert!(err.contains("Invalid command name"));
    }

    #[test]
    fn description_over_200_chars_errors() {
        let long = "x".repeat(201);
        let yaml = format!(
            "schema_version: '1.0'\nextension:\n  id: x\n  name: X\n  version: '1.0.0'\n  description: '{long}'\nprovides:\n  commands:\n    - name: solidspec.x.y\n      file: x.md\n"
        );
        assert!(ExtensionManifest::parse(&yaml).is_err());
    }

    #[test]
    fn empty_commands_list_errors() {
        let yaml = "schema_version: '1.0'\nextension:\n  id: x\n  name: X\n  version: '1.0.0'\nprovides:\n  commands: []\n";
        let err = ExtensionManifest::parse(yaml).unwrap_err().to_string();
        assert!(err.contains("at least one command"));
    }

    #[test]
    fn hook_referencing_undeclared_command_errors() {
        let yaml = r#"
schema_version: "1.0"
extension:
  id: x
  name: X
  version: "1.0.0"
provides:
  commands:
    - name: solidspec.x.real
      file: commands/real.md
hooks:
  after_tasks:
    command: solidspec.x.nonexistent
"#;
        let err = ExtensionManifest::parse(yaml).unwrap_err().to_string();
        assert!(
            err.contains("undeclared command"),
            "Expected undeclared command error, got: {err}"
        );
    }

    #[test]
    fn hook_referencing_declared_command_passes() {
        let yaml = r#"
schema_version: "1.0"
extension:
  id: x
  name: X
  version: "1.0.0"
provides:
  commands:
    - name: solidspec.x.validate
      file: commands/validate.md
hooks:
  after_tasks:
    command: solidspec.x.validate
"#;
        assert!(ExtensionManifest::parse(yaml).is_ok());
    }
}
