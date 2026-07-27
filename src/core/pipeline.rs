use std::path::Path;

use anyhow::Result;

use super::artifact_graph::ArtifactGraph;
use super::spec_parser;

/// Pipeline phase names for the default SDD workflow.
pub const PHASES: &[&str] = &[
    "specify",
    "clarify",
    "plan",
    "tasks",
    "tests",
    "implement",
    "analyze",
    "review",
];

/// Pipeline phase names for the IDSD workflow (`intent-driven` schema).
/// Intent is phase 0 — it runs before spec creation.
/// Evidence runs after implement to measure criterion satisfaction.
pub const PHASES_IDSD: &[&str] = &[
    "intent",
    "specify",
    "clarify",
    "plan",
    "tasks",
    "tests",
    "implement",
    "evidence",
    "analyze",
    "review",
];

/// Pipeline phase names for the `apex-driven` schema.
/// Identical to PHASES except `apex` replaces `implement`.
pub const PHASES_APEX: &[&str] = &[
    "specify", "clarify", "plan", "tasks", "tests", "apex", "analyze", "review",
];

/// Pipeline phase names for the `intent-apex` schema.
/// Identical to PHASES_IDSD except `apex` replaces `implement`.
pub const PHASES_APEX_IDSD: &[&str] = &[
    "intent", "specify", "clarify", "plan", "tasks", "tests", "apex", "evidence", "analyze",
    "review",
];

/// Pipeline phase names for the `tdd-driven` schema.
/// Identical to PHASES except `tests` → `tdd-tests` and `tdd-refactor` is added after `implement`.
pub const PHASES_TDD: &[&str] = &[
    "specify",
    "clarify",
    "plan",
    "tasks",
    "tdd-tests",
    "implement",
    "tdd-refactor",
    "analyze",
    "review",
];

/// Pipeline phase names for the `minimal` schema (`schemas/minimal/schema.yaml`):
/// only spec/plan/tasks/implement — no clarify, tests, analyze, or review artifacts.
pub const PHASES_MINIMAL: &[&str] = &["specify", "plan", "tasks", "implement"];

/// Pipeline phase names for the `security-first` schema
/// (`schemas/security-first/schema.yaml`): adds `security-review` between
/// plan and tasks; no clarify, tests, analyze, or review artifacts.
///
/// `execute_phase` (`cli/pipeline.rs`) runs `security-review` via
/// `solidspec security-review`, which performs an OWASP Top 10 heuristic
/// audit of `plan.md` (`core::security_review`) and writes
/// `security-review.md` — no AI agent required, so `--no-agent` runs of
/// this schema complete end to end.
pub const PHASES_SECURITY_FIRST: &[&str] =
    &["specify", "plan", "security-review", "tasks", "implement"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhaseType {
    Auto,
    Handoff,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhaseStatus {
    /// Reserved for a future live-progress reporter; the current synchronous
    /// CLI only ever records a phase's outcome after it finishes, so these
    /// two intermediate states are never actually constructed today.
    #[allow(dead_code)]
    Pending,
    #[allow(dead_code)]
    Running,
    Done,
    Skipped,
    Failed,
    Handoff,
}

impl std::fmt::Display for PhaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhaseStatus::Pending => write!(f, "pending"),
            PhaseStatus::Running => write!(f, "running"),
            PhaseStatus::Done => write!(f, "done"),
            PhaseStatus::Skipped => write!(f, "skipped"),
            PhaseStatus::Failed => write!(f, "failed"),
            PhaseStatus::Handoff => write!(f, "handoff"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PhaseResult {
    pub name: String,
    pub agent: String,
    pub status: PhaseStatus,
    pub duration_ms: u64,
    pub output: String,
}

/// Map a pipeline phase name to its schema artifact ID. Every phase name
/// matches its artifact ID directly except `specify`, whose schema artifact
/// is named `spec` (see `schemas/*/schema.yaml`).
fn schema_artifact_id(phase: &str) -> &str {
    if phase == "specify" { "spec" } else { phase }
}

/// Check if a phase should be skipped based on existing artifacts.
///
/// Phases whose completion can't be determined from file existence alone
/// (`clarify`, `implement`, `apex`, `analyze`, `tdd-tests`) use dedicated,
/// content-aware checks. Every other phase — including artifacts introduced
/// or extended by a project-local schema override — defers to `graph`'s
/// `generates` declarations, so schema overrides actually take effect here
/// instead of being silently ignored.
pub fn should_skip(phase: &str, feature_dir: &Path, force: bool, graph: &ArtifactGraph) -> bool {
    if force {
        return false;
    }
    match phase {
        "clarify" => {
            let spec_path = feature_dir.join("spec.md");
            if !spec_path.exists() {
                return true; // no spec to clarify
            }
            if let Ok(spec) = spec_parser::parse_spec(&spec_path) {
                spec.clarification_markers.is_empty()
            } else {
                true
            }
        }
        "implement" => {
            let tasks_path = feature_dir.join("tasks.md");
            if !tasks_path.exists() {
                return true;
            }
            if let Ok(content) = std::fs::read_to_string(&tasks_path) {
                content.matches("- [ ] T").count() == 0
            } else {
                true
            }
        }
        "analyze" => false, // never skipped
        "apex" => {
            // Skip only when APEX fully completed: 09-finish.md must exist inside
            // a run subdirectory of feature_dir/apex/. Merely having the directory
            // present (an in-progress run) does NOT count as done.
            // This path is populated when APEX is run with --save and --output-dir
            // pointing to the feature directory; without --save it never auto-skips.
            let apex_dir = feature_dir.join("apex");
            if !apex_dir.exists() {
                return false;
            }
            std::fs::read_dir(&apex_dir)
                .ok()
                .map(|d| {
                    d.filter_map(|e| e.ok())
                        .filter(|e| e.path().is_dir())
                        .any(|run_dir| run_dir.path().join("09-finish.md").exists())
                })
                .unwrap_or(false)
        }
        // TDD RED phase: intentionally checked by report existence only —
        // the `tests/` scaffold is written on this same run before the
        // report, so requiring it non-empty here would never let this
        // phase skip on a later invocation.
        "tdd-tests" => feature_dir.join("tdd-red-report.md").exists(),
        _ => graph
            .get(schema_artifact_id(phase))
            .map(|node| ArtifactGraph::generates_present(node, feature_dir))
            .unwrap_or(false),
    }
}

/// Get the phase type (auto or handoff).
pub fn phase_type(phase: &str) -> PhaseType {
    match phase {
        "implement" | "apex" | "tdd-tests" | "tdd-refactor" => PhaseType::Handoff,
        _ => PhaseType::Auto, // ship, evidence, analyze, review, etc. are all auto
    }
}

/// Filter phases by --from and --to range for the given schema.
pub fn filter_phases(
    schema: &str,
    from: Option<&str>,
    to: Option<&str>,
) -> Result<Vec<&'static str>> {
    let all: &[&str] = match schema {
        "intent-driven" => PHASES_IDSD,
        "apex-driven" => PHASES_APEX,
        "intent-apex" => PHASES_APEX_IDSD,
        "tdd-driven" => PHASES_TDD,
        "minimal" => PHASES_MINIMAL,
        "security-first" => PHASES_SECURITY_FIRST,
        _ => PHASES, // spec-driven, custom, unknown
    };
    let from_idx = if let Some(f) = from {
        all.iter().position(|p| *p == f).ok_or_else(|| {
            anyhow::anyhow!("Unknown phase '{}'. Valid phases: {}", f, all.join(", "))
        })?
    } else {
        0
    };

    let to_idx = if let Some(t) = to {
        all.iter().position(|p| *p == t).ok_or_else(|| {
            anyhow::anyhow!("Unknown phase '{}'. Valid phases: {}", t, all.join(", "))
        })?
    } else {
        all.len() - 1
    };

    if from_idx > to_idx {
        anyhow::bail!(
            "--from '{}' comes after --to '{}'. Phases run in order: {}",
            all[from_idx],
            all[to_idx],
            all.join(" → ")
        );
    }

    Ok(all[from_idx..=to_idx].to_vec())
}

/// Generate the pipeline log entry for a run.
pub fn format_log_entry(results: &[PhaseResult]) -> String {
    let timestamp = chrono::Utc::now().to_rfc3339();
    let mut out = format!("## Run {timestamp}\n\n");
    out.push_str("| Phase | Agent | Status | Duration | Output |\n");
    out.push_str("|-------|-------|--------|----------|--------|\n");

    let mut total_ms: u64 = 0;
    let mut agents = Vec::new();

    for r in results {
        let duration = if r.status == PhaseStatus::Skipped || r.status == PhaseStatus::Handoff {
            "—".to_string()
        } else {
            total_ms += r.duration_ms;
            format!("{:.1}s", r.duration_ms as f64 / 1000.0)
        };
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            r.name, r.agent, r.status, duration, r.output
        ));
        if !agents.contains(&r.agent) {
            agents.push(r.agent.clone());
        }
    }

    let all_ok = results.iter().all(|r| {
        r.status == PhaseStatus::Done
            || r.status == PhaseStatus::Skipped
            || r.status == PhaseStatus::Handoff
    });
    let status = if all_ok { "complete" } else { "failed" };

    out.push_str(&format!(
        "\n**Total:** {:.1}s (automated) | **Agents:** {} | **Status:** {status}\n",
        total_ms as f64 / 1000.0,
        agents.join(", ")
    ));
    out
}

/// Write or append the pipeline log.
pub fn write_log(feature_dir: &Path, feature_name: &str, results: &[PhaseResult]) -> Result<()> {
    let log_path = feature_dir.join("pipeline-log.md");
    let entry = format_log_entry(results);

    let content = if log_path.exists() {
        let existing = std::fs::read_to_string(&log_path)?;
        format!("{existing}\n---\n\n{entry}")
    } else {
        format!("# Pipeline Log: {feature_name}\n\n{entry}")
    };

    std::fs::write(&log_path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Build the artifact graph for a built-in schema, for should_skip tests.
    fn graph_for(schema_name: &str) -> ArtifactGraph {
        let (schema, _) = crate::core::schema::resolve_schema(schema_name, Path::new(".")).unwrap();
        schema.into_graph().unwrap()
    }

    #[test]
    fn filter_all_phases() {
        let phases = filter_phases("spec-driven", None, None).unwrap();
        assert_eq!(phases.len(), 8);
        assert_eq!(phases[0], "specify");
        assert_eq!(phases[7], "review");
    }

    #[test]
    fn filter_idsd_phases_includes_intent_and_evidence() {
        let phases = filter_phases("intent-driven", None, None).unwrap();
        assert_eq!(phases.len(), 10);
        assert_eq!(phases[0], "intent");
        assert_eq!(phases[1], "specify");
        assert_eq!(phases[7], "evidence");
        assert_eq!(phases[9], "review");
    }

    #[test]
    fn filter_minimal_phases_excludes_tests_and_review() {
        let phases = filter_phases("minimal", None, None).unwrap();
        assert_eq!(phases, vec!["specify", "plan", "tasks", "implement"]);
    }

    #[test]
    fn filter_security_first_phases_includes_security_review() {
        let phases = filter_phases("security-first", None, None).unwrap();
        assert_eq!(
            phases,
            vec!["specify", "plan", "security-review", "tasks", "implement"]
        );
    }

    #[test]
    fn filter_from_plan_to_tasks() {
        let phases = filter_phases("spec-driven", Some("plan"), Some("tasks")).unwrap();
        assert_eq!(phases, vec!["plan", "tasks"]);
    }

    #[test]
    fn filter_only_one_phase() {
        let phases = filter_phases("spec-driven", Some("plan"), Some("plan")).unwrap();
        assert_eq!(phases, vec!["plan"]);
    }

    #[test]
    fn filter_from_after_to_errors() {
        let result = filter_phases("spec-driven", Some("analyze"), Some("plan"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("comes after"));
    }

    #[test]
    fn filter_invalid_phase_errors() {
        let result = filter_phases("spec-driven", Some("nonexistent"), None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown phase"));
    }

    #[test]
    fn should_skip_specify_when_spec_exists() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("spec-driven");
        std::fs::write(dir.path().join("spec.md"), "# Spec").unwrap();
        assert!(should_skip("specify", dir.path(), false, &graph));
        assert!(!should_skip("specify", dir.path(), true, &graph)); // force overrides
    }

    #[test]
    fn should_skip_clarify_when_no_markers() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("spec-driven");
        std::fs::write(dir.path().join("spec.md"), "# Spec\nNo markers here.").unwrap();
        assert!(should_skip("clarify", dir.path(), false, &graph));
    }

    #[test]
    fn should_not_skip_clarify_when_markers_present() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("spec-driven");
        std::fs::write(
            dir.path().join("spec.md"),
            "# Spec\n[NEEDS CLARIFICATION: something]",
        )
        .unwrap();
        assert!(!should_skip("clarify", dir.path(), false, &graph));
    }

    #[test]
    fn should_skip_implement_when_all_tasks_done() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("spec-driven");
        std::fs::write(
            dir.path().join("tasks.md"),
            "- [x] T001 Done\n- [x] T002 Done\n",
        )
        .unwrap();
        assert!(should_skip("implement", dir.path(), false, &graph));
    }

    #[test]
    fn should_not_skip_implement_when_tasks_pending() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("spec-driven");
        std::fs::write(
            dir.path().join("tasks.md"),
            "- [x] T001 Done\n- [ ] T002 Pending\n",
        )
        .unwrap();
        assert!(!should_skip("implement", dir.path(), false, &graph));
    }

    #[test]
    fn analyze_never_skipped() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("spec-driven");
        assert!(!should_skip("analyze", dir.path(), false, &graph));
    }

    #[test]
    fn phase_types_correct() {
        assert_eq!(phase_type("intent"), PhaseType::Auto);
        assert_eq!(phase_type("specify"), PhaseType::Auto);
        assert_eq!(phase_type("plan"), PhaseType::Auto);
        assert_eq!(phase_type("implement"), PhaseType::Handoff);
        assert_eq!(phase_type("apex"), PhaseType::Handoff);
        assert_eq!(phase_type("evidence"), PhaseType::Auto);
        assert_eq!(phase_type("analyze"), PhaseType::Auto);
        assert_eq!(phase_type("ship"), PhaseType::Auto);
    }

    #[test]
    fn should_skip_ship_when_report_exists() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("spec-driven");
        std::fs::write(dir.path().join("ship-report.md"), "<!-- ship: true -->").unwrap();
        assert!(should_skip("ship", dir.path(), false, &graph));
        assert!(!should_skip("ship", dir.path(), true, &graph)); // force overrides
    }

    #[test]
    fn should_not_skip_ship_when_report_absent() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("spec-driven");
        assert!(!should_skip("ship", dir.path(), false, &graph));
    }

    #[test]
    fn should_skip_evidence_when_report_exists() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("intent-driven");
        std::fs::write(dir.path().join("evidence-report.md"), "# Evidence").unwrap();
        assert!(should_skip("evidence", dir.path(), false, &graph));
        assert!(!should_skip("evidence", dir.path(), true, &graph));
    }

    #[test]
    fn should_not_skip_evidence_when_absent() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("intent-driven");
        assert!(!should_skip("evidence", dir.path(), false, &graph));
    }

    #[test]
    fn should_skip_intent_when_intent_exists() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("intent-driven");
        std::fs::write(dir.path().join("intent.md"), "# Intent: Test").unwrap();
        assert!(should_skip("intent", dir.path(), false, &graph));
        assert!(!should_skip("intent", dir.path(), true, &graph)); // force overrides
    }

    #[test]
    fn should_not_skip_intent_when_absent() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("intent-driven");
        assert!(!should_skip("intent", dir.path(), false, &graph));
    }

    #[test]
    fn format_log_has_table_and_totals() {
        let results = vec![
            PhaseResult {
                name: "specify".into(),
                agent: "claude".into(),
                status: PhaseStatus::Done,
                duration_ms: 1200,
                output: "spec.md".into(),
            },
            PhaseResult {
                name: "clarify".into(),
                agent: "claude".into(),
                status: PhaseStatus::Skipped,
                duration_ms: 0,
                output: "no markers".into(),
            },
        ];
        let log = format_log_entry(&results);
        assert!(log.contains("| specify | claude | done | 1.2s |"));
        assert!(log.contains("| clarify | claude | skipped | — |"));
        assert!(log.contains("**Total:**"));
        assert!(log.contains("complete"));
    }

    #[test]
    fn write_log_creates_file() {
        let dir = TempDir::new().unwrap();
        let results = vec![PhaseResult {
            name: "plan".into(),
            agent: "claude".into(),
            status: PhaseStatus::Done,
            duration_ms: 500,
            output: "plan.md".into(),
        }];
        write_log(dir.path(), "001-test", &results).unwrap();
        let content = std::fs::read_to_string(dir.path().join("pipeline-log.md")).unwrap();
        assert!(content.contains("# Pipeline Log: 001-test"));
        assert!(content.contains("| plan | claude | done |"));
    }

    #[test]
    fn write_log_appends_to_existing() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("pipeline-log.md"),
            "# Pipeline Log: 001\n\n## Run 1\nold content\n",
        )
        .unwrap();
        let results = vec![PhaseResult {
            name: "analyze".into(),
            agent: "vibe".into(),
            status: PhaseStatus::Done,
            duration_ms: 300,
            output: "100%".into(),
        }];
        write_log(dir.path(), "001", &results).unwrap();
        let content = std::fs::read_to_string(dir.path().join("pipeline-log.md")).unwrap();
        assert!(content.contains("old content"));
        assert!(content.contains("| analyze | vibe | done |"));
    }

    // ── APEX phase tests ──────────────────────────────────────────────────────

    #[test]
    fn filter_apex_driven_has_apex_not_implement() {
        let phases = filter_phases("apex-driven", None, None).unwrap();
        assert_eq!(phases.len(), 8);
        assert!(
            phases.contains(&"apex"),
            "apex-driven must include apex phase"
        );
        assert!(
            !phases.contains(&"implement"),
            "apex-driven must not include implement phase"
        );
    }

    #[test]
    fn filter_apex_driven_apex_at_correct_position() {
        let phases = filter_phases("apex-driven", None, None).unwrap();
        // apex follows tests (index 4) and precedes analyze (index 6)
        let apex_idx = phases.iter().position(|p| *p == "apex").unwrap();
        let tests_idx = phases.iter().position(|p| *p == "tests").unwrap();
        let analyze_idx = phases.iter().position(|p| *p == "analyze").unwrap();
        assert!(tests_idx < apex_idx);
        assert!(apex_idx < analyze_idx);
    }

    #[test]
    fn filter_intent_apex_has_all_idsd_phases_with_apex() {
        let phases = filter_phases("intent-apex", None, None).unwrap();
        assert_eq!(phases.len(), 10);
        assert_eq!(phases[0], "intent");
        assert!(phases.contains(&"apex"));
        assert!(phases.contains(&"evidence"));
        assert!(!phases.contains(&"implement"));
    }

    #[test]
    fn filter_intent_apex_apex_before_evidence() {
        let phases = filter_phases("intent-apex", None, None).unwrap();
        let apex_idx = phases.iter().position(|p| *p == "apex").unwrap();
        let evidence_idx = phases.iter().position(|p| *p == "evidence").unwrap();
        assert!(apex_idx < evidence_idx);
    }

    #[test]
    fn filter_apex_driven_from_tasks_to_analyze() {
        let phases = filter_phases("apex-driven", Some("tasks"), Some("analyze")).unwrap();
        assert_eq!(phases, vec!["tasks", "tests", "apex", "analyze"]);
    }

    #[test]
    fn filter_apex_driven_only_apex() {
        let phases = filter_phases("apex-driven", Some("apex"), Some("apex")).unwrap();
        assert_eq!(phases, vec!["apex"]);
    }

    #[test]
    fn filter_existing_schemas_unchanged_by_apex_addition() {
        // spec-driven and intent-driven must be unaffected
        let sdd = filter_phases("spec-driven", None, None).unwrap();
        assert_eq!(sdd.len(), 8);
        assert!(sdd.contains(&"implement"));
        assert!(!sdd.contains(&"apex"));

        let idsd = filter_phases("intent-driven", None, None).unwrap();
        assert_eq!(idsd.len(), 10);
        assert!(idsd.contains(&"implement"));
        assert!(!idsd.contains(&"apex"));
    }

    #[test]
    fn should_skip_apex_false_when_no_apex_dir() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("apex-driven");
        assert!(!should_skip("apex", dir.path(), false, &graph));
    }

    #[test]
    fn should_skip_apex_false_when_apex_dir_empty() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("apex-driven");
        std::fs::create_dir(dir.path().join("apex")).unwrap();
        assert!(!should_skip("apex", dir.path(), false, &graph));
    }

    #[test]
    fn should_skip_apex_false_when_run_dir_has_no_finish() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("apex-driven");
        let run = dir.path().join("apex").join("auth-system");
        std::fs::create_dir_all(&run).unwrap();
        std::fs::write(run.join("03-execute.md"), "in progress").unwrap();
        assert!(!should_skip("apex", dir.path(), false, &graph));
    }

    #[test]
    fn should_skip_apex_true_when_finish_exists() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("apex-driven");
        let run = dir.path().join("apex").join("auth-system");
        std::fs::create_dir_all(&run).unwrap();
        std::fs::write(run.join("09-finish.md"), "done").unwrap();
        assert!(should_skip("apex", dir.path(), false, &graph));
    }

    #[test]
    fn should_skip_apex_false_when_force() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("apex-driven");
        let run = dir.path().join("apex").join("auth-system");
        std::fs::create_dir_all(&run).unwrap();
        std::fs::write(run.join("09-finish.md"), "done").unwrap();
        assert!(!should_skip("apex", dir.path(), true, &graph)); // force overrides
    }

    #[test]
    fn should_skip_apex_ignores_file_entries_in_apex_dir() {
        // Files directly in apex/ (not subdirectories) must not trigger skip
        let dir = TempDir::new().unwrap();
        let graph = graph_for("apex-driven");
        let apex_dir = dir.path().join("apex");
        std::fs::create_dir_all(&apex_dir).unwrap();
        std::fs::write(apex_dir.join("09-finish.md"), "stray file").unwrap();
        assert!(!should_skip("apex", dir.path(), false, &graph));
    }

    #[test]
    fn should_skip_tests_when_dir_nonempty_via_schema_generates() {
        // "tests" has no dedicated match arm; verifies the schema-driven
        // default correctly consults ArtifactGraph::generates_present.
        let dir = TempDir::new().unwrap();
        let graph = graph_for("spec-driven");
        assert!(!should_skip("tests", dir.path(), false, &graph));
        let tests_dir = dir.path().join("tests");
        std::fs::create_dir(&tests_dir).unwrap();
        assert!(
            !should_skip("tests", dir.path(), false, &graph),
            "empty tests/ dir must not count as complete"
        );
        std::fs::write(tests_dir.join("story1.md"), "# Test").unwrap();
        assert!(should_skip("tests", dir.path(), false, &graph));
    }

    #[test]
    fn should_skip_unknown_phase_defaults_false_when_absent_from_schema() {
        let dir = TempDir::new().unwrap();
        let graph = graph_for("spec-driven");
        assert!(!should_skip("nonexistent-phase", dir.path(), false, &graph));
    }

    #[test]
    fn should_skip_security_review_when_report_exists() {
        // Regression test: security-review is a real artifact id in the
        // security-first schema's graph, so the schema-driven default arm
        // must correctly detect security-review.md and skip.
        let dir = TempDir::new().unwrap();
        let graph = graph_for("security-first");
        assert!(!should_skip("security-review", dir.path(), false, &graph));
        std::fs::write(dir.path().join("security-review.md"), "# Security Review").unwrap();
        assert!(should_skip("security-review", dir.path(), false, &graph));
    }

    #[test]
    fn filter_phases_minimal_and_security_first_never_include_undeclared_phases() {
        // Regression test: filter_phases must route each schema to its own
        // phase list rather than falling back to the generic 8-phase
        // spec-driven list, which would include phases (tests, review,
        // clarify, analyze) these schemas don't declare as artifacts —
        // and which should_skip's schema-driven default can never skip,
        // since graph.get() returns None for them on these schemas.
        for phase in ["clarify", "tests", "analyze", "review"] {
            assert!(
                !filter_phases("minimal", None, None)
                    .unwrap()
                    .contains(&phase),
                "minimal schema must not include phase '{phase}'"
            );
            assert!(
                !filter_phases("security-first", None, None)
                    .unwrap()
                    .contains(&phase),
                "security-first schema must not include phase '{phase}'"
            );
        }
    }
}
