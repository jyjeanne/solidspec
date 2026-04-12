use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::core::errors::RustySpecError;

/// Root configuration from `rustyspec.toml`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootConfig {
    pub project: ProjectConfig,
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub git: GitConfig,
    #[serde(default)]
    pub templates: TemplatesConfig,
    #[serde(default)]
    pub pipeline: PipelineConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiConfig {
    #[serde(default = "default_agent")]
    pub default_agent: String,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            default_agent: default_agent(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitConfig {
    #[serde(default = "default_true")]
    pub auto_branch: bool,
    #[serde(default = "default_true")]
    pub auto_commit: bool,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            auto_branch: true,
            auto_commit: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TemplatesConfig {
    #[serde(default = "default_override_dir")]
    pub override_dir: String,
}

/// Pipeline configuration: maps SDD phases to agent IDs.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PipelineConfig {
    #[serde(default)]
    pub specify: Option<String>,
    #[serde(default)]
    pub clarify: Option<String>,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub tasks: Option<String>,
    #[serde(default)]
    pub tests: Option<String>,
    #[serde(default)]
    pub implement: Option<String>,
    #[serde(default)]
    pub analyze: Option<String>,
    #[serde(default)]
    pub review: Option<String>,
}

impl PipelineConfig {
    /// Get the agent for a phase, falling back to default_agent.
    pub fn agent_for_phase(&self, phase: &str, default_agent: &str) -> String {
        let mapped = match phase {
            "specify" => &self.specify,
            "clarify" => &self.clarify,
            "plan" => &self.plan,
            "tasks" => &self.tasks,
            "tests" => &self.tests,
            "implement" => &self.implement,
            "analyze" => &self.analyze,
            "review" => &self.review,
            _ => &None,
        };
        mapped.as_deref().unwrap_or(default_agent).to_string()
    }

    /// Validate all mapped agent IDs exist.
    pub fn validate(&self, valid_agents: &[&str]) -> Result<()> {
        let mappings = [
            ("specify", &self.specify),
            ("clarify", &self.clarify),
            ("plan", &self.plan),
            ("tasks", &self.tasks),
            ("tests", &self.tests),
            ("implement", &self.implement),
            ("analyze", &self.analyze),
            ("review", &self.review),
        ];
        for (phase, agent) in &mappings {
            if let Some(id) = agent
                && !valid_agents.contains(&id.as_str())
            {
                anyhow::bail!(
                    "Pipeline phase '{}' maps to unknown agent '{}'. Available: {}",
                    phase,
                    id,
                    valid_agents.join(", ")
                );
            }
        }
        Ok(())
    }
}

fn default_version() -> String {
    "0.1.0".into()
}
fn default_agent() -> String {
    "claude".into()
}
fn default_true() -> bool {
    true
}
fn default_override_dir() -> String {
    ".rustyspec/templates/overrides".into()
}

impl RootConfig {
    pub fn new(project_name: &str) -> Self {
        Self {
            project: ProjectConfig {
                name: project_name.to_string(),
                version: default_version(),
            },
            ai: AiConfig::default(),
            git: GitConfig::default(),
            templates: TemplatesConfig::default(),
            pipeline: PipelineConfig::default(),
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| RustySpecError::Config {
            path: path.to_path_buf(),
            message: format!("Cannot read file: {e}"),
            fix: "Ensure rustyspec.toml exists. Run 'rustyspec init' to create it.".into(),
        })?;

        toml::from_str(&content)
            .map_err(|e| RustySpecError::Config {
                path: path.to_path_buf(),
                message: format!("Invalid TOML: {e}"),
                fix: "Check rustyspec.toml syntax. See docs for format.".into(),
            })
            .map_err(Into::into)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Project-internal config from `.rustyspec/config.toml`
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProjectInternalConfig {
    #[serde(default)]
    pub presets: CatalogList,
    #[serde(default)]
    pub extensions: CatalogList,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CatalogList {
    #[serde(default)]
    pub catalogs: Vec<String>,
}

impl ProjectInternalConfig {
    pub fn save(&self, path: &Path) -> Result<()> {
        let content =
            toml::to_string_pretty(self).context("Failed to serialize internal config")?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Init options persisted to `.rustyspec/init-options.json`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitOptions {
    pub ai_assistant: String,
    pub script_type: String,
    pub installed_at: String,
}

impl InitOptions {
    pub fn save(&self, path: &Path) -> Result<()> {
        let content =
            serde_json::to_string_pretty(self).context("Failed to serialize init options")?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Find the project root by looking for `rustyspec.toml` or `.rustyspec/`
pub fn find_project_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join("rustyspec.toml").exists() || current.join(".rustyspec").exists() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn new_config_has_correct_defaults() {
        let cfg = RootConfig::new("test-project");
        assert_eq!(cfg.project.name, "test-project");
        assert_eq!(cfg.project.version, "0.1.0");
        assert_eq!(cfg.ai.default_agent, "claude");
        assert!(cfg.git.auto_branch);
        assert!(cfg.git.auto_commit);
    }

    #[test]
    fn round_trip_serialize_deserialize() {
        let cfg = RootConfig::new("myapp");
        let serialized = toml::to_string_pretty(&cfg).unwrap();
        let deserialized: RootConfig = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized.project.name, "myapp");
        assert_eq!(deserialized.ai.default_agent, "claude");
    }

    #[test]
    fn load_valid_config() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("rustyspec.toml");
        let content = r#"
[project]
name = "hello"
version = "1.0.0"
"#;
        std::fs::write(&path, content).unwrap();
        let cfg = RootConfig::load(&path).unwrap();
        assert_eq!(cfg.project.name, "hello");
        assert_eq!(cfg.project.version, "1.0.0");
    }

    #[test]
    fn load_malformed_toml_returns_error() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("rustyspec.toml");
        std::fs::write(&path, "not valid toml {{{}").unwrap();
        assert!(RootConfig::load(&path).is_err());
    }

    #[test]
    fn load_missing_file_returns_error() {
        let path = Path::new("/nonexistent/rustyspec.toml");
        assert!(RootConfig::load(path).is_err());
    }

    #[test]
    fn save_and_reload() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("rustyspec.toml");
        let cfg = RootConfig::new("roundtrip");
        cfg.save(&path).unwrap();
        let loaded = RootConfig::load(&path).unwrap();
        assert_eq!(loaded.project.name, "roundtrip");
    }

    #[test]
    fn defaults_when_optional_fields_missing() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("rustyspec.toml");
        std::fs::write(&path, "[project]\nname = \"minimal\"\n").unwrap();
        let cfg = RootConfig::load(&path).unwrap();
        assert_eq!(cfg.ai.default_agent, "claude");
        assert!(cfg.git.auto_branch);
    }

    #[test]
    fn find_project_root_finds_rustyspec_toml() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("rustyspec.toml"), "").unwrap();
        let sub = dir.path().join("specs/001");
        std::fs::create_dir_all(&sub).unwrap();
        assert_eq!(find_project_root(&sub).unwrap(), dir.path());
    }

    #[test]
    fn find_project_root_returns_none_at_root() {
        assert!(find_project_root(Path::new("/tmp/nonexistent-rustyspec-test")).is_none());
    }
}
