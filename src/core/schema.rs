//! Workflow schema loading and resolution.
//!
//! Schemas define workflow artifact graphs in YAML. Resolution follows
//! a 3-level cascade:
//! 1. `.solidspec/workflows/<name>/schema.yaml` — project-local
//! 2. Built-in schemas embedded in the binary
//! 3. Default: `spec-driven` built-in schema

use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

use super::artifact_graph::{ArtifactGraph, ArtifactNode};

/// A workflow schema deserialized from YAML.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct WorkflowSchema {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
    pub artifacts: Vec<SchemaArtifact>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SchemaArtifact {
    pub id: String,
    #[serde(default)]
    pub generates: Vec<String>,
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub instruction: String,
    #[serde(default)]
    pub template: Option<String>,
}

/// Embedded built-in schemas.
pub mod builtin {
    pub const SPEC_DRIVEN: &str = include_str!("../../schemas/spec-driven/schema.yaml");
    pub const MINIMAL: &str = include_str!("../../schemas/minimal/schema.yaml");
    pub const SECURITY_FIRST: &str = include_str!("../../schemas/security-first/schema.yaml");

    #[allow(dead_code)]
    pub fn names() -> Vec<&'static str> {
        vec!["spec-driven", "minimal", "security-first"]
    }

    pub fn by_name(name: &str) -> Option<&'static str> {
        match name {
            "spec-driven" => Some(SPEC_DRIVEN),
            "minimal" => Some(MINIMAL),
            "security-first" => Some(SECURITY_FIRST),
            _ => None,
        }
    }
}

impl WorkflowSchema {
    /// Parse a schema from a YAML string.
    pub fn parse(yaml: &str) -> Result<Self> {
        let schema: Self = serde_yaml::from_str(yaml)?;
        Ok(schema)
    }

    /// Convert this schema into an artifact graph.
    pub fn into_graph(self) -> Result<ArtifactGraph, String> {
        let nodes: Vec<ArtifactNode> = self
            .artifacts
            .into_iter()
            .map(|a| ArtifactNode {
                id: a.id,
                generates: a.generates,
                requires: a.requires,
                instruction: a.instruction,
                template: a.template,
            })
            .collect();

        ArtifactGraph::new(nodes)
    }
}

/// 3-level schema resolution.
///
/// 1. Project-local: `.solidspec/workflows/<name>/schema.yaml`
/// 2. Built-in: embedded in binary
/// 3. Default: `spec-driven` built-in
pub fn resolve_schema(name: &str, project_root: &Path) -> Result<(WorkflowSchema, SchemaSource)> {
    // Level 1: project-local override
    let local_path = project_root
        .join(".solidspec")
        .join("workflows")
        .join(name)
        .join("schema.yaml");
    if local_path.exists() {
        let content = std::fs::read_to_string(&local_path)?;
        let schema = WorkflowSchema::parse(&content)?;
        return Ok((schema, SchemaSource::ProjectLocal(local_path)));
    }

    // Level 2: built-in
    if let Some(content) = builtin::by_name(name) {
        let schema = WorkflowSchema::parse(content)?;
        return Ok((schema, SchemaSource::Builtin));
    }

    // Level 3: default fallback
    let schema = WorkflowSchema::parse(builtin::SPEC_DRIVEN)?;
    Ok((schema, SchemaSource::Default))
}

/// Where the schema was resolved from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaSource {
    ProjectLocal(std::path::PathBuf),
    Builtin,
    Default,
}

/// List all available schema names (built-in + project-local).
#[allow(dead_code)]
pub fn list_available_schemas(project_root: &Path) -> Vec<SchemaInfo> {
    let mut schemas = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // Built-in schemas
    for name in builtin::names() {
        let content = builtin::by_name(name).unwrap();
        if let Ok(schema) = WorkflowSchema::parse(content) {
            schemas.push(SchemaInfo {
                name: schema.name,
                version: schema.version,
                description: schema.description,
                artifact_count: schema.artifacts.len(),
                source: "built-in".into(),
            });
            seen.insert(name.to_string());
        }
    }

    // Project-local overrides
    let workflows_dir = project_root.join(".solidspec/workflows");
    if let Ok(entries) = std::fs::read_dir(&workflows_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let name = entry.file_name().to_string_lossy().to_string();
                if seen.contains(&name) {
                    continue;
                }
                let schema_path = entry.path().join("schema.yaml");
                if schema_path.exists()
                    && let Ok(content) = std::fs::read_to_string(&schema_path)
                    && let Ok(schema) = WorkflowSchema::parse(&content)
                {
                    schemas.push(SchemaInfo {
                        name: schema.name,
                        version: schema.version,
                        description: schema.description,
                        artifact_count: schema.artifacts.len(),
                        source: "project-local".into(),
                    });
                }
            }
        }
    }

    schemas
}

/// Summary info about an available schema.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SchemaInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub artifact_count: usize,
    pub source: String,
}

/// Load a schema and convert to ArtifactGraph in one step.
pub fn load_graph(name: &str, project_root: &Path) -> Result<(ArtifactGraph, SchemaSource)> {
    let (schema, source) = resolve_schema(name, project_root)?;
    let graph = schema
        .into_graph()
        .map_err(|e| anyhow::anyhow!("Invalid schema '{name}': {e}"))?;
    Ok((graph, source))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn parse_spec_driven_schema() {
        let schema = WorkflowSchema::parse(builtin::SPEC_DRIVEN).unwrap();
        assert_eq!(schema.name, "spec-driven");
        assert_eq!(schema.version, "1.0");
        assert_eq!(schema.artifacts.len(), 8);
    }

    #[test]
    fn parse_minimal_schema() {
        let schema = WorkflowSchema::parse(builtin::MINIMAL).unwrap();
        assert_eq!(schema.name, "minimal");
        assert_eq!(schema.artifacts.len(), 4);
    }

    #[test]
    fn parse_security_first_schema() {
        let schema = WorkflowSchema::parse(builtin::SECURITY_FIRST).unwrap();
        assert_eq!(schema.name, "security-first");
        // security-first adds a review artifact between plan and tasks
        let security = schema.artifacts.iter().find(|a| a.id == "security-review");
        assert!(security.is_some());
    }

    #[test]
    fn schema_converts_to_valid_graph() {
        let schema = WorkflowSchema::parse(builtin::SPEC_DRIVEN).unwrap();
        let graph = schema.into_graph().unwrap();
        let order = graph.topological_order().unwrap();
        assert_eq!(order.len(), 8);
    }

    #[test]
    fn resolve_builtin_schema() {
        let dir = TempDir::new().unwrap();
        let (schema, source) = resolve_schema("spec-driven", dir.path()).unwrap();
        assert_eq!(schema.name, "spec-driven");
        assert_eq!(source, SchemaSource::Builtin);
    }

    #[test]
    fn resolve_unknown_falls_back_to_default() {
        let dir = TempDir::new().unwrap();
        let (schema, source) = resolve_schema("nonexistent", dir.path()).unwrap();
        assert_eq!(schema.name, "spec-driven");
        assert_eq!(source, SchemaSource::Default);
    }

    #[test]
    fn resolve_project_local_override() {
        let dir = TempDir::new().unwrap();
        let workflows = dir.path().join(".solidspec/workflows/custom");
        std::fs::create_dir_all(&workflows).unwrap();
        std::fs::write(
            workflows.join("schema.yaml"),
            r#"
name: custom
version: "1.0"
artifacts:
  - id: spec
    generates: ["spec.md"]
    requires: []
    instruction: "custom spec"
"#,
        )
        .unwrap();

        let (schema, source) = resolve_schema("custom", dir.path()).unwrap();
        assert_eq!(schema.name, "custom");
        matches!(source, SchemaSource::ProjectLocal(_));
    }

    #[test]
    fn list_available_schemas_includes_builtins() {
        let dir = TempDir::new().unwrap();
        let schemas = list_available_schemas(dir.path());
        assert!(schemas.iter().any(|s| s.name == "spec-driven"));
        assert!(schemas.iter().any(|s| s.name == "minimal"));
        assert!(schemas.iter().any(|s| s.name == "security-first"));
        // all built-in schemas have 2+ artifacts
        for s in &schemas {
            assert!(s.artifact_count >= 2, "{} has too few artifacts", s.name);
        }
    }

    #[test]
    fn load_graph_one_step() {
        let dir = TempDir::new().unwrap();
        let (graph, source) = load_graph("minimal", dir.path()).unwrap();
        assert_eq!(graph.nodes.len(), 4);
        assert_eq!(source, SchemaSource::Builtin);
        // minimal graph must be valid (no cycles, all deps exist)
        assert!(graph.topological_order().is_ok());
    }
}
