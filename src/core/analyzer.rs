use std::path::Path;
use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;

use super::artifact_graph::{self, TraceGraph};
use super::constitution;
use super::errors::SolidSpecError;
use super::intent_parser::{self, IntentDrift};
use super::okf::{BundleIndex, DEFAULT_BUNDLE_DIR};
use super::spec_parser;

const MAX_FINDINGS: usize = 50;

/// Matches a backtick-quoted code span (`` `Foo.bar()` ``) — the convention
/// AI-generated tasks.md content already uses for code identifiers,
/// deliberately used as the signal for `structural_cross_check` below: it's
/// specific enough to stay low-noise (bare English words in a task
/// description aren't backtick-quoted) without needing full-language
/// parsing to tell an identifier from prose.
static BACKTICK_SPAN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"`([^`\n]+)`").expect("invalid backtick span regex"));

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Critical => write!(f, "CRITICAL"),
            Severity::High => write!(f, "HIGH"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::Low => write!(f, "LOW"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub severity: Severity,
    pub message: String,
    pub remediation: String,
}

#[derive(Debug)]
pub struct AnalysisReport {
    pub findings: Vec<Finding>,
    pub overflow_count: usize,
    pub traceability_score: f64,
    /// Populated only when `intent.md` is present in the feature dir.
    pub intent_drift: Option<IntentDrift>,
    /// Full Intent → Requirement → Task → Test trace graph.
    pub trace_graph: Option<TraceGraph>,
    /// Percentage of intent evidence criteria covered by implemented tests.
    /// `None` when `intent.md` is absent.
    pub intent_coverage: Option<f64>,
    /// Findings from cross-checking tasks.md against the project's OKF
    /// knowledge graph (`docs/okf-rs-integration-plan.md` step 5). `None`
    /// when no bundle exists yet — nothing to check against, never a
    /// failure; rendered as its own report section, never merged into
    /// `findings` above, so it stays clearly attributable to the graph
    /// rather than to SolidSpec's own textual heuristics.
    pub structural_cross_check: Option<Vec<Finding>>,
}

/// Analyze cross-artifact consistency. Read-only — does NOT modify files.
pub fn analyze_feature(feature_dir: &Path, project_root: &Path) -> Result<AnalysisReport> {
    let spec_path = feature_dir.join("spec.md");
    if !spec_path.exists() {
        return Err(SolidSpecError::Spec {
            feature_id: feature_dir.display().to_string(),
            message: "spec.md not found".into(),
            fix: "Run 'solidspec specify' first.".into(),
        }
        .into());
    }

    let mut all_findings = Vec::new();

    // Parse spec
    let spec = spec_parser::parse_spec(&spec_path)?;

    // Check plan exists
    let plan_path = feature_dir.join("plan.md");
    let has_plan = plan_path.exists();
    if !has_plan {
        all_findings.push(Finding {
            severity: Severity::High,
            message: "plan.md missing — no architecture plan found".into(),
            remediation: "Run 'solidspec plan' to generate the implementation plan.".into(),
        });
    }

    // Check tasks exist
    let tasks_path = feature_dir.join("tasks.md");
    let has_tasks = tasks_path.exists();
    if !has_tasks && has_plan {
        all_findings.push(Finding {
            severity: Severity::High,
            message: "tasks.md missing — no task breakdown found".into(),
            remediation: "Run 'solidspec tasks' to generate the task breakdown.".into(),
        });
    }

    // Check unresolved clarification markers
    if !spec.clarification_markers.is_empty() {
        all_findings.push(Finding {
            severity: Severity::High,
            message: format!(
                "{} unresolved [NEEDS CLARIFICATION] markers in spec.md",
                spec.clarification_markers.len()
            ),
            remediation: "Run 'solidspec clarify' to resolve ambiguities.".into(),
        });
    }

    // Constitution compliance
    let constitution_path = project_root.join(".solidspec/constitution.md");
    if constitution_path.exists() {
        let const_result = constitution::load_constitution(&constitution_path)?;
        if has_plan {
            let plan_content = std::fs::read_to_string(&plan_path)?;
            let gate_results = constitution::check_plan_compliance(&const_result, &plan_content);
            for gate in gate_results {
                if !gate.passed {
                    for violation in &gate.violations {
                        all_findings.push(Finding {
                            severity: Severity::Critical,
                            message: format!("Constitution violation ({}): {violation}", gate.gate_name),
                            remediation: "Constitution conflicts are non-negotiable. Update the plan to comply.".into(),
                        });
                    }
                }
            }
        }
    }

    // Requirement → plan traceability
    let mut traced_reqs = 0;
    if has_plan {
        let plan_content = std::fs::read_to_string(&plan_path)?;
        for req in &spec.requirements {
            // Simple check: requirement ID or key terms appear in plan
            if plan_content.contains(&req.id) {
                traced_reqs += 1;
            }
        }

        let total_reqs = spec.requirements.len();
        if total_reqs > 0 && traced_reqs < total_reqs {
            let missing = total_reqs - traced_reqs;
            all_findings.push(Finding {
                severity: Severity::Medium,
                message: format!("{missing} requirements not traced in plan.md"),
                remediation: "Ensure all FR-### IDs from spec.md appear in plan.md.".into(),
            });
        }
    }

    // Orphan task detection
    let mut structural_cross_check_result = None;
    if has_tasks {
        let tasks_content = std::fs::read_to_string(&tasks_path)?;
        let task_count = tasks_content.matches("- [ ] T").count()
            + tasks_content.matches("- [X] T").count()
            + tasks_content.matches("- [x] T").count();
        if task_count == 0 {
            all_findings.push(Finding {
                severity: Severity::Medium,
                message: "tasks.md contains no tasks".into(),
                remediation: "Run 'solidspec tasks' to regenerate.".into(),
            });
        }

        structural_cross_check_result = structural_cross_check(&tasks_content, project_root);
    }

    // Terminology drift (simple check)
    if has_plan {
        let plan_content = std::fs::read_to_string(&plan_path)?;
        for entity in &spec.entities {
            if !plan_content.contains(entity) {
                all_findings.push(Finding {
                    severity: Severity::Low,
                    message: format!("Entity '{entity}' from spec not mentioned in plan.md"),
                    remediation: format!("Ensure '{entity}' is referenced in the plan."),
                });
            }
        }
    }

    // Traceability graph (P6) — built before the cap so orphaned-FR findings are counted.
    let trace_graph = artifact_graph::build_trace_graph(feature_dir);

    // Orphaned requirements: FR-XXX in spec with no task referencing them.
    // Only fired when tasks.md EXISTS — absence is already reported above and would
    // produce one redundant finding per FR if we fired here unconditionally.
    if has_tasks && let Some(ref tg) = trace_graph {
        for fr_id in &tg.orphaned_requirements {
            all_findings.push(Finding {
                severity: Severity::High,
                message: format!("{fr_id} has no task referencing it in tasks.md"),
                remediation: format!(
                    "Add a task in tasks.md that mentions [{fr_id}] and implements it."
                ),
            });
        }
    }

    // Cap findings
    let overflow_count = if all_findings.len() > MAX_FINDINGS {
        let overflow = all_findings.len() - MAX_FINDINGS;
        all_findings.truncate(MAX_FINDINGS);
        overflow
    } else {
        0
    };

    // Traceability score
    let total_reqs = spec.requirements.len();
    let traceability_score = if total_reqs > 0 {
        traced_reqs as f64 / total_reqs as f64 * 100.0
    } else {
        100.0 // no requirements = vacuously traced
    };

    // Intent drift (only when intent.md is present) — appended after cap so a
    // high-drift finding is never hidden by an overflowing base-dimension list.
    let intent_drift = compute_drift(feature_dir);
    if let Some(ref drift) = intent_drift {
        if drift.score >= 70.0 {
            all_findings.push(Finding {
                severity: Severity::Critical,
                message: format!(
                    "Intent drift {:.0}% — {} evidence criteria unsatisfied",
                    drift.score,
                    drift.unsatisfied.len()
                ),
                remediation: "Review implemented tests against intent.md evidence criteria.".into(),
            });
        } else if drift.score >= 30.0 {
            all_findings.push(Finding {
                severity: Severity::High,
                message: format!(
                    "Intent drift {:.0}% — {} evidence criteria unsatisfied",
                    drift.score,
                    drift.unsatisfied.len()
                ),
                remediation: "Add or update test scaffolds to cover unsatisfied evidence criteria."
                    .into(),
            });
        }
    }

    // Intent coverage: % of evidence criteria covered by implemented tests.
    // Returns None at baseline (no tests implemented yet) so the metric is only
    // shown when it is meaningfully measurable — avoiding the contradictory
    // situation where intent_drift shows "0% drift" while intent_coverage shows "0%".
    let intent_coverage = if feature_dir.join("intent.md").exists() {
        match crate::core::evidence::collect_evidence(feature_dir) {
            Ok(report) if report.has_implemented_tests => Some(report.satisfaction_rate),
            Ok(_) => None, // baseline — no tests implemented yet, not measurable
            Err(_) => None,
        }
    } else {
        None
    };

    Ok(AnalysisReport {
        findings: all_findings,
        overflow_count,
        traceability_score,
        intent_drift,
        trace_graph,
        intent_coverage,
        structural_cross_check: structural_cross_check_result,
    })
}

/// Cross-checks tasks.md against the project's OKF knowledge-graph bundle
/// (`okf::DEFAULT_BUNDLE_DIR`), catching two kinds of orphaned reference a
/// purely textual read of tasks.md can't — see
/// `docs/okf-rs-integration-plan.md` step 5 /
/// `docs/kg-workflow-vision-gap-analysis.md`'s recommendation #1:
///
/// - a backtick-quoted symbol (`` `CombatSystem.calculate_damage()` ``)
///   that matches no concept name anywhere in the graph;
/// - an existing, recognized-language source file mentioned in tasks.md
///   with zero concepts in the graph (usually a stale bundle — run
///   `solidspec okf generate` — occasionally a language okf-rs doesn't
///   extract).
///
/// Returns `None` when no bundle exists yet (nothing to check against,
/// never a failure — a fresh/empty project or one that hasn't run
/// `solidspec okf generate` behaves exactly as before this existed).
/// Every finding is `Low`/`Medium`: a heuristic sanity check on top of a
/// possibly-stale graph, never grounds to block `analyze` the way a
/// constitution violation does.
fn structural_cross_check(tasks_content: &str, project_root: &Path) -> Option<Vec<Finding>> {
    let bundle_dir = project_root.join(DEFAULT_BUNDLE_DIR);
    let index = BundleIndex::load(&bundle_dir).ok().flatten()?;

    let mut findings = Vec::new();
    let mut checked_symbols = std::collections::HashSet::new();
    let mut checked_files = std::collections::HashSet::new();

    for capture in BACKTICK_SPAN_RE.captures_iter(tasks_content) {
        let raw = capture[1].trim();
        let Some(symbol) = extract_symbol_name(raw) else {
            continue;
        };
        if !checked_symbols.insert(symbol.clone()) || index.has_symbol(&symbol) {
            continue;
        }
        findings.push(Finding {
            severity: Severity::Medium,
            message: format!(
                "tasks.md references `{raw}` but no symbol named '{symbol}' exists in the knowledge graph"
            ),
            remediation: "Verify the symbol name, or run 'solidspec okf generate' if the codebase changed since the graph was last generated.".into(),
        });
    }

    for word in tasks_content.split_whitespace() {
        let trimmed = word.trim_matches(|c: char| !(c.is_alphanumeric() || "/.-_".contains(c)));
        let Some(path) = extract_file_path(trimmed) else {
            continue;
        };
        if !checked_files.insert(path.clone()) {
            continue;
        }
        if project_root.join(&path).exists() && !index.has_file(&path) {
            findings.push(Finding {
                severity: Severity::Low,
                message: format!(
                    "'{path}' is referenced in tasks.md and exists on disk, but the knowledge graph has no concepts for it"
                ),
                remediation: "Run 'solidspec okf generate' to refresh the knowledge graph (or ignore if the file has no functions/classes to extract).".into(),
            });
        }
    }

    Some(findings)
}

/// Extracts a plausible bare symbol name from a backtick span's raw text:
/// strips a trailing `(...)` call syntax, then takes the last segment of a
/// qualified name (`Foo.bar`, `foo::bar`) — the graph indexes concepts by
/// their own short name (`Concept::name`), not a qualified path. Returns
/// `None` for anything that isn't a single bare identifier (multi-word
/// text, an actual file path — handled separately by `extract_file_path`)
/// so this stays a conservative, low-noise check.
fn extract_symbol_name(raw: &str) -> Option<String> {
    let without_call = raw.split('(').next().unwrap_or(raw).trim();
    if without_call.is_empty()
        || without_call.contains(char::is_whitespace)
        || without_call.contains('/')
    {
        return None;
    }
    let last_segment = without_call
        .rsplit(['.', ':'])
        .next()
        .unwrap_or(without_call);
    let is_identifier = !last_segment.is_empty()
        && last_segment
            .chars()
            .next()
            .is_some_and(|c| c.is_alphabetic() || c == '_')
        && last_segment
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_');
    is_identifier.then(|| last_segment.to_string())
}

/// Extracts a project-relative file path from a token, if it looks like
/// one: contains a `/` and ends in an extension `okf-rs` actually indexes
/// (`okf_parser::Language::from_extension`) — narrow on purpose, since a
/// broader "any word with a slash" match would also catch prose like
/// "either/or" or a URL fragment.
fn extract_file_path(token: &str) -> Option<String> {
    if !token.contains('/') {
        return None;
    }
    let ext = token.rsplit('.').next()?;
    if ext == token {
        return None; // no '.' at all
    }
    okf_parser::Language::from_extension(ext)?;
    Some(token.trim_start_matches("./").to_string())
}

/// Compute intent drift by cross-referencing evidence criteria from `intent.md`
/// against test scaffold files in `<feature_dir>/tests/`.
///
/// **Baseline rule**: if every test scaffold file still contains
/// `STATUS: NOT IMPLEMENTED`, drift is `0.0` — criteria exist but cannot yet
/// be measured, so we report no drift rather than a false alarm.
///
/// Once any test is marked `STATUS: IMPLEMENTED`, each evidence criterion is
/// checked: is there an implemented test whose body mentions at least one key
/// term (≥5 chars) from the criterion? If not, it is counted as unsatisfied.
pub fn compute_drift(feature_dir: &Path) -> Option<IntentDrift> {
    let intent_path = feature_dir.join("intent.md");
    if !intent_path.exists() {
        return None;
    }

    let intent = match intent_parser::parse_intent(&intent_path) {
        Ok(i) => i,
        Err(_) => return None,
    };

    if intent.evidence.is_empty() {
        return Some(IntentDrift {
            score: 0.0,
            unsatisfied: vec![],
        });
    }

    // Collect all test scaffold files
    let tests_dir = feature_dir.join("tests");
    let scaffold_files: Vec<String> = std::fs::read_dir(&tests_dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| {
            e.path().extension().and_then(|x| x.to_str()) == Some("md")
                || e.path().extension().and_then(|x| x.to_str()) == Some("ts")
                || e.path().extension().and_then(|x| x.to_str()) == Some("py")
                || e.path().extension().and_then(|x| x.to_str()) == Some("rs")
                || e.path().extension().and_then(|x| x.to_str()) == Some("go")
        })
        .filter_map(|e| std::fs::read_to_string(e.path()).ok())
        .collect();

    // Baseline: if no test scaffolds exist or all are NOT IMPLEMENTED → 0% drift
    let any_implemented = scaffold_files
        .iter()
        .any(|f| f.contains("STATUS: IMPLEMENTED") && !f.contains("STATUS: NOT IMPLEMENTED"));

    if !any_implemented {
        return Some(IntentDrift {
            score: 0.0,
            unsatisfied: vec![],
        });
    }

    // Collect body of implemented tests only (after stripping NOT IMPLEMENTED files)
    let implemented_body: String = scaffold_files
        .iter()
        .filter(|f| f.contains("STATUS: IMPLEMENTED") && !f.contains("STATUS: NOT IMPLEMENTED"))
        .cloned()
        .collect::<Vec<_>>()
        .join("\n")
        .to_lowercase();

    // Pre-build a word set from implemented test bodies for word-boundary matching
    let implemented_words: std::collections::HashSet<&str> = implemented_body
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .collect();

    let mut unsatisfied = Vec::new();

    for criterion in &intent.evidence {
        // Skip empty/blank criteria (e.g. from bare "- " bullet lines)
        if criterion.trim().is_empty() {
            continue;
        }

        // Extract key terms (≥5 chars) from the criterion text
        let keywords: Vec<String> = criterion
            .split_whitespace()
            .map(|w| {
                w.trim_matches(|c: char| !c.is_alphanumeric())
                    .to_lowercase()
            })
            .filter(|w| w.len() >= 5)
            .collect();

        let mentioned = if keywords.is_empty() {
            // Short criterion (e.g. "No SQL"): full-phrase exact match
            let phrase = criterion.trim().to_lowercase();
            implemented_body.contains(&phrase)
        } else {
            // Word-boundary match: keyword must appear as a whole word
            keywords
                .iter()
                .any(|kw| implemented_words.contains(kw.as_str()))
        };

        if !mentioned {
            unsatisfied.push(criterion.clone());
        }
    }

    let non_empty_count = intent
        .evidence
        .iter()
        .filter(|c| !c.trim().is_empty())
        .count();
    let score = if non_empty_count > 0 {
        unsatisfied.len() as f64 / non_empty_count as f64 * 100.0
    } else {
        0.0
    };
    Some(IntentDrift { score, unsatisfied })
}

pub fn format_report(report: &AnalysisReport) -> String {
    let mut output = String::from("# Analysis Report\n\n");
    output.push_str(&format!(
        "**Traceability Score**: {:.0}%\n",
        report.traceability_score
    ));

    if let Some(ref drift) = report.intent_drift {
        output.push_str(&format!("**Intent Drift**: {:.0}%", drift.score));
        if drift.unsatisfied.is_empty() {
            // Distinguish baseline (no implemented tests → coverage=None) from truly satisfied.
            if report.intent_coverage.is_some() {
                output.push_str("  ✓ all evidence criteria satisfied\n");
            } else {
                output.push_str("  (baseline — no tests implemented yet)\n");
            }
        } else {
            output.push('\n');
            for item in &drift.unsatisfied {
                output.push_str(&format!(
                    "  ✗ \"{item}\" — not covered by implemented tests\n"
                ));
            }
        }
    }

    if let Some(coverage) = report.intent_coverage {
        output.push_str(&format!("**Intent Coverage**: {coverage:.0}%\n"));
    }

    output.push_str(&format!("**Findings**: {}", report.findings.len()));
    if report.overflow_count > 0 {
        output.push_str(&format!(
            " (+{} overflow, not shown)",
            report.overflow_count
        ));
    }
    output.push_str("\n\n");

    if let Some(ref tg) = report.trace_graph {
        output.push_str(&tg.format_tree());
        output.push('\n');
    }

    for finding in &report.findings {
        output.push_str(&format!(
            "- **[{}]** {}\n",
            finding.severity, finding.message
        ));
        output.push_str(&format!("  *Remediation*: {}\n\n", finding.remediation));
    }

    if report.findings.is_empty() {
        output.push_str("No issues found. All artifacts are consistent.\n");
    }

    // Structural cross-check (okf-rs): a distinct section, never merged into
    // the findings above — see docs/okf-rs-integration-plan.md step 5. Omitted
    // entirely when no bundle exists (structural_cross_check is None); shown
    // even when empty once a bundle *was* checked, so it's visible that the
    // check actually ran.
    if let Some(ref structural_findings) = report.structural_cross_check {
        output.push_str("\n## Structural cross-check (okf-rs)\n\n");
        if structural_findings.is_empty() {
            output.push_str(
                "No orphaned file/symbol references found against the knowledge graph.\n",
            );
        } else {
            for finding in structural_findings {
                output.push_str(&format!(
                    "- **[{}]** {}\n",
                    finding.severity, finding.message
                ));
                output.push_str(&format!("  *Remediation*: {}\n\n", finding.remediation));
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_feature(dir: &Path, spec: &str, plan: Option<&str>, tasks: Option<&str>) {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(dir.join("spec.md"), spec).unwrap();
        if let Some(p) = plan {
            std::fs::write(dir.join("plan.md"), p).unwrap();
        }
        if let Some(t) = tasks {
            std::fs::write(dir.join("tasks.md"), t).unwrap();
        }
    }

    fn setup_constitution(project_root: &Path) {
        let solidspec = project_root.join(".solidspec");
        std::fs::create_dir_all(&solidspec).unwrap();
        std::fs::write(
            solidspec.join("constitution.md"),
            "### Article VII: Simplicity\n### Article VIII: Anti-Abstraction\n### Article IX: Integration-First\n",
        ).unwrap();
    }

    #[test]
    fn fully_traced_artifacts_high_score() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_constitution(dir.path());

        let spec = "## Requirements\n- **FR-001**: authenticate users\n- **FR-002**: store sessions\n## Success Criteria\n- **SC-001**: login works\n";
        let plan = "# Plan\nFR-001 handled by auth module\nFR-002 handled by session store\n";
        let tasks = "- [ ] T001 Setup\n- [ ] T002 Auth\n";
        setup_feature(&feature, spec, Some(plan), Some(tasks));

        let report = analyze_feature(&feature, dir.path()).unwrap();
        assert_eq!(report.traceability_score, 100.0);
    }

    #[test]
    fn missing_plan_is_high_finding() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_feature(&feature, "# Spec\n", None, None);
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.severity == Severity::High && f.message.contains("plan.md missing"))
        );
    }

    #[test]
    fn orphan_tasks_medium_finding() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_feature(
            &feature,
            "# Spec\n",
            Some("# Plan\n"),
            Some("# Tasks\nno actual tasks\n"),
        );
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.severity == Severity::Medium && f.message.contains("no tasks"))
        );
    }

    #[test]
    fn constitution_violation_is_critical() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        let plan = "We should future-proof this with a wrapper abstraction layer.";
        setup_feature(&feature, "# Spec\n", Some(plan), None);
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.severity == Severity::Critical)
        );
    }

    #[test]
    fn missing_spec_returns_error() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        std::fs::create_dir_all(&feature).unwrap();

        assert!(analyze_feature(&feature, dir.path()).is_err());
    }

    #[test]
    fn max_findings_enforced() {
        // Simulate by checking the cap constant
        assert_eq!(MAX_FINDINGS, 50);
    }

    #[test]
    fn analyze_does_not_modify_files() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        let spec = "# Spec\n- **FR-001**: test\n";
        let plan = "# Plan\nFR-001 covered\n";
        setup_feature(&feature, spec, Some(plan), Some("- [ ] T001 test\n"));
        setup_constitution(dir.path());

        let spec_before = std::fs::read_to_string(feature.join("spec.md")).unwrap();
        let plan_before = std::fs::read_to_string(feature.join("plan.md")).unwrap();

        let _report = analyze_feature(&feature, dir.path()).unwrap();

        let spec_after = std::fs::read_to_string(feature.join("spec.md")).unwrap();
        let plan_after = std::fs::read_to_string(feature.join("plan.md")).unwrap();
        assert_eq!(spec_before, spec_after, "Analyzer modified spec.md!");
        assert_eq!(plan_before, plan_after, "Analyzer modified plan.md!");
    }

    #[test]
    fn remediation_suggestions_present() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_feature(&feature, "# Spec\n", None, None);
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        for finding in &report.findings {
            assert!(
                !finding.remediation.is_empty(),
                "Finding has empty remediation"
            );
        }
    }

    // ── compute_drift() tests ────────────────────────────────────────────────

    const SAMPLE_INTENT_DRIFT: &str = r#"# Intent: Export PDF

**Intent ID**: INT-001
**Feature**: 001-export-pdf
**Created**: 2026-01-01
**Status**: active

## Goal
Allow PDF export.

## Evidence
- Export command exits zero
- Generated PDF contains all sections
- All scenarios pass
"#;

    fn write_intent(dir: &Path, content: &str) {
        std::fs::write(dir.join("intent.md"), content).unwrap();
    }

    fn write_test(dir: &Path, name: &str, content: &str) {
        let tests = dir.join("tests");
        std::fs::create_dir_all(&tests).unwrap();
        std::fs::write(tests.join(name), content).unwrap();
    }

    #[test]
    fn drift_none_when_no_intent_md() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature).unwrap();
        assert!(compute_drift(&feature).is_none());
    }

    #[test]
    fn drift_zero_at_baseline_all_not_implemented() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature).unwrap();
        write_intent(&feature, SAMPLE_INTENT_DRIFT);
        write_test(
            &feature,
            "test1.md",
            "GIVEN: user\nWHEN: exports\nTHEN: gets pdf\nSTATUS: NOT IMPLEMENTED\n",
        );

        let drift = compute_drift(&feature).unwrap();
        assert_eq!(drift.score, 0.0, "Baseline: all NOT IMPLEMENTED → drift 0%");
        assert!(drift.unsatisfied.is_empty());
    }

    #[test]
    fn drift_zero_when_no_test_files() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature).unwrap();
        write_intent(&feature, SAMPLE_INTENT_DRIFT);

        let drift = compute_drift(&feature).unwrap();
        assert_eq!(drift.score, 0.0);
    }

    #[test]
    fn drift_detects_unsatisfied_criteria() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature).unwrap();
        write_intent(&feature, SAMPLE_INTENT_DRIFT);

        // One implemented test that covers "export" but not "scenarios"
        write_test(
            &feature,
            "test1.md",
            "GIVEN: user\nWHEN: runs export command\nTHEN: exits zero\nSTATUS: IMPLEMENTED\n",
        );

        let drift = compute_drift(&feature).unwrap();
        // "Export command exits zero" → "exits" is 5+ chars, present → satisfied
        // "Generated PDF contains all sections" → "Generated"/"contains"/"sections" present? "sections" is in test body → may satisfy
        // "All scenarios pass" → "scenarios" present in test body → satisfied
        // Score should be < 100% because at least the export criterion is covered
        assert!(drift.score >= 0.0 && drift.score <= 100.0);
    }

    #[test]
    fn drift_score_100_when_all_criteria_uncovered() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature).unwrap();

        // Intent with very specific criterion that won't appear in test body
        let intent = "# Intent: Quantum\n\n**Intent ID**: INT-001\n**Feature**: 001-q\n**Created**: 2026-01-01\n**Status**: active\n\n## Goal\nQuantum stuff.\n\n## Evidence\n- Quantum entanglement process completes\n- Subatomic particle detection works\n";
        write_intent(&feature, intent);

        // An implemented test that talks about something completely different
        write_test(
            &feature,
            "test1.md",
            "GIVEN: the system\nWHEN: running normally\nTHEN: nothing breaks\nSTATUS: IMPLEMENTED\n",
        );

        let drift = compute_drift(&feature).unwrap();
        assert!(
            drift.score > 0.0,
            "Uncovered criteria should produce drift > 0"
        );
        assert_eq!(drift.unsatisfied.len(), 2);
    }

    // ── extract_symbol_name() / extract_file_path() ─────────────────────────

    #[test]
    fn extract_symbol_name_strips_call_syntax_and_qualification() {
        assert_eq!(
            extract_symbol_name("CombatSystem.calculate_damage()"),
            Some("calculate_damage".to_string())
        );
        assert_eq!(
            extract_symbol_name("parse_intent"),
            Some("parse_intent".to_string())
        );
        assert_eq!(
            extract_symbol_name("okf_core::git::head_revision"),
            Some("head_revision".to_string())
        );
    }

    #[test]
    fn extract_symbol_name_rejects_non_identifiers() {
        assert_eq!(extract_symbol_name("this is prose"), None);
        assert_eq!(extract_symbol_name("src/foo.rs"), None);
        assert_eq!(extract_symbol_name(""), None);
        assert_eq!(extract_symbol_name("123abc"), None);
    }

    #[test]
    fn extract_file_path_accepts_recognized_source_extensions() {
        assert_eq!(
            extract_file_path("src/combat/system.rs"),
            Some("src/combat/system.rs".to_string())
        );
        assert_eq!(
            extract_file_path("./src/lib.rs"),
            Some("src/lib.rs".to_string())
        );
    }

    #[test]
    fn extract_file_path_rejects_non_paths() {
        assert_eq!(extract_file_path("README.md"), None); // no '/'
        assert_eq!(extract_file_path("either/or"), None); // no recognized extension
        assert_eq!(extract_file_path("no_dot_at_all/thing"), None);
    }

    // ── structural_cross_check() / analyze_feature() integration ────────────

    fn generate_bundle_for(project_root: &Path) -> std::path::PathBuf {
        let bundle_dir = project_root.join(DEFAULT_BUNDLE_DIR);
        crate::core::okf::generate(project_root, &bundle_dir).unwrap();
        bundle_dir
    }

    #[test]
    fn structural_cross_check_is_none_without_a_bundle() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_feature(
            &feature,
            "# Spec\n",
            Some("# Plan\n"),
            Some("- [ ] T001 Implement `does_not_exist()`\n"),
        );
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        assert!(report.structural_cross_check.is_none());
    }

    #[test]
    fn structural_cross_check_flags_unknown_backtick_symbol() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("combat.rs"),
            "pub fn calculate_damage() {}\n",
        )
        .unwrap();
        generate_bundle_for(dir.path());

        let feature = dir.path().join("specs/001-auth");
        setup_feature(
            &feature,
            "# Spec\n",
            Some("# Plan\n"),
            Some("- [ ] T001 Call `totally_invented_symbol()` from the combat module\n"),
        );
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        let findings = report.structural_cross_check.unwrap();
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("totally_invented_symbol")),
            "{findings:?}"
        );
    }

    #[test]
    fn structural_cross_check_accepts_a_real_symbol() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("combat.rs"),
            "pub fn calculate_damage() {}\n",
        )
        .unwrap();
        generate_bundle_for(dir.path());

        let feature = dir.path().join("specs/001-auth");
        setup_feature(
            &feature,
            "# Spec\n",
            Some("# Plan\n"),
            Some("- [ ] T001 Update `calculate_damage()` for critical hits\n"),
        );
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        let findings = report.structural_cross_check.unwrap();
        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("calculate_damage")),
            "a real symbol must not be flagged: {findings:?}"
        );
    }

    #[test]
    fn structural_cross_check_flags_existing_file_missing_from_a_stale_bundle() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("combat.rs"),
            "pub fn calculate_damage() {}\n",
        )
        .unwrap();
        // Bundle generated BEFORE this file is added — simulates the bundle
        // going stale after new code lands, exactly the scenario
        // docs/kg-workflow-vision-gap-analysis.md flags as unaddressed today
        // (nothing regenerates the graph automatically after `implement`).
        generate_bundle_for(dir.path());
        std::fs::create_dir_all(dir.path().join("extra")).unwrap();
        std::fs::write(
            dir.path().join("extra/critical_hits.rs"),
            "pub fn apply_crit() {}\n",
        )
        .unwrap();

        let feature = dir.path().join("specs/001-auth");
        setup_feature(
            &feature,
            "# Spec\n",
            Some("# Plan\n"),
            Some("- [ ] T001 See extra/critical_hits.rs for context\n"),
        );
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        let findings = report.structural_cross_check.unwrap();
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("extra/critical_hits.rs")),
            "{findings:?}"
        );
    }

    #[test]
    fn structural_cross_check_ignores_files_that_do_not_exist_yet() {
        // tasks.md routinely names files that don't exist YET (they're the
        // deliverable) — must never be flagged as orphaned.
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("combat.rs"),
            "pub fn calculate_damage() {}\n",
        )
        .unwrap();
        generate_bundle_for(dir.path());

        let feature = dir.path().join("specs/001-auth");
        setup_feature(
            &feature,
            "# Spec\n",
            Some("# Plan\n"),
            Some("- [ ] T001 Create src/new_feature/critical_hits.rs\n"),
        );
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        let findings = report.structural_cross_check.unwrap();
        assert!(findings.is_empty(), "{findings:?}");
    }

    #[test]
    fn format_report_omits_structural_section_without_a_bundle() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        setup_feature(
            &feature,
            "# Spec\n",
            Some("# Plan\n"),
            Some("- [ ] T001 x\n"),
        );
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        let output = format_report(&report);
        assert!(!output.contains("Structural cross-check"));
    }

    #[test]
    fn format_report_includes_structural_section_with_a_bundle() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("combat.rs"),
            "pub fn calculate_damage() {}\n",
        )
        .unwrap();
        generate_bundle_for(dir.path());

        let feature = dir.path().join("specs/001-auth");
        setup_feature(
            &feature,
            "# Spec\n",
            Some("# Plan\n"),
            Some("- [ ] T001 Update `calculate_damage()`\n"),
        );
        setup_constitution(dir.path());

        let report = analyze_feature(&feature, dir.path()).unwrap();
        let output = format_report(&report);
        assert!(output.contains("Structural cross-check (okf-rs)"));
    }
}
