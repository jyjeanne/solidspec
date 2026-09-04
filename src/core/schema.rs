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
pub struct WorkflowSchema {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
    /// One-line "use this when..." blurb — shown by `solidspec schemas`
    /// (docs/simplification-study-openspec.md item #5: one command to
    /// discover all schemas and their specifics, instead of a README table
    /// a new user has to read before running anything).
    #[serde(default)]
    pub use_case: String,
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
    pub const INTENT_DRIVEN: &str = include_str!("../../schemas/intent-driven/schema.yaml");
    pub const APEX_DRIVEN: &str = include_str!("../../schemas/apex-driven/schema.yaml");
    pub const INTENT_APEX: &str = include_str!("../../schemas/intent-apex/schema.yaml");
    pub const TDD_DRIVEN: &str = include_str!("../../schemas/tdd-driven/schema.yaml");

    /// Used by `list_available_schemas` below, which backs `solidspec schemas`.
    pub fn names() -> Vec<&'static str> {
        vec![
            "spec-driven",
            "minimal",
            "security-first",
            "intent-driven",
            "apex-driven",
            "intent-apex",
            "tdd-driven",
        ]
    }

    pub fn by_name(name: &str) -> Option<&'static str> {
        match name {
            "spec-driven" => Some(SPEC_DRIVEN),
            "minimal" => Some(MINIMAL),
            "security-first" => Some(SECURITY_FIRST),
            "intent-driven" => Some(INTENT_DRIVEN),
            "apex-driven" => Some(APEX_DRIVEN),
            "intent-apex" => Some(INTENT_APEX),
            "tdd-driven" => Some(TDD_DRIVEN),
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

/// True for schemas that follow the IDSD workflow: an `intent` phase runs
/// before `specify` (creating the feature directory), and intent.md drives
/// template selection, constraint gates, and drift metrics.
pub fn is_intent_schema(name: &str) -> bool {
    matches!(name, "intent-driven" | "intent-apex")
}

/// List all available schema names (built-in + project-local). Backs
/// `solidspec schemas`.
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
                use_case: schema.use_case,
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
                        // The directory name, not `schema.name`: nothing
                        // enforces that a project-local schema.yaml's own
                        // `name:` field matches the directory it lives in
                        // (e.g. one started as a copy of another schema's
                        // schema.yaml with that field left unedited) — this
                        // is the identifier `--schema`/`[pipeline].schema`
                        // actually resolves by (see `resolve_schema`'s
                        // Level 1), so it's the one that must be shown and
                        // used, not the possibly-stale label.
                        name,
                        version: schema.version,
                        description: schema.description,
                        use_case: schema.use_case,
                        artifact_count: schema.artifacts.len(),
                        source: "project-local".into(),
                    });
                }
            }
        }
    }

    schemas
}

/// Summary info about an available schema. Return type of
/// `list_available_schemas`, printed by `solidspec schemas`.
#[derive(Debug, Clone)]
pub struct SchemaInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub use_case: String,
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
        assert_eq!(schema.artifacts.len(), 9);
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
        assert_eq!(order.len(), 9);
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
    fn spec_driven_schema_has_ship_artifact() {
        let schema = WorkflowSchema::parse(builtin::SPEC_DRIVEN).unwrap();
        let ship = schema.artifacts.iter().find(|a| a.id == "ship");
        assert!(
            ship.is_some(),
            "spec-driven schema must contain a 'ship' artifact"
        );
        let ship = ship.unwrap();
        assert!(
            ship.requires.contains(&"analyze".to_string()),
            "ship must require analyze"
        );
        assert!(
            ship.requires.contains(&"review".to_string()),
            "ship must require review"
        );
    }

    #[test]
    fn intent_driven_schema_has_ship_artifact() {
        let schema = WorkflowSchema::parse(builtin::INTENT_DRIVEN).unwrap();
        assert_eq!(
            schema.artifacts.len(),
            11,
            "intent-driven schema must have 11 artifacts"
        );
        let ship = schema.artifacts.iter().find(|a| a.id == "ship");
        assert!(
            ship.is_some(),
            "intent-driven schema must contain a 'ship' artifact"
        );
    }

    #[test]
    fn intent_driven_schema_has_evidence_artifact() {
        let schema = WorkflowSchema::parse(builtin::INTENT_DRIVEN).unwrap();
        assert_eq!(schema.name, "intent-driven");
        let evidence = schema.artifacts.iter().find(|a| a.id == "evidence");
        assert!(
            evidence.is_some(),
            "intent-driven schema must contain an 'evidence' artifact"
        );
        let ev = evidence.unwrap();
        assert!(
            ev.requires.contains(&"implement".to_string()),
            "evidence must require implement"
        );
        assert!(
            ev.requires.contains(&"tests".to_string()),
            "evidence must require tests"
        );
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
    fn list_available_schemas_project_local_uses_directory_name_not_yaml_label() {
        // A project-local schema's schema.yaml can carry a `name:` field
        // that doesn't match the directory it lives in (e.g. started as a
        // copy of an existing schema.yaml with that field left unedited) —
        // list_available_schemas (and everything downstream of it, like
        // `solidspec schemas` and /spcx:<name>:* registration) must report
        // the directory name, the actual identifier `--schema`/
        // `[pipeline].schema` resolve by, not the possibly-stale label.
        let dir = TempDir::new().unwrap();
        let workflow_dir = dir.path().join(".solidspec/workflows/my-flow");
        std::fs::create_dir_all(&workflow_dir).unwrap();
        // builtin::SPEC_DRIVEN's own `name:` field is "spec-driven", not
        // "my-flow" — written verbatim, deliberately mismatched.
        std::fs::write(workflow_dir.join("schema.yaml"), builtin::SPEC_DRIVEN).unwrap();

        let schemas = list_available_schemas(dir.path());
        let custom = schemas
            .iter()
            .find(|s| s.source == "project-local")
            .expect("project-local schema missing");
        assert_eq!(custom.name, "my-flow");
    }

    // ── APEX schemas ──────────────────────────────────────────────────────────

    #[test]
    fn parse_apex_driven_schema() {
        let schema = WorkflowSchema::parse(builtin::APEX_DRIVEN).unwrap();
        assert_eq!(schema.name, "apex-driven");
        assert_eq!(schema.version, "1.0");
        assert_eq!(schema.artifacts.len(), 9);
    }

    #[test]
    fn apex_driven_has_apex_not_implement() {
        let schema = WorkflowSchema::parse(builtin::APEX_DRIVEN).unwrap();
        assert!(
            schema.artifacts.iter().any(|a| a.id == "apex"),
            "apex-driven must contain an 'apex' artifact"
        );
        assert!(
            !schema.artifacts.iter().any(|a| a.id == "implement"),
            "apex-driven must not contain an 'implement' artifact"
        );
    }

    #[test]
    fn apex_driven_apex_requires_tasks() {
        let schema = WorkflowSchema::parse(builtin::APEX_DRIVEN).unwrap();
        let apex = schema.artifacts.iter().find(|a| a.id == "apex").unwrap();
        assert!(
            apex.requires.contains(&"tasks".to_string()),
            "apex artifact must require tasks"
        );
    }

    #[test]
    fn apex_driven_schema_has_ship_artifact() {
        let schema = WorkflowSchema::parse(builtin::APEX_DRIVEN).unwrap();
        let ship = schema.artifacts.iter().find(|a| a.id == "ship").unwrap();
        assert!(ship.requires.contains(&"analyze".to_string()));
        assert!(ship.requires.contains(&"review".to_string()));
    }

    #[test]
    fn apex_driven_converts_to_valid_graph() {
        let schema = WorkflowSchema::parse(builtin::APEX_DRIVEN).unwrap();
        let graph = schema.into_graph().unwrap();
        let order = graph.topological_order().unwrap();
        assert_eq!(order.len(), 9);
    }

    #[test]
    fn parse_intent_apex_schema() {
        let schema = WorkflowSchema::parse(builtin::INTENT_APEX).unwrap();
        assert_eq!(schema.name, "intent-apex");
        assert_eq!(schema.version, "1.0");
        assert_eq!(schema.artifacts.len(), 11);
    }

    #[test]
    fn intent_apex_has_apex_not_implement() {
        let schema = WorkflowSchema::parse(builtin::INTENT_APEX).unwrap();
        assert!(
            schema.artifacts.iter().any(|a| a.id == "apex"),
            "intent-apex must contain an 'apex' artifact"
        );
        assert!(
            !schema.artifacts.iter().any(|a| a.id == "implement"),
            "intent-apex must not contain an 'implement' artifact"
        );
    }

    #[test]
    fn intent_apex_evidence_requires_apex_not_implement() {
        let schema = WorkflowSchema::parse(builtin::INTENT_APEX).unwrap();
        let evidence = schema
            .artifacts
            .iter()
            .find(|a| a.id == "evidence")
            .unwrap();
        assert!(
            evidence.requires.contains(&"apex".to_string()),
            "evidence must require apex in intent-apex schema"
        );
        assert!(
            !evidence.requires.contains(&"implement".to_string()),
            "evidence must not require implement in intent-apex schema"
        );
        assert!(
            evidence.requires.contains(&"tests".to_string()),
            "evidence must still require tests"
        );
    }

    #[test]
    fn intent_apex_has_intent_and_evidence() {
        let schema = WorkflowSchema::parse(builtin::INTENT_APEX).unwrap();
        assert!(schema.artifacts.iter().any(|a| a.id == "intent"));
        assert!(schema.artifacts.iter().any(|a| a.id == "evidence"));
    }

    #[test]
    fn intent_apex_converts_to_valid_graph() {
        let schema = WorkflowSchema::parse(builtin::INTENT_APEX).unwrap();
        let graph = schema.into_graph().unwrap();
        let order = graph.topological_order().unwrap();
        assert_eq!(order.len(), 11);
    }

    #[test]
    fn resolve_apex_driven_builtin() {
        let dir = TempDir::new().unwrap();
        let (schema, source) = resolve_schema("apex-driven", dir.path()).unwrap();
        assert_eq!(schema.name, "apex-driven");
        assert_eq!(source, SchemaSource::Builtin);
    }

    #[test]
    fn resolve_intent_apex_builtin() {
        let dir = TempDir::new().unwrap();
        let (schema, source) = resolve_schema("intent-apex", dir.path()).unwrap();
        assert_eq!(schema.name, "intent-apex");
        assert_eq!(source, SchemaSource::Builtin);
    }

    #[test]
    fn list_available_schemas_includes_apex_schemas() {
        let dir = TempDir::new().unwrap();
        let schemas = list_available_schemas(dir.path());
        assert!(schemas.iter().any(|s| s.name == "apex-driven"));
        assert!(schemas.iter().any(|s| s.name == "intent-apex"));
    }

    #[test]
    fn is_intent_schema_matches_both_idsd_variants() {
        assert!(is_intent_schema("intent-driven"));
        assert!(is_intent_schema("intent-apex"));
        assert!(!is_intent_schema("spec-driven"));
        assert!(!is_intent_schema("apex-driven"));
        assert!(!is_intent_schema("minimal"));
        assert!(!is_intent_schema("tdd-driven"));
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
