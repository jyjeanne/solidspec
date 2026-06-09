use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::time::Instant;

use anyhow::Result;
use regex::Regex;

use crate::config::FanOutConfig;
use crate::core::review::{self, Dimension, Severity};

// ── Regex statics ────────────────────────────────────────────────────────────

/// Matches `SCORE: N` in agent output (1–3 digits).
static SCORE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"SCORE:\s*(\d{1,3})").expect("invalid SCORE_RE"));

/// Matches `SEVERITY: LEVEL` in structured agent output.
static SEVERITY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"SEVERITY:\s*(CRITICAL|HIGH|MEDIUM|LOW)").expect("invalid SEVERITY_RE")
});

// ── Core types ───────────────────────────────────────────────────────────────

/// One specialized review cluster used by `solidspec ship`.
#[derive(Clone)]
pub struct ReviewLane {
    /// Short machine-readable identifier: "code" | "security" | "tests" | "perf"
    pub id: &'static str,
    /// Human-readable label shown in terminal output.
    pub label: &'static str,
    /// Agent ID to invoke for this lane (e.g. "claude", "gemini").
    pub agent_id: String,
    /// Fully-assembled deep-focus prompt supplied to the agent.
    pub prompt: String,
    /// Score threshold 0–100. A lane score below this value → HOLD.
    pub threshold: u8,
}

/// Result produced by a single review lane after the agent (or heuristic) completes.
pub struct LaneResult {
    pub lane_id: &'static str,
    pub lane_label: &'static str,
    pub agent_id: String,
    /// Score 0–100 derived from `SCORE: N` in agent output or from heuristic penalties.
    pub score: u8,
    pub findings: Vec<FanOutFinding>,
    /// Wall-clock time from lane start to result, in milliseconds.
    pub duration_ms: u64,
    pub status: LaneStatus,
    /// Pass/fail threshold for this lane (copied from `ReviewLane.threshold`).
    pub threshold: u8,
}

#[derive(PartialEq, Eq, Debug)]
pub enum LaneStatus {
    Done,
    TimedOut,
    /// Agent process exited non-zero or the invoker returned an error.
    Failed(String),
}

/// Aggregated ship decision produced after all lanes complete.
pub struct ShipReport {
    pub feature_id: String,
    pub decision: ShipDecision,
    /// All lane results (pass and fail).
    pub lanes: Vec<LaneResult>,
    /// Subset of findings that directly caused a HOLD decision.
    pub blocking_findings: Vec<FanOutFinding>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ShipDecision {
    Ship,
    Hold,
}

/// A single finding surfaced by a review lane.
///
/// Reuses `crate::core::review::Severity` to avoid duplicating the severity enum.
#[derive(Clone)]
pub struct FanOutFinding {
    /// Lane that produced this finding ("code" | "security" | "tests" | "perf").
    pub lane: &'static str,
    pub severity: Severity,
    pub message: String,
    pub remediation: String,
}

// ── Phase 2: heuristic lane scoring ─────────────────────────────────────────

/// Derive a lane score 0–100 and its findings from heuristic checks (no AI agent).
///
/// Calls [`review::preflight_review`] and filters its findings to the lane's
/// dimension cluster, then applies the standard penalty formula.
/// Returns `Err` when the heuristic cannot run (e.g. `spec.md` missing) so
/// the caller can surface [`LaneStatus::Failed`].
#[allow(dead_code)]
pub(crate) fn score_from_heuristics(
    lane: &ReviewLane,
    feature_dir: &Path,
    project_root: &Path,
) -> Result<(u8, Vec<FanOutFinding>)> {
    let report = review::preflight_review(feature_dir, project_root)?;
    Ok(lane_findings_from_report(lane.id, &report))
}

/// Execute a review lane using heuristic checks only — no AI agent is invoked.
///
/// Used by `solidspec ship --no-agent`. Scores are derived from
/// [`review::preflight_review`] findings filtered to the lane's dimension cluster,
/// never placeholder zeros — a clean spec scores 100, a flawed one is penalised.
pub fn run_lane_no_agent(lane: &ReviewLane, feature_dir: &Path, project_root: &Path) -> LaneResult {
    let start = Instant::now();

    let report = match review::preflight_review(feature_dir, project_root) {
        Ok(r) => r,
        Err(e) => {
            return LaneResult {
                lane_id: lane.id,
                lane_label: lane.label,
                agent_id: lane.agent_id.clone(),
                score: 0,
                findings: vec![],
                duration_ms: start.elapsed().as_millis() as u64,
                status: LaneStatus::Failed(format!("{e}")),
                threshold: lane.threshold,
            };
        }
    };

    let (score, findings) = lane_findings_from_report(lane.id, &report);

    LaneResult {
        lane_id: lane.id,
        lane_label: lane.label,
        agent_id: lane.agent_id.clone(),
        score,
        findings,
        duration_ms: start.elapsed().as_millis() as u64,
        status: LaneStatus::Done,
        threshold: lane.threshold,
    }
}

/// Filter a preflight report's findings to those belonging to `lane_id`'s
/// dimension cluster and compute a penalty-based 0–100 score.
fn lane_findings_from_report(
    lane_id: &'static str,
    report: &review::ReviewReport,
) -> (u8, Vec<FanOutFinding>) {
    let findings: Vec<FanOutFinding> = report
        .findings
        .iter()
        .filter(|f| lane_covers_dimension(lane_id, &f.dimension))
        .map(|f| FanOutFinding {
            lane: lane_id,
            severity: f.severity.clone(),
            message: f.message.clone(),
            remediation: f.remediation.clone(),
        })
        .collect();

    let score = apply_penalty_formula(&findings);
    (score, findings)
}

/// Returns true when `dim` belongs to `lane_id`'s review cluster.
///
/// Cluster mapping:
/// - code     → Completeness, Clarity, Consistency, Maintainability
/// - security → Security
/// - tests    → Testability
/// - perf     → Performance
///
/// `IntentAlignment` is excluded from all lanes (IDSD-specific; handled by analyze/review).
fn lane_covers_dimension(lane_id: &str, dim: &Dimension) -> bool {
    matches!(
        (lane_id, dim),
        ("code", Dimension::Completeness)
            | ("code", Dimension::Clarity)
            | ("code", Dimension::Consistency)
            | ("code", Dimension::Maintainability)
            | ("security", Dimension::Security)
            | ("tests", Dimension::Testability)
            | ("perf", Dimension::Performance)
    )
}

/// Apply the fan-out penalty formula to a set of lane findings.
///
/// Base score 100; deduct 10×CRITICAL, 5×HIGH, 2×MEDIUM, 0.5×LOW. Clamped to [0, 100].
fn apply_penalty_formula(findings: &[FanOutFinding]) -> u8 {
    let penalty: f64 = findings
        .iter()
        .map(|f| match f.severity {
            Severity::Critical => 10.0,
            Severity::High => 5.0,
            Severity::Medium => 2.0,
            Severity::Low => 0.5,
            Severity::Info => 0.0,
        })
        .sum();
    (100.0_f64 - penalty).max(0.0).round() as u8
}

// ── Phase 3: agent-based lane execution ─────────────────────────────────────

/// Build the 4 review lanes with specialised prompts and per-lane agent/threshold config.
///
/// `feature_dir` is used to extract the feature slug (e.g. `"001-task-manager"`) for
/// the prompt. `default_agent` is used when the `FanOutConfig` has no override for a lane.
pub fn build_lanes(
    config: &FanOutConfig,
    feature_dir: &Path,
    default_agent: &str,
) -> Vec<ReviewLane> {
    let feat = feature_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    vec![
        ReviewLane {
            id: "code",
            label: "Code Review",
            agent_id: config
                .code_agent
                .as_deref()
                .unwrap_or(default_agent)
                .to_string(),
            prompt: code_review_prompt(&feat),
            threshold: config.code_threshold,
        },
        ReviewLane {
            id: "security",
            label: "Security Audit",
            agent_id: config
                .security_agent
                .as_deref()
                .unwrap_or(default_agent)
                .to_string(),
            prompt: security_audit_prompt(&feat),
            threshold: config.security_threshold,
        },
        ReviewLane {
            id: "tests",
            label: "Test Coverage",
            agent_id: config
                .tests_agent
                .as_deref()
                .unwrap_or(default_agent)
                .to_string(),
            prompt: test_coverage_prompt(&feat),
            threshold: config.tests_threshold,
        },
        ReviewLane {
            id: "perf",
            label: "Performance",
            agent_id: config
                .perf_agent
                .as_deref()
                .unwrap_or(default_agent)
                .to_string(),
            prompt: performance_prompt(&feat),
            threshold: config.perf_threshold,
        },
    ]
}

fn code_review_prompt(feat: &str) -> String {
    format!(
        "Read the project context from .solidspec/AGENT.md.\n\
         Feature: {feat} — find specs/{feat}/\n\n\
         You are performing a CODE REVIEW. Focus ONLY on:\n\
         - Requirement completeness: every FR-### in spec.md is addressed in plan.md and tasks.md\n\
         - Clarity: no placeholder text remains in any artifact\n\
         - Consistency: entity names, FR-IDs, US-labels are consistent across spec/plan/tasks\n\
         - Maintainability: plan decisions are justified; tasks are each independently deliverable\n\n\
         DO NOT assess security, performance, or test coverage.\n\n\
         For each issue found, state:\n\
           SEVERITY: CRITICAL | HIGH | MEDIUM | LOW\n\
           LOCATION: <file and section>\n\
           PROBLEM: <what is wrong>\n\
           FIX: <what to change>\n\n\
         Score the feature 0-100 on code quality.\n\
         Deduct: 10 per CRITICAL, 5 per HIGH, 2 per MEDIUM, 0.5 per LOW.\n\
         End your response with exactly: SCORE: N"
    )
}

fn security_audit_prompt(feat: &str) -> String {
    format!(
        "Read the project context from .solidspec/AGENT.md.\n\
         Feature: {feat} — find specs/{feat}/\n\n\
         You are performing a SECURITY AUDIT. Focus ONLY on:\n\
         - Authentication and authorization: are constraints explicit in spec and plan?\n\
         - PII handling: identified entities with personal data have a stated storage/transmission policy\n\
         - API contract security: no internal IDs exposed, no sensitive data in URLs\n\
         - OWASP Top 10 coverage for user-facing requirements\n\
         - Hardcoded credentials or secrets in any artifact\n\
         - Rate limiting for unauthenticated endpoints\n\n\
         DO NOT assess code quality, test coverage, or performance.\n\n\
         For each issue found, state:\n\
           SEVERITY: CRITICAL | HIGH | MEDIUM | LOW\n\
           LOCATION: <file and section>\n\
           PROBLEM: <what is wrong>\n\
           FIX: <what to change>\n\n\
         Score the feature 0-100 on security posture.\n\
         Deduct: 10 per CRITICAL, 5 per HIGH, 2 per MEDIUM, 0.5 per LOW.\n\
         End your response with exactly: SCORE: N"
    )
}

fn test_coverage_prompt(feat: &str) -> String {
    format!(
        "Read the project context from .solidspec/AGENT.md.\n\
         Feature: {feat} — find specs/{feat}/\n\n\
         You are performing a TEST COVERAGE review. Focus ONLY on:\n\
         - Every Given/When/Then scenario in spec.md has a test scaffold in tests/\n\
         - Test scaffolds are not all STATUS: NOT IMPLEMENTED for completed features\n\
         - Tasks reference user stories ([US1], [US2], etc.) for traceability\n\
         - Edge cases from spec.md appear in test scaffolds\n\
         - Test descriptions match the acceptance criteria they verify\n\n\
         DO NOT assess code quality, security, or performance.\n\n\
         For each gap found, state:\n\
           SEVERITY: CRITICAL | HIGH | MEDIUM | LOW\n\
           LOCATION: <file and section>\n\
           PROBLEM: <what is missing or wrong>\n\
           FIX: <what to add or change>\n\n\
         Score the feature 0-100 on test coverage.\n\
         Deduct: 10 per CRITICAL, 5 per HIGH, 2 per MEDIUM, 0.5 per LOW.\n\
         End your response with exactly: SCORE: N"
    )
}

fn performance_prompt(feat: &str) -> String {
    format!(
        "Read the project context from .solidspec/AGENT.md.\n\
         Feature: {feat} — find specs/{feat}/\n\n\
         You are performing a PERFORMANCE review. Focus ONLY on:\n\
         - Pagination strategy for any entity collection in spec.md and plan.md\n\
         - Measurable performance targets in success criteria addressed in plan\n\
         - Data model access patterns justified for expected load\n\
         - Caching strategy for read-heavy requirements\n\
         - Chunking/streaming for bulk import or export operations\n\
         - Unbounded queries (list all X with no page size) flagged\n\n\
         DO NOT assess code quality, security, or test coverage.\n\n\
         For each issue found, state:\n\
           SEVERITY: CRITICAL | HIGH | MEDIUM | LOW\n\
           LOCATION: <file and section>\n\
           PROBLEM: <what is missing or risky>\n\
           FIX: <what to add to plan.md or data-model.md>\n\n\
         Score the feature 0-100 on performance readiness.\n\
         Deduct: 10 per CRITICAL, 5 per HIGH, 2 per MEDIUM, 0.5 per LOW.\n\
         End your response with exactly: SCORE: N"
    )
}

/// Extract a 0–100 score from agent stdout.
///
/// Primary: scan for the last `SCORE: N` line (as instructed in the prompt).
/// Fallback: count `SEVERITY: LEVEL` lines and apply the penalty formula.
/// Returns 100 when neither is found (no findings reported → clean).
pub(crate) fn extract_score(stdout: &str) -> u8 {
    // Take the last SCORE: N match in the entire output.
    if let Some(caps) = SCORE_RE.captures_iter(stdout).last()
        && let Ok(n) = caps[1].parse::<u16>()
    {
        return n.min(100) as u8;
    }
    // Fallback: derive from SEVERITY: keyword frequency.
    derive_score_from_keywords(stdout)
}

/// Fallback scorer: counts `SEVERITY: LEVEL` lines and applies penalty formula.
fn derive_score_from_keywords(output: &str) -> u8 {
    let mut penalty = 0.0_f64;
    for caps in SEVERITY_RE.captures_iter(output) {
        penalty += match &caps[1] {
            "CRITICAL" => 10.0,
            "HIGH" => 5.0,
            "MEDIUM" => 2.0,
            "LOW" => 0.5,
            _ => 0.0,
        };
    }
    (100.0_f64 - penalty).max(0.0).round() as u8
}

/// Parse structured `SEVERITY / LOCATION / PROBLEM / FIX` blocks from agent stdout.
///
/// The agent is instructed (via the lane prompt) to format each finding as:
/// ```text
/// SEVERITY: HIGH
/// LOCATION: spec.md
/// PROBLEM: <description>
/// FIX: <remediation>
/// ```
/// Blocks are terminated by a new `SEVERITY:` line or end-of-output. `LOCATION:` is
/// consumed but not stored — `FanOutFinding` uses `lane` instead of a file path.
pub(crate) fn parse_findings_from_output(
    output: &str,
    lane_id: &'static str,
) -> Vec<FanOutFinding> {
    let mut findings = Vec::new();
    let mut current_severity: Option<Severity> = None;
    let mut current_problem = String::new();
    let mut current_fix = String::new();

    let flush = |sev: Option<Severity>, prob: &str, fix: &str, acc: &mut Vec<FanOutFinding>| {
        if let Some(severity) = sev
            && !prob.trim().is_empty()
        {
            acc.push(FanOutFinding {
                lane: lane_id,
                severity,
                message: prob.trim().to_string(),
                remediation: fix.trim().to_string(),
            });
        }
    };

    for line in output.lines() {
        let t = line.trim();

        if let Some(rest) = t.strip_prefix("SEVERITY:") {
            // Flush the previous block before starting a new one.
            flush(
                current_severity.take(),
                &current_problem,
                &current_fix,
                &mut findings,
            );
            current_problem.clear();
            current_fix.clear();
            current_severity = parse_severity(rest.trim());
        } else if let Some(rest) = t.strip_prefix("PROBLEM:") {
            current_problem = rest.trim().to_string();
        } else if let Some(rest) = t.strip_prefix("FIX:") {
            current_fix = rest.trim().to_string();
        }
        // LOCATION: is consumed silently — no field in FanOutFinding.
    }

    // Flush the final block.
    flush(
        current_severity,
        &current_problem,
        &current_fix,
        &mut findings,
    );

    findings
}

/// Map a severity label string to the `Severity` enum. Returns `None` for unknown labels.
fn parse_severity(s: &str) -> Option<Severity> {
    match s {
        "CRITICAL" => Some(Severity::Critical),
        "HIGH" => Some(Severity::High),
        "MEDIUM" => Some(Severity::Medium),
        "LOW" => Some(Severity::Low),
        _ => None,
    }
}

/// Execute a review lane by invoking an AI agent and parsing its stdout.
///
/// Calls [`invoke_agent_with_prompt`] which captures agent stdout, then
/// extracts the score via [`extract_score`] and the findings via
/// [`parse_findings_from_output`]. `timeout_secs` is passed through to the
/// invoker's per-lane timeout.
pub fn run_lane_with_agent(
    lane: &ReviewLane,
    project_root: &Path,
    timeout_secs: u64,
) -> LaneResult {
    use crate::agents::invoker::{self, InvokeResult};

    let start = Instant::now();

    let (score, findings, status) = match invoker::invoke_agent_with_prompt(
        &lane.agent_id,
        &lane.prompt,
        project_root,
        timeout_secs,
    ) {
        InvokeResult::Success { output } => {
            let score = extract_score(&output);
            let findings = parse_findings_from_output(&output, lane.id);
            (score, findings, LaneStatus::Done)
        }
        InvokeResult::NotAvailable { reason } => (0, vec![], LaneStatus::Failed(reason)),
        InvokeResult::Failed { error } if error.contains("timed out") => {
            (0, vec![], LaneStatus::TimedOut)
        }
        InvokeResult::Failed { error } => (0, vec![], LaneStatus::Failed(error)),
    };

    LaneResult {
        lane_id: lane.id,
        lane_label: lane.label,
        agent_id: lane.agent_id.clone(),
        score,
        findings,
        duration_ms: start.elapsed().as_millis() as u64,
        status,
        threshold: lane.threshold,
    }
}

// ── Phase 4: fan-out orchestration and ship gate ─────────────────────────────

/// Dispatch a lane to agent or heuristic execution based on `no_agent`.
pub fn run_lane(
    lane: ReviewLane,
    feature_dir: &Path,
    project_root: &Path,
    no_agent: bool,
    timeout_secs: u64,
) -> LaneResult {
    if no_agent {
        run_lane_no_agent(&lane, feature_dir, project_root)
    } else {
        run_lane_with_agent(&lane, project_root, timeout_secs)
    }
}

/// Run all lanes concurrently via OS threads, collecting results through an mpsc channel.
///
/// Each thread owns a cloned `PathBuf` to avoid lifetime issues across thread boundaries.
/// Results are printed as each lane completes. `drop(tx)` closes the last sender so
/// `rx` terminates once all threads finish.
pub fn run_fan_out(
    lanes: Vec<ReviewLane>,
    feature_dir: PathBuf,
    project_root: PathBuf,
    no_agent: bool,
    timeout_secs: u64,
) -> Vec<LaneResult> {
    let (tx, rx) = std::sync::mpsc::channel::<LaneResult>();

    for lane in lanes {
        let tx = tx.clone();
        let feat = feature_dir.clone();
        let root = project_root.clone();
        std::thread::spawn(move || {
            let result = run_lane(lane, &feat, &root, no_agent, timeout_secs);
            tx.send(result).ok();
        });
    }
    drop(tx); // all senders dropped → rx terminates when all threads finish

    let mut results = Vec::new();
    for result in rx {
        print_lane_result(&result);
        results.push(result);
    }
    results
}

/// Print a single lane result line to stdout as it completes (real-time progress).
pub fn print_lane_result(r: &LaneResult) {
    let (icon, note) = match &r.status {
        LaneStatus::Done => {
            let icon = if r.score >= r.threshold { "✓" } else { "✗" };
            let note = if r.score < r.threshold {
                format!("  ← below threshold ({})", r.threshold)
            } else {
                String::new()
            };
            (icon, note)
        }
        LaneStatus::TimedOut => ("⏱", " ← timed out".to_string()),
        LaneStatus::Failed(e) => ("✗", format!(" ← failed: {e}")),
    };
    println!(
        "  {} {:<18} ({})   {}/100  done in {:.1}s{}",
        icon,
        r.lane_label,
        r.agent_id,
        r.score,
        r.duration_ms as f64 / 1000.0,
        note
    );
}

/// Aggregate lane results into a `ShipReport` using the ship gate logic.
///
/// Evaluation order per lane (first matching rule wins):
/// 1. `TimedOut` → HOLD (skipped when `ignore_timeout = true`)
/// 2. `Failed` → HOLD (always)
/// 3. Critical finding in security lane → HOLD (unconditional)
/// 4. Critical finding in any lane when `block_on_critical` → HOLD
/// 5. Score below lane threshold → HOLD (blocking High + Critical findings surfaced)
pub fn aggregate_results(
    results: Vec<LaneResult>,
    feature_id: &str,
    block_on_critical: bool,
    ignore_timeout: bool,
) -> ShipReport {
    let mut ship = true;
    let mut blocking_findings: Vec<FanOutFinding> = Vec::new();

    for result in &results {
        match &result.status {
            LaneStatus::TimedOut => {
                if !ignore_timeout {
                    ship = false;
                    blocking_findings.push(FanOutFinding {
                        lane: result.lane_id,
                        severity: Severity::Critical,
                        message: format!(
                            "Lane '{}' timed out — review manually",
                            result.lane_label
                        ),
                        remediation: "Re-run with a longer --timeout or investigate agent hang."
                            .to_string(),
                    });
                }
                continue;
            }
            LaneStatus::Failed(msg) => {
                ship = false;
                blocking_findings.push(FanOutFinding {
                    lane: result.lane_id,
                    severity: Severity::Critical,
                    message: format!("Lane '{}' agent failed: {}", result.lane_label, msg),
                    remediation: "Check agent availability and re-run.".to_string(),
                });
                continue;
            }
            LaneStatus::Done => {}
        }

        let has_critical = result
            .findings
            .iter()
            .any(|f| f.severity == Severity::Critical);
        if has_critical && (result.lane_id == "security" || block_on_critical) {
            ship = false;
            for f in result
                .findings
                .iter()
                .filter(|f| f.severity == Severity::Critical)
            {
                if !blocking_findings.iter().any(|b| b.message == f.message) {
                    blocking_findings.push(f.clone());
                }
            }
        }

        if result.score < result.threshold {
            ship = false;
            for f in result
                .findings
                .iter()
                .filter(|f| matches!(f.severity, Severity::Critical | Severity::High))
            {
                if !blocking_findings.iter().any(|b| b.message == f.message) {
                    blocking_findings.push(f.clone());
                }
            }
        }
    }

    // When ignore_timeout is true and every lane timed out, there is no real review data.
    // A vacuous SHIP would be a false positive — hold instead.
    if ignore_timeout
        && !results.is_empty()
        && results.iter().all(|r| r.status == LaneStatus::TimedOut)
    {
        ship = false;
        blocking_findings.push(FanOutFinding {
            lane: "all",
            severity: Severity::Critical,
            message: "All review lanes timed out — no data to evaluate".to_string(),
            remediation: "Re-run with a longer --timeout or check agent availability.".to_string(),
        });
    }

    ShipReport {
        feature_id: feature_id.to_string(),
        decision: if ship {
            ShipDecision::Ship
        } else {
            ShipDecision::Hold
        },
        lanes: results,
        blocking_findings,
    }
}

/// Render a `ShipReport` as Markdown with a machine-readable `<!-- ship: bool -->` header.
pub fn format_ship_report(report: &ShipReport) -> String {
    let is_ship = report.decision == ShipDecision::Ship;
    let decision_str = if is_ship { "SHIP" } else { "HOLD" };
    let generated = chrono::Utc::now().to_rfc3339();

    let mut out = format!(
        "# Ship Report: {feature}\n\n\
         <!-- ship: {is_ship} -->\n\
         <!-- generated: {generated} -->\n\n\
         **Decision**: {decision_str}\n\n\
         ## Lane Scores\n\n\
         | Lane | Agent | Score | Threshold | Status |\n\
         |------|-------|-------|-----------|--------|\n",
        feature = report.feature_id,
    );

    for result in &report.lanes {
        let status_cell = match &result.status {
            LaneStatus::Done if result.score >= result.threshold => "✓ Pass",
            LaneStatus::Done => "✗ Fail",
            LaneStatus::TimedOut => "⏱ Timed Out",
            LaneStatus::Failed(_) => "✗ Failed",
        };
        out.push_str(&format!(
            "| {} | {} | {}/100 | {} | {} |\n",
            result.lane_label, result.agent_id, result.score, result.threshold, status_cell
        ));
    }

    if !report.blocking_findings.is_empty() {
        out.push_str("\n## Blocking Findings\n");

        // Group findings by lane for readable output.
        let mut by_lane: std::collections::BTreeMap<&str, Vec<&FanOutFinding>> =
            std::collections::BTreeMap::new();
        for f in &report.blocking_findings {
            by_lane.entry(f.lane).or_default().push(f);
        }
        for (lane_id, findings) in &by_lane {
            let label = report
                .lanes
                .iter()
                .find(|r| r.lane_id == *lane_id)
                .map(|r| r.lane_label)
                .unwrap_or(lane_id);
            out.push_str(&format!("\n### {label}\n\n"));
            for f in findings {
                out.push_str(&format!(
                    "- **[{}]** {}\n  *Fix*: {}\n\n",
                    f.severity, f.message, f.remediation
                ));
            }
        }
    }

    out.push_str(&format!(
        "\n## Re-run\n\n```bash\nsolidspec ship {}\n```\n",
        report.feature_id
    ));

    out
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_lane(id: &'static str, label: &'static str) -> ReviewLane {
        ReviewLane {
            id,
            label,
            agent_id: "heuristic".to_string(),
            prompt: String::new(),
            threshold: 70,
        }
    }

    fn write(dir: &Path, rel: &str, content: &str) {
        let full = dir.join(rel);
        if let Some(p) = full.parent() {
            std::fs::create_dir_all(p).unwrap();
        }
        std::fs::write(full, content).unwrap();
    }

    // A well-formed spec used as the "clean" baseline.
    const GOOD_SPEC: &str = r#"# Feature Spec

## User Scenarios & Testing

### User Story 1 - Login (Priority: P1)

**Acceptance Scenarios**:

1. **Given** valid credentials, **When** user submits form, **Then** session is created

### User Story 2 - Password reset (Priority: P2)

**Acceptance Scenarios**:

1. **Given** registered email, **When** user requests reset, **Then** email is sent

## Requirements

### Functional Requirements

- **FR-001**: System MUST authenticate users via email
- **FR-002**: System MUST allow password resets

### Key Entities

- **[User]**: A registered account holder
- **[Session]**: An active authentication session

## Success Criteria

- **SC-001**: Users can log in and out
"#;

    const GOOD_PLAN: &str = "# Plan\n\
        FR-001 addressed by authentication module.\nFR-002 addressed by email service.\n\
        User entity stored in users table.\nSession managed via JWT tokens.\n";
    const GOOD_TASKS: &str =
        "# Tasks\n- [ ] T001 Setup project [US1]\n- [ ] T002 Email service [US2]\n";

    // ── apply_penalty_formula ────────────────────────────────────────────────

    #[test]
    fn penalty_formula_two_high_findings_scores_90() {
        let findings = vec![
            FanOutFinding {
                lane: "code",
                severity: Severity::High,
                message: "Issue 1".into(),
                remediation: "Fix 1".into(),
            },
            FanOutFinding {
                lane: "code",
                severity: Severity::High,
                message: "Issue 2".into(),
                remediation: "Fix 2".into(),
            },
        ];
        assert_eq!(apply_penalty_formula(&findings), 90);
    }

    #[test]
    fn penalty_formula_no_findings_scores_100() {
        assert_eq!(apply_penalty_formula(&[]), 100);
    }

    #[test]
    fn penalty_formula_mixed_severities() {
        // 1 CRITICAL (10) + 1 HIGH (5) + 1 MEDIUM (2) + 1 LOW (0.5) = 17.5 → 83
        let findings = vec![
            FanOutFinding {
                lane: "code",
                severity: Severity::Critical,
                message: "c".into(),
                remediation: "fix".into(),
            },
            FanOutFinding {
                lane: "code",
                severity: Severity::High,
                message: "h".into(),
                remediation: "fix".into(),
            },
            FanOutFinding {
                lane: "code",
                severity: Severity::Medium,
                message: "m".into(),
                remediation: "fix".into(),
            },
            FanOutFinding {
                lane: "code",
                severity: Severity::Low,
                message: "l".into(),
                remediation: "fix".into(),
            },
        ];
        assert_eq!(apply_penalty_formula(&findings), 83); // round(100 - 17.5) = 83
    }

    #[test]
    fn penalty_formula_clamped_at_zero() {
        let findings: Vec<FanOutFinding> = (0..15)
            .map(|i| FanOutFinding {
                lane: "code",
                severity: Severity::Critical,
                message: format!("Critical {i}"),
                remediation: "Fix".into(),
            })
            .collect();
        assert_eq!(apply_penalty_formula(&findings), 0);
    }

    // ── run_lane_no_agent ────────────────────────────────────────────────────

    #[test]
    fn code_lane_placeholder_spec_is_penalized() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        // Prepend a [TODO] to an otherwise valid spec → Completeness/Medium → score < 100
        let spec = format!("[TODO: fill in header]\n{GOOD_SPEC}");
        write(&feature, "spec.md", &spec);
        write(&feature, "plan.md", GOOD_PLAN);
        write(&feature, "tasks.md", GOOD_TASKS);

        let lane = make_lane("code", "Code Review");
        let result = run_lane_no_agent(&lane, &feature, dir.path());

        assert_eq!(result.status, LaneStatus::Done);
        assert!(
            result.score < 100,
            "placeholder spec should reduce code lane score (got {})",
            result.score
        );
        assert!(
            result.findings.iter().all(|f| f.lane == "code"),
            "all findings must carry the correct lane id"
        );
    }

    #[test]
    fn security_lane_auth_spec_without_plan_security_section_is_penalized() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-auth");
        write(&feature, "spec.md", GOOD_SPEC); // has "authenticate" + "password"
        // Plan has NO "security" or "authentication" — triggers Security/Medium finding
        write(
            &feature,
            "plan.md",
            "# Plan\nFR-001 handled by login module.\nFR-002 handled by email module.\nUser: users table.\n",
        );
        write(&feature, "tasks.md", GOOD_TASKS);

        let lane = make_lane("security", "Security Audit");
        let result = run_lane_no_agent(&lane, &feature, dir.path());

        assert_eq!(result.status, LaneStatus::Done);
        assert!(
            result.score < 100,
            "auth spec without security plan should penalize security lane (got {})",
            result.score
        );
        assert!(
            result.findings.iter().any(|f| f.lane == "security"),
            "security lane must surface at least one finding"
        );
    }

    #[test]
    fn clean_spec_code_lane_scores_at_or_above_threshold() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-clean");
        write(&feature, "spec.md", GOOD_SPEC);
        // Plan mentions "authentication" → no Security/Medium finding bleeds into code lane
        write(&feature, "plan.md", GOOD_PLAN);
        write(&feature, "tasks.md", GOOD_TASKS);

        let lane = make_lane("code", "Code Review");
        let result = run_lane_no_agent(&lane, &feature, dir.path());

        assert_eq!(result.status, LaneStatus::Done);
        assert!(
            result.score >= lane.threshold,
            "clean spec should score at or above code threshold {} (got {})",
            lane.threshold,
            result.score
        );
    }

    #[test]
    fn missing_spec_returns_failed_status() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-missing");
        std::fs::create_dir_all(&feature).unwrap(); // no spec.md

        let lane = make_lane("code", "Code Review");
        let result = run_lane_no_agent(&lane, &feature, dir.path());

        assert!(
            matches!(result.status, LaneStatus::Failed(_)),
            "missing spec.md should produce Failed status, got {:?}",
            result.status
        );
        assert_eq!(result.score, 0);
        assert!(result.findings.is_empty());
    }

    #[test]
    fn score_from_heuristics_returns_err_when_no_spec() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-missing");
        std::fs::create_dir_all(&feature).unwrap();

        let lane = make_lane("code", "Code Review");
        assert!(
            score_from_heuristics(&lane, &feature, dir.path()).is_err(),
            "score_from_heuristics must return Err when spec.md is absent"
        );
    }

    #[test]
    fn tests_lane_covers_testability_only() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        let spec = "# Spec\n[TODO: fill this]\n\
                    ## User Scenarios & Testing\n\
                    ### User Story 1 - Login (Priority: P1)\nNo scenarios.\n\
                    ## Requirements\n### Functional Requirements\n\
                    - **FR-001**: System MUST authenticate\n\
                    ### Key Entities\n- **[User]**: A user\n\
                    ## Success Criteria\n- **SC-001**: Users can log in\n";
        write(&feature, "spec.md", spec);

        let lane = make_lane("tests", "Test Coverage");
        let result = run_lane_no_agent(&lane, &feature, dir.path());

        assert_eq!(result.status, LaneStatus::Done);
        assert!(
            result.findings.iter().all(|f| f.lane == "tests"),
            "tests lane must only carry tests-cluster findings"
        );
        assert!(
            !result.findings.iter().any(|f| f.message.contains("TODO")),
            "tests lane must not include Completeness findings"
        );
    }

    #[test]
    fn duration_ms_is_recorded() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-time");
        std::fs::create_dir_all(&feature).unwrap();

        let lane = make_lane("code", "Code Review");
        let result = run_lane_no_agent(&lane, &feature, dir.path());

        assert!(
            result.duration_ms < 60_000,
            "duration should be less than 60s, got {}ms",
            result.duration_ms
        );
    }

    // ── extract_score ────────────────────────────────────────────────────────

    #[test]
    fn extract_score_parses_score_line() {
        assert_eq!(extract_score("Some output\nSCORE: 87\n"), 87);
    }

    #[test]
    fn extract_score_takes_last_score_line() {
        // If agent writes two SCORE: lines, last one wins.
        assert_eq!(extract_score("SCORE: 50\nSome more text\nSCORE: 72\n"), 72);
    }

    #[test]
    fn extract_score_clamps_over_100() {
        assert_eq!(extract_score("SCORE: 999"), 100);
    }

    #[test]
    fn extract_score_score_100_is_not_clamped() {
        assert_eq!(extract_score("SCORE: 100"), 100);
    }

    #[test]
    fn extract_score_no_keywords_returns_100() {
        // No SCORE: line and no SEVERITY: keywords → no findings → clean → 100
        assert_eq!(
            extract_score("Some neutral prose without any keywords."),
            100
        );
    }

    #[test]
    fn extract_score_fallback_two_high_findings_scores_90() {
        // No SCORE: line; 2 × SEVERITY: HIGH → 2×5 = 10 penalty → 90
        let output = "SEVERITY: HIGH\nPROBLEM: Issue 1\nFIX: Fix 1\n\n\
                      SEVERITY: HIGH\nPROBLEM: Issue 2\nFIX: Fix 2\n";
        assert_eq!(extract_score(output), 90);
    }

    #[test]
    fn extract_score_fallback_mixed_penalties() {
        // 1 CRITICAL (10) + 1 MEDIUM (2) = 12 → 88
        let output = "SEVERITY: CRITICAL\nPROBLEM: Bad thing\nFIX: Fix it\n\n\
                      SEVERITY: MEDIUM\nPROBLEM: Small thing\nFIX: Tweak it\n";
        assert_eq!(extract_score(output), 88);
    }

    // ── parse_findings_from_output ───────────────────────────────────────────

    #[test]
    fn parse_findings_extracts_two_findings() {
        let output = "SEVERITY: HIGH\n\
                      LOCATION: spec.md\n\
                      PROBLEM: Missing pagination strategy\n\
                      FIX: Add pagination section to plan.md\n\n\
                      SEVERITY: MEDIUM\n\
                      LOCATION: plan.md\n\
                      PROBLEM: No caching strategy\n\
                      FIX: Document cache invalidation policy\n";

        let findings = parse_findings_from_output(output, "perf");

        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].severity, Severity::High);
        assert_eq!(findings[0].message, "Missing pagination strategy");
        assert_eq!(findings[0].remediation, "Add pagination section to plan.md");
        assert_eq!(findings[0].lane, "perf");
        assert_eq!(findings[1].severity, Severity::Medium);
        assert_eq!(findings[1].message, "No caching strategy");
        assert_eq!(findings[1].lane, "perf");
    }

    #[test]
    fn parse_findings_empty_output_returns_empty() {
        assert!(parse_findings_from_output("", "code").is_empty());
    }

    #[test]
    fn parse_findings_unknown_severity_skips_block() {
        let output = "SEVERITY: UNKNOWN\nPROBLEM: Something\nFIX: Fix it\n";
        // Unknown severity → parse_severity returns None → block is not flushed
        assert!(parse_findings_from_output(output, "code").is_empty());
    }

    #[test]
    fn parse_findings_problem_without_fix_still_captured() {
        let output = "SEVERITY: HIGH\nLOCATION: spec.md\nPROBLEM: Missing section\n";
        let findings = parse_findings_from_output(output, "code");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].message, "Missing section");
        assert_eq!(findings[0].remediation, ""); // no FIX: line → empty
    }

    // ── build_lanes ──────────────────────────────────────────────────────────

    #[test]
    fn build_lanes_creates_four_lanes() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-task-manager");
        std::fs::create_dir_all(&feature).unwrap();

        let config = FanOutConfig::default();
        let lanes = build_lanes(&config, &feature, "claude");

        assert_eq!(lanes.len(), 4);
        let ids: Vec<&str> = lanes.iter().map(|l| l.id).collect();
        assert_eq!(ids, vec!["code", "security", "tests", "perf"]);
    }

    #[test]
    fn build_lanes_uses_default_agent_when_no_override() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature).unwrap();

        let config = FanOutConfig::default(); // all agent overrides are None
        let lanes = build_lanes(&config, &feature, "gemini");

        assert!(
            lanes.iter().all(|l| l.agent_id == "gemini"),
            "all lanes should fall back to the supplied default agent"
        );
    }

    #[test]
    fn build_lanes_uses_per_lane_agent_override() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature).unwrap();

        let mut config = FanOutConfig::default();
        config.security_agent = Some("gemini".to_string());

        let lanes = build_lanes(&config, &feature, "claude");
        let security = lanes.iter().find(|l| l.id == "security").unwrap();
        let code = lanes.iter().find(|l| l.id == "code").unwrap();

        assert_eq!(security.agent_id, "gemini");
        assert_eq!(code.agent_id, "claude"); // falls back to default
    }

    #[test]
    fn build_lanes_uses_config_thresholds() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature).unwrap();

        let mut config = FanOutConfig::default();
        config.security_threshold = 90;
        config.perf_threshold = 50;

        let lanes = build_lanes(&config, &feature, "claude");
        let sec = lanes.iter().find(|l| l.id == "security").unwrap();
        let perf = lanes.iter().find(|l| l.id == "perf").unwrap();

        assert_eq!(sec.threshold, 90);
        assert_eq!(perf.threshold, 50);
    }

    #[test]
    fn build_lanes_prompts_contain_feature_name() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-task-manager");
        std::fs::create_dir_all(&feature).unwrap();

        let config = FanOutConfig::default();
        let lanes = build_lanes(&config, &feature, "claude");

        for lane in &lanes {
            assert!(
                lane.prompt.contains("001-task-manager"),
                "{} prompt must contain the feature dir name",
                lane.label
            );
            assert!(
                !lane.prompt.is_empty(),
                "{} prompt must not be empty",
                lane.label
            );
        }
    }

    #[test]
    fn build_lanes_prompts_contain_score_instruction() {
        let dir = TempDir::new().unwrap();
        let feature = dir.path().join("specs/001-test");
        std::fs::create_dir_all(&feature).unwrap();

        let config = FanOutConfig::default();
        let lanes = build_lanes(&config, &feature, "claude");

        for lane in &lanes {
            assert!(
                lane.prompt.contains("SCORE: N"),
                "{} prompt must instruct the agent to end with SCORE: N",
                lane.label
            );
        }
    }

    // ── aggregate_results ────────────────────────────────────────────────────

    fn make_done_result(
        lane_id: &'static str,
        label: &'static str,
        score: u8,
        threshold: u8,
    ) -> LaneResult {
        LaneResult {
            lane_id,
            lane_label: label,
            agent_id: "test".to_string(),
            score,
            findings: vec![],
            duration_ms: 100,
            status: LaneStatus::Done,
            threshold,
        }
    }

    fn make_critical_finding(lane: &'static str) -> FanOutFinding {
        FanOutFinding {
            lane,
            severity: Severity::Critical,
            message: "Critical issue".to_string(),
            remediation: "Fix it".to_string(),
        }
    }

    #[test]
    fn all_lanes_pass_returns_ship() {
        let results = vec![
            make_done_result("code", "Code Review", 85, 70),
            make_done_result("security", "Security Audit", 90, 80),
            make_done_result("tests", "Test Coverage", 75, 70),
            make_done_result("perf", "Performance", 65, 60),
        ];
        let report = aggregate_results(results, "001-test", true, false);
        assert_eq!(report.decision, ShipDecision::Ship);
        assert!(report.blocking_findings.is_empty());
    }

    #[test]
    fn one_lane_below_threshold_returns_hold() {
        let results = vec![
            make_done_result("code", "Code Review", 85, 70),
            make_done_result("tests", "Test Coverage", 60, 70), // below threshold
        ];
        let report = aggregate_results(results, "001-test", true, false);
        assert_eq!(report.decision, ShipDecision::Hold);
    }

    #[test]
    fn critical_finding_in_security_lane_always_holds() {
        let mut security_result = make_done_result("security", "Security Audit", 95, 80);
        security_result
            .findings
            .push(make_critical_finding("security"));

        let results = vec![security_result];
        let report = aggregate_results(results, "001-test", false, false); // block_on_critical = false
        assert_eq!(
            report.decision,
            ShipDecision::Hold,
            "critical in security lane must block regardless of block_on_critical"
        );
    }

    #[test]
    fn timed_out_lane_returns_hold() {
        let results = vec![
            make_done_result("code", "Code Review", 85, 70),
            LaneResult {
                lane_id: "security",
                lane_label: "Security Audit",
                agent_id: "test".to_string(),
                score: 0,
                findings: vec![],
                duration_ms: 300_000,
                status: LaneStatus::TimedOut,
                threshold: 80,
            },
        ];
        let report = aggregate_results(results, "001-test", true, false);
        assert_eq!(report.decision, ShipDecision::Hold);
        assert!(
            report
                .blocking_findings
                .iter()
                .any(|f| f.message.contains("timed out")),
            "blocking findings must mention the timed out lane"
        );
    }

    #[test]
    fn ignore_timeout_uses_partial_results() {
        let results = vec![
            make_done_result("code", "Code Review", 85, 70),
            LaneResult {
                lane_id: "security",
                lane_label: "Security Audit",
                agent_id: "test".to_string(),
                score: 0,
                findings: vec![],
                duration_ms: 300_000,
                status: LaneStatus::TimedOut,
                threshold: 80,
            },
            make_done_result("tests", "Test Coverage", 80, 70),
        ];
        let report = aggregate_results(results, "001-test", true, true); // ignore_timeout = true
        assert_eq!(
            report.decision,
            ShipDecision::Ship,
            "with ignore_timeout, timed-out lane should not block when others pass"
        );
    }

    #[test]
    fn all_lanes_failed_returns_hold_with_message() {
        let results = vec![
            LaneResult {
                lane_id: "code",
                lane_label: "Code Review",
                agent_id: "test".to_string(),
                score: 0,
                findings: vec![],
                duration_ms: 10,
                status: LaneStatus::Failed("agent crash".to_string()),
                threshold: 70,
            },
            LaneResult {
                lane_id: "security",
                lane_label: "Security Audit",
                agent_id: "test".to_string(),
                score: 0,
                findings: vec![],
                duration_ms: 10,
                status: LaneStatus::Failed("agent crash".to_string()),
                threshold: 80,
            },
        ];
        let report = aggregate_results(results, "001-test", true, false);
        assert_eq!(report.decision, ShipDecision::Hold);
        // Each failed lane produces its own blocking finding.
        assert_eq!(report.blocking_findings.len(), 2);
        assert!(
            report
                .blocking_findings
                .iter()
                .any(|f| f.message.contains("Code Review")),
            "must surface the code lane failure"
        );
    }

    #[test]
    fn all_lanes_timed_out_with_ignore_timeout_returns_hold() {
        let make_timeout = |lane_id: &'static str, label: &'static str| LaneResult {
            lane_id,
            lane_label: label,
            agent_id: "test".to_string(),
            score: 0,
            findings: vec![],
            duration_ms: 300_000,
            status: LaneStatus::TimedOut,
            threshold: 70,
        };
        let results = vec![
            make_timeout("code", "Code Review"),
            make_timeout("security", "Security Audit"),
            make_timeout("tests", "Test Coverage"),
            make_timeout("perf", "Performance"),
        ];
        // With ignore_timeout=true, timed-out lanes are normally non-blocking.
        // But when ALL lanes time out, there is no real data — must still HOLD.
        let report = aggregate_results(results, "001-test", true, true);
        assert_eq!(
            report.decision,
            ShipDecision::Hold,
            "all lanes timed out with ignore_timeout=true must still produce HOLD"
        );
        assert!(
            report
                .blocking_findings
                .iter()
                .any(|f| f.message.contains("timed out")),
            "blocking findings must explain the all-timed-out situation"
        );
    }

    // ── format_ship_report ───────────────────────────────────────────────────

    #[test]
    fn format_ship_report_contains_machine_readable_hold_header() {
        let mut result = make_done_result("code", "Code Review", 50, 70);
        result.findings.push(FanOutFinding {
            lane: "code",
            severity: Severity::High,
            message: "Missing section".to_string(),
            remediation: "Add it".to_string(),
        });
        let results = vec![result];
        let report = aggregate_results(results, "001-test", false, false);
        let markdown = format_ship_report(&report);

        assert!(
            markdown.contains("<!-- ship: false -->"),
            "HOLD report must have machine-readable ship: false header"
        );
        assert!(markdown.contains("## Lane Scores"), "must have lane table");
        assert!(
            markdown.contains("## Blocking Findings"),
            "must have blocking findings section"
        );
        assert!(markdown.contains("Code Review"), "must mention lane label");
    }

    #[test]
    fn format_ship_report_contains_machine_readable_ship_header() {
        let results = vec![make_done_result("code", "Code Review", 85, 70)];
        let report = aggregate_results(results, "001-clean", false, false);
        let markdown = format_ship_report(&report);
        assert!(markdown.contains("<!-- ship: true -->"));
        assert!(!markdown.contains("## Blocking Findings"));
    }
}
