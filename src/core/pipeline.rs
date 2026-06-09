#![allow(dead_code)]
use std::path::Path;

use anyhow::Result;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhaseType {
    Auto,
    Handoff,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhaseStatus {
    Pending,
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

/// Check if a phase should be skipped based on existing artifacts.
pub fn should_skip(phase: &str, feature_dir: &Path, force: bool) -> bool {
    if force {
        return false;
    }
    match phase {
        "intent" => feature_dir.join("intent.md").exists(),
        "specify" => feature_dir.join("spec.md").exists(),
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
        "plan" => feature_dir.join("plan.md").exists(),
        "tasks" => feature_dir.join("tasks.md").exists(),
        "tests" => {
            let tests_dir = feature_dir.join("tests");
            tests_dir.exists()
                && std::fs::read_dir(&tests_dir)
                    .map(|mut d| d.next().is_some())
                    .unwrap_or(false)
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
        "evidence" => feature_dir.join("evidence-report.md").exists(),
        "analyze" => false, // never skipped
        "review" => feature_dir.join("review-report.md").exists(),
        "ship" => feature_dir.join("ship-report.md").exists(),
        _ => false,
    }
}

/// Get the phase type (auto or handoff).
pub fn phase_type(phase: &str) -> PhaseType {
    match phase {
        "implement" => PhaseType::Handoff,
        _ => PhaseType::Auto, // ship, evidence, analyze, review, etc. are all auto
    }
}

/// Filter phases by --from and --to range for the given schema.
/// Pass `schema = "intent-driven"` to include the `intent` phase 0.
pub fn filter_phases(
    schema: &str,
    from: Option<&str>,
    to: Option<&str>,
) -> Result<Vec<&'static str>> {
    let all = if schema == "intent-driven" {
        PHASES_IDSD
    } else {
        PHASES
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
        std::fs::write(dir.path().join("spec.md"), "# Spec").unwrap();
        assert!(should_skip("specify", dir.path(), false));
        assert!(!should_skip("specify", dir.path(), true)); // force overrides
    }

    #[test]
    fn should_skip_clarify_when_no_markers() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("spec.md"), "# Spec\nNo markers here.").unwrap();
        assert!(should_skip("clarify", dir.path(), false));
    }

    #[test]
    fn should_not_skip_clarify_when_markers_present() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("spec.md"),
            "# Spec\n[NEEDS CLARIFICATION: something]",
        )
        .unwrap();
        assert!(!should_skip("clarify", dir.path(), false));
    }

    #[test]
    fn should_skip_implement_when_all_tasks_done() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("tasks.md"),
            "- [x] T001 Done\n- [x] T002 Done\n",
        )
        .unwrap();
        assert!(should_skip("implement", dir.path(), false));
    }

    #[test]
    fn should_not_skip_implement_when_tasks_pending() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("tasks.md"),
            "- [x] T001 Done\n- [ ] T002 Pending\n",
        )
        .unwrap();
        assert!(!should_skip("implement", dir.path(), false));
    }

    #[test]
    fn analyze_never_skipped() {
        let dir = TempDir::new().unwrap();
        assert!(!should_skip("analyze", dir.path(), false));
    }

    #[test]
    fn phase_types_correct() {
        assert_eq!(phase_type("intent"), PhaseType::Auto);
        assert_eq!(phase_type("specify"), PhaseType::Auto);
        assert_eq!(phase_type("plan"), PhaseType::Auto);
        assert_eq!(phase_type("implement"), PhaseType::Handoff);
        assert_eq!(phase_type("evidence"), PhaseType::Auto);
        assert_eq!(phase_type("analyze"), PhaseType::Auto);
        assert_eq!(phase_type("ship"), PhaseType::Auto);
    }

    #[test]
    fn should_skip_ship_when_report_exists() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("ship-report.md"), "<!-- ship: true -->").unwrap();
        assert!(should_skip("ship", dir.path(), false));
        assert!(!should_skip("ship", dir.path(), true)); // force overrides
    }

    #[test]
    fn should_not_skip_ship_when_report_absent() {
        let dir = TempDir::new().unwrap();
        assert!(!should_skip("ship", dir.path(), false));
    }

    #[test]
    fn should_skip_evidence_when_report_exists() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("evidence-report.md"), "# Evidence").unwrap();
        assert!(should_skip("evidence", dir.path(), false));
        assert!(!should_skip("evidence", dir.path(), true));
    }

    #[test]
    fn should_not_skip_evidence_when_absent() {
        let dir = TempDir::new().unwrap();
        assert!(!should_skip("evidence", dir.path(), false));
    }

    #[test]
    fn should_skip_intent_when_intent_exists() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("intent.md"), "# Intent: Test").unwrap();
        assert!(should_skip("intent", dir.path(), false));
        assert!(!should_skip("intent", dir.path(), true)); // force overrides
    }

    #[test]
    fn should_not_skip_intent_when_absent() {
        let dir = TempDir::new().unwrap();
        assert!(!should_skip("intent", dir.path(), false));
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
}
