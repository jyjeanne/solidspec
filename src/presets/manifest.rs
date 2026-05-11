use std::sync::LazyLock;

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

static ID_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(ID_REGEX).expect("invalid preset id regex"));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetManifest {
    pub schema_version: String,
    pub preset: PresetInfo,
    #[serde(default)]
    pub requires: PresetRequires,
    #[serde(default)]
    pub provides: PresetProvides,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetInfo {
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
pub struct PresetRequires {
    #[serde(default)]
    pub solidspec_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PresetProvides {
    #[serde(default)]
    pub templates: Vec<PresetTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetTemplate {
    #[serde(rename = "type")]
    pub template_type: String, // "template", "command", "script"
    pub name: String,
    pub file: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub replaces: Option<String>,
}

const VALID_TEMPLATE_TYPES: &[&str] = &["template", "command", "script"];
const ID_REGEX: &str = r"^[a-z0-9-]+$";

impl PresetManifest {
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
        // Schema version
        if self.schema_version != "1.0" {
            bail!(
                "Unsupported schema_version '{}'. Expected '1.0'.",
                self.schema_version
            );
        }

        // ID format
        if !ID_RE.is_match(&self.preset.id) {
            bail!(
                "Invalid preset ID '{}'. Must match {} (lowercase alphanumeric + hyphens).",
                self.preset.id,
                ID_REGEX
            );
        }

        // Version (semver)
        if semver::Version::parse(&self.preset.version).is_err() {
            bail!(
                "Invalid version '{}'. Must be valid semver (e.g., 1.0.0).",
                self.preset.version
            );
        }

        // Description length
        if self.preset.description.len() > 200 {
            bail!(
                "Description too long ({} chars). Max 200.",
                self.preset.description.len()
            );
        }

        // Template types
        for tmpl in &self.provides.templates {
            if !VALID_TEMPLATE_TYPES.contains(&tmpl.template_type.as_str()) {
                bail!(
                    "Invalid template type '{}'. Must be one of: {}",
                    tmpl.template_type,
                    VALID_TEMPLATE_TYPES.join(", ")
                );
            }
        }

        // Version specifier
        if !self.requires.solidspec_version.is_empty()
            && semver::VersionReq::parse(&self.requires.solidspec_version).is_err()
        {
            bail!(
                "Invalid version specifier '{}'. Use semver ranges like >=0.1.0",
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
preset:
  id: my-preset
  name: My Preset
  version: "1.0.0"
  description: "A test preset"
  author: Test
requires:
  solidspec_version: ">=0.1.0"
provides:
  templates:
    - type: template
      name: spec-template
      file: templates/spec-template.md
      replaces: spec-template
    - type: command
      name: solidspec.specify
      file: commands/specify.md
"#;

    #[test]
    fn parse_valid_manifest() {
        let manifest = PresetManifest::parse(VALID_MANIFEST).unwrap();
        assert_eq!(manifest.preset.id, "my-preset");
        assert_eq!(manifest.preset.name, "My Preset");
        assert_eq!(manifest.preset.version, "1.0.0");
        assert_eq!(manifest.provides.templates.len(), 2);
        assert_eq!(manifest.provides.templates[0].template_type, "template");
        assert_eq!(manifest.provides.templates[1].template_type, "command");
    }

    #[test]
    fn missing_schema_version_errors() {
        let yaml = "preset:\n  id: x\n  name: X\n  version: '1.0.0'\n";
        assert!(PresetManifest::parse(yaml).is_err());
    }

    #[test]
    fn wrong_schema_version_errors() {
        let yaml = "schema_version: '2.0'\npreset:\n  id: x\n  name: X\n  version: '1.0.0'\n";
        let err = PresetManifest::parse(yaml).unwrap_err().to_string();
        assert!(err.contains("schema_version"));
    }

    #[test]
    fn invalid_semver_errors() {
        let yaml = "schema_version: '1.0'\npreset:\n  id: x\n  name: X\n  version: 'notvalid'\n";
        let err = PresetManifest::parse(yaml).unwrap_err().to_string();
        assert!(err.contains("semver"));
    }

    #[test]
    fn invalid_id_with_uppercase_errors() {
        let yaml =
            "schema_version: '1.0'\npreset:\n  id: MyPreset\n  name: X\n  version: '1.0.0'\n";
        let err = PresetManifest::parse(yaml).unwrap_err().to_string();
        assert!(err.contains("Invalid preset ID"));
    }

    #[test]
    fn unknown_template_type_errors() {
        let yaml = r#"
schema_version: "1.0"
preset:
  id: x
  name: X
  version: "1.0.0"
provides:
  templates:
    - type: invalid
      name: x
      file: x.md
"#;
        let err = PresetManifest::parse(yaml).unwrap_err().to_string();
        assert!(err.contains("Invalid template type"));
    }

    #[test]
    fn invalid_version_specifier_errors() {
        let yaml = "schema_version: '1.0'\npreset:\n  id: x\n  name: X\n  version: '1.0.0'\nrequires:\n  solidspec_version: 'not-a-range'\n";
        let err = PresetManifest::parse(yaml).unwrap_err().to_string();
        assert!(err.contains("version specifier"));
    }

    #[test]
    fn description_over_200_chars_errors() {
        let long_desc = "x".repeat(201);
        let yaml = format!(
            "schema_version: '1.0'\npreset:\n  id: x\n  name: X\n  version: '1.0.0'\n  description: '{}'\n",
            long_desc
        );
        let err = PresetManifest::parse(&yaml).unwrap_err().to_string();
        assert!(err.contains("too long"));
    }
}
