use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::core::errors::SolidSpecError;

/// Root configuration from `solidspec.toml`
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
    #[serde(default)]
    pub context: ContextConfig,
    #[serde(default)]
    pub fan_out: FanOutConfig,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplatesConfig {
    #[serde(default = "default_override_dir")]
    pub override_dir: String,
}

impl Default for TemplatesConfig {
    fn default() -> Self {
        Self {
            override_dir: default_override_dir(),
        }
    }
}

/// Project context injected into every agent prompt.
/// Provides tech stack, conventions, and per-phase rules.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ContextConfig {
    /// Project overview — tech stack, conventions, architecture notes
    #[serde(default)]
    pub description: String,
    /// Per-phase rules (spec, plan, tasks, implement, tests, analyze, review)
    #[serde(default)]
    pub rules: ContextRules,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ContextRules {
    #[serde(default)]
    pub spec: String,
    #[serde(default)]
    pub clarify: String,
    #[serde(default)]
    pub plan: String,
    #[serde(default)]
    pub tasks: String,
    #[serde(default)]
    pub implement: String,
    #[serde(default)]
    pub tests: String,
    #[serde(default)]
    pub analyze: String,
    #[serde(default)]
    pub review: String,
}

#[allow(dead_code)]
impl ContextConfig {
    /// Format context as a prompt section.
    #[allow(dead_code)]
    pub fn as_prompt_section(&self) -> String {
        if self.description.is_empty() {
            return String::new();
        }
        format!("## Project Context\n\n{}\n", self.description)
    }

    /// Get per-phase rules for a given phase name.
    pub fn rules_for_phase(&self, phase: &str) -> &str {
        match phase {
            "specify" => &self.rules.spec,
            "clarify" => &self.rules.clarify,
            "plan" => &self.rules.plan,
            "tasks" => &self.rules.tasks,
            "implement" => &self.rules.implement,
            "tests" => &self.rules.tests,
            "analyze" => &self.rules.analyze,
            "review" => &self.rules.review,
            _ => "",
        }
    }

    /// Format the full context + rules for a phase, for prompt injection.
    pub fn as_phase_prompt(&self, phase: &str) -> String {
        let mut out = self.as_prompt_section();
        let rules = self.rules_for_phase(phase);
        if !rules.is_empty() {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&format!("## Phase-Specific Rules ({phase})\n\n{rules}\n"));
        }
        out
    }
}

/// Pipeline configuration: maps SDD phases to agent IDs.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PipelineConfig {
    /// Workflow schema to use. Default: "spec-driven". Use "intent-driven" for IDSD mode.
    #[serde(default = "default_schema")]
    pub schema: String,
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

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            schema: default_schema(),
            specify: None,
            clarify: None,
            plan: None,
            tasks: None,
            tests: None,
            implement: None,
            analyze: None,
            review: None,
        }
    }
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
    ".solidspec/templates/overrides".into()
}
fn default_schema() -> String {
    "spec-driven".into()
}
fn default_code_threshold() -> u8 {
    70
}
fn default_security_threshold() -> u8 {
    80
}
fn default_tests_threshold() -> u8 {
    70
}
fn default_perf_threshold() -> u8 {
    60
}
fn default_fanout_timeout() -> u64 {
    300
}

/// Fan-out orchestration configuration for `solidspec ship`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FanOutConfig {
    /// Override the agent used for the code review lane (None → default_agent).
    #[serde(default)]
    pub code_agent: Option<String>,
    /// Override the agent used for the security audit lane (None → default_agent).
    #[serde(default)]
    pub security_agent: Option<String>,
    /// Override the agent used for the test coverage lane (None → default_agent).
    #[serde(default)]
    pub tests_agent: Option<String>,
    /// Override the agent used for the performance lane (None → default_agent).
    #[serde(default)]
    pub perf_agent: Option<String>,
    /// Score threshold for the code lane (0–100). Below this → HOLD.
    #[serde(default = "default_code_threshold")]
    pub code_threshold: u8,
    /// Score threshold for the security lane (0–100). Below this → HOLD.
    #[serde(default = "default_security_threshold")]
    pub security_threshold: u8,
    /// Score threshold for the test coverage lane (0–100). Below this → HOLD.
    #[serde(default = "default_tests_threshold")]
    pub tests_threshold: u8,
    /// Score threshold for the performance lane (0–100). Below this → HOLD.
    #[serde(default = "default_perf_threshold")]
    pub perf_threshold: u8,
    /// Per-lane timeout in seconds before the lane is killed and marked TimedOut.
    #[serde(default = "default_fanout_timeout")]
    pub timeout: u64,
    /// When true, any Critical finding in any lane blocks shipping regardless of score.
    #[serde(default = "default_true")]
    pub block_on_critical: bool,
}

impl Default for FanOutConfig {
    fn default() -> Self {
        Self {
            code_agent: None,
            security_agent: None,
            tests_agent: None,
            perf_agent: None,
            code_threshold: default_code_threshold(),
            security_threshold: default_security_threshold(),
            tests_threshold: default_tests_threshold(),
            perf_threshold: default_perf_threshold(),
            timeout: default_fanout_timeout(),
            block_on_critical: true,
        }
    }
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
            context: ContextConfig::default(),
            fan_out: FanOutConfig::default(),
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| SolidSpecError::Config {
            path: path.to_path_buf(),
            message: format!("Cannot read file: {e}"),
            fix: "Ensure solidspec.toml exists. Run 'solidspec init' to create it.".into(),
        })?;

        toml::from_str(&content)
            .map_err(|e| SolidSpecError::Config {
                path: path.to_path_buf(),
                message: format!("Invalid TOML: {e}"),
                fix: "Check solidspec.toml syntax. See docs for format.".into(),
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

/// Project-internal config from `.solidspec/config.toml`
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

/// Init options persisted to `.solidspec/init-options.json`
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

/// Find the project root by looking for `solidspec.toml` or `.solidspec/`
pub fn find_project_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join("solidspec.toml").exists() || current.join(".solidspec").exists() {
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
        let path = dir.path().join("solidspec.toml");
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
        let path = dir.path().join("solidspec.toml");
        std::fs::write(&path, "not valid toml {{{}").unwrap();
        assert!(RootConfig::load(&path).is_err());
    }

    #[test]
    fn load_missing_file_returns_error() {
        let path = Path::new("/nonexistent/solidspec.toml");
        assert!(RootConfig::load(path).is_err());
    }

    #[test]
    fn save_and_reload() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("solidspec.toml");
        let cfg = RootConfig::new("roundtrip");
        cfg.save(&path).unwrap();
        let loaded = RootConfig::load(&path).unwrap();
        assert_eq!(loaded.project.name, "roundtrip");
    }

    #[test]
    fn defaults_when_optional_fields_missing() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("solidspec.toml");
        std::fs::write(&path, "[project]\nname = \"minimal\"\n").unwrap();
        let cfg = RootConfig::load(&path).unwrap();
        assert_eq!(cfg.ai.default_agent, "claude");
        assert!(cfg.git.auto_branch);
    }

    #[test]
    fn find_project_root_finds_solidspec_toml() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("solidspec.toml"), "").unwrap();
        let sub = dir.path().join("specs/001");
        std::fs::create_dir_all(&sub).unwrap();
        assert_eq!(find_project_root(&sub).unwrap(), dir.path());
    }

    #[test]
    fn find_project_root_returns_none_at_root() {
        assert!(find_project_root(Path::new("/tmp/nonexistent-solidspec-test")).is_none());
    }

    #[test]
    fn fanout_config_defaults_when_section_absent() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("solidspec.toml");
        std::fs::write(&path, "[project]\nname = \"minimal\"\n").unwrap();
        let cfg = RootConfig::load(&path).unwrap();
        assert_eq!(cfg.fan_out.code_threshold, 70);
        assert_eq!(cfg.fan_out.security_threshold, 80);
        assert_eq!(cfg.fan_out.tests_threshold, 70);
        assert_eq!(cfg.fan_out.perf_threshold, 60);
        assert_eq!(cfg.fan_out.timeout, 300);
        assert!(cfg.fan_out.block_on_critical);
        assert!(cfg.fan_out.code_agent.is_none());
        assert!(cfg.fan_out.security_agent.is_none());
    }

    #[test]
    fn fanout_config_round_trips_toml() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("solidspec.toml");
        let content = r#"
[project]
name = "fanout-test"

[fan_out]
code_agent     = "claude"
security_agent = "gemini"
code_threshold     = 75
security_threshold = 90
tests_threshold    = 65
perf_threshold     = 55
timeout            = 120
block_on_critical  = false
"#;
        std::fs::write(&path, content).unwrap();
        let cfg = RootConfig::load(&path).unwrap();
        assert_eq!(cfg.fan_out.code_agent.as_deref(), Some("claude"));
        assert_eq!(cfg.fan_out.security_agent.as_deref(), Some("gemini"));
        assert!(cfg.fan_out.tests_agent.is_none());
        assert_eq!(cfg.fan_out.code_threshold, 75);
        assert_eq!(cfg.fan_out.security_threshold, 90);
        assert_eq!(cfg.fan_out.tests_threshold, 65);
        assert_eq!(cfg.fan_out.perf_threshold, 55);
        assert_eq!(cfg.fan_out.timeout, 120);
        assert!(!cfg.fan_out.block_on_critical);
    }
}
