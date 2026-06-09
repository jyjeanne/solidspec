# Parallel Fan-Out Orchestration — Implementation Plan

**Feature:** Parallel Fan-Out Orchestration  
**Target version:** v0.4.0  
**Priority:** HIGH  
**Estimated effort:** 14h (revised from 12h — invoker extension and heuristic fallback added)  
**Status:** Planned

---

## 1. Overview

Today's review pipeline is sequential: `solidspec analyze` runs first, then `solidspec review` — both on a single thread, one dimension at a time. This means a code-quality finding blocks waiting for security analysis, and a security finding waits for performance checks. The total review wall-clock time is the sum of every dimension.

**Parallel Fan-Out Orchestration** changes this to a fan-out model:

```
                  ┌─── Code Review Agent ──────►  (code findings, score)
                  │
solidspec ship ───┤─── Security Audit Agent ──►  (security findings, score)
                  │
                  ├─── Test Coverage Agent ──►   (test findings, score)
                  │
                  └─── Performance Agent ─────►  (perf findings, score)
                             │
                          (join via mpsc::channel)
                             │
                      Ship Gate Aggregator
                             │
                      ┌──────┴──────┐
                   SHIP ✓       HOLD ✗
                             │
                       ship-report.md
```

Four specialized agents run concurrently using OS threads. Each focuses on a specific review dimension cluster. When all complete (or the per-lane timeout fires), a **ship gate** aggregates findings and makes a binary `SHIP / HOLD` decision based on configurable per-dimension thresholds.

This is the **first concurrent multi-agent review system** in the SDD space. Review wall-clock time drops from `sum(all dimensions)` to `max(slowest agent)` — typically 3–4× faster.

---

## 2. Problem Statement

### Where sequential review falls short

| Pain point | Impact |
|------------|--------|
| Review is a blocking single-thread operation | Code + security + tests + performance = sum of all latencies. Fan-out reduces this to max of all latencies. |
| No binary ship signal | `solidspec review` produces a report but leaves the go/no-go decision to the developer. Large reports bury the critical finding. |
| All dimensions carry equal weight | A Low-severity style finding and a Critical security hole appear in the same severity sort. No per-dimension threshold exists to block shipping. |
| One reviewer context-switches internally | A single agent reviewing code, security, tests, and performance loses focus on each area. |
| No specialized prompts | The current `review` command uses a generic multi-dimension prompt. Agents produce better findings with a narrower, deeper focus. |

### What fan-out adds

| Addition | Benefit |
|----------|---------|
| Concurrent execution | Review time = `max(agent latency)` not `sum(agent latency)` |
| Dimension-specialized prompts | Each agent gets a deep, focused prompt for its cluster — no context switching |
| Configurable ship gate | Per-dimension score thresholds block shipping if any critical dimension fails |
| Explicit SHIP / HOLD decision | Eliminates ambiguity; integrates cleanly into CI (`--fail-on-hold`) |
| Independent partial results | If one lane times out, the other three results are still usable |
| Heuristic fallback | When no agent is available (`--no-agent`), existing `preflight_review()` and `analyze_feature()` findings are mapped to lane scores — no placeholder zeros |
| `ship-report.md` artifact | Machine-readable `<!-- ship: true/false -->` header suitable for git hooks and CI |

### `solidspec review` vs `solidspec ship`

These two commands are complementary, not alternatives:

| | `solidspec review` | `solidspec ship` |
|---|---|---|
| **Purpose** | Detailed quality assessment with remediation guidance | Binary SHIP / HOLD decision for release readiness |
| **Execution** | Single agent, sequential, 8 dimensions | 4 agents, concurrent, 4 dimension clusters |
| **Output** | `review-report.md` with scored dimension table | `ship-report.md` with blocking findings only |
| **Use when** | Iterating on spec/plan quality during development | Deciding whether to merge/release a completed feature |
| **CI integration** | Not designed for exit-code gates | `--fail-on-hold` exits 1 on HOLD |
| **Agent required** | No (heuristic only) | No (`--no-agent` uses heuristic fallback) |

---

## 3. Architecture

### Concurrency model

Each agent invocation is a subprocess (`std::process::Command`). Subprocess-based concurrency maps naturally to `std::thread::spawn` — each thread owns a `ReviewHandle` that blocks on a `try_wait()` polling loop. No `tokio` or async runtime is needed.

**Timeout policy**: each lane has an independent per-lane timeout (default 300s). When a lane times out, its thread kills the child process and sends a `LaneStatus::TimedOut` result through the channel. Other running lanes are **not** killed — they continue to their own timeouts or completion. This gives the best partial-result coverage.

```
main thread
    │
    │  mpsc::channel::<LaneResult>()
    │         │ tx cloned × 4        │ rx (main thread)
    │         │                      │
    ├── thread::spawn (code)   ──tx──┤
    ├── thread::spawn (security)──tx─┤  ← results arrive as lanes finish
    ├── thread::spawn (tests)  ──tx──┤
    └── thread::spawn (perf)   ──tx──┘
              │
         drop(tx)  ← rx terminates when all senders dropped
              │
         aggregate_results() → ShipReport
```

### Score extraction from agent output

Agents are prompted to end their response with `SCORE: N` (0–100). Extraction uses a regex on the last 200 bytes of agent stdout: `r"SCORE:\s*(\d{1,3})"`.

**Fallback chain** (in order):
1. Parse `SCORE: N` from agent stdout → use if 0 ≤ N ≤ 100
2. Agent stdout missing/unparseable → derive score from finding counts in the response text using the penalty formula (10 × CRITICAL + 5 × HIGH + 2 × MEDIUM + 0.5 × LOW, deducted from 100)
3. Agent failed or timed out → score = 0, status = `Failed` / `TimedOut`

This resolves Open Question 1: `SCORE: N` is the primary path; the penalty-formula derivation is the fallback. A `--format json` mode is a v0.5.0 concern.

### `--no-agent` mode: heuristic lane scoring

When `--no-agent` is passed, each lane runs existing heuristic checks instead of invoking an agent:

| Lane | Heuristic source |
|------|-----------------|
| Code | `review::preflight_review()` — Completeness + Clarity + Consistency + Maintainability findings |
| Security | `review::preflight_review()` — Security dimension findings |
| Tests | `review::preflight_review()` — Testability dimension findings |
| Performance | `analyzer::analyze_feature()` — entity/traceability findings as a proxy; Performance dimension from review |

Each lane filters the heuristic findings to its own dimension cluster and applies the standard penalty formula to produce a score. This is more useful than placeholder zeros and ensures `--no-agent` produces actionable results for CI environments without agents.

### New types in `src/core/fan_out.rs`

```rust
/// One specialized review cluster.
#[derive(Clone)]
pub struct ReviewLane {
    pub id: &'static str,     // "code" | "security" | "tests" | "perf"
    pub label: &'static str,  // "Code Review" | …
    pub agent_id: String,     // from config or default_agent
    pub prompt: String,       // specialized deep-focus prompt
    pub threshold: u8,        // 0–100; score below this → HOLD
}

/// Result returned by one lane after its agent/heuristic completes.
pub struct LaneResult {
    pub lane_id: &'static str,
    pub lane_label: &'static str,
    pub agent_id: String,
    pub score: u8,                  // 0–100
    pub findings: Vec<FanOutFinding>,
    pub duration_ms: u64,
    pub status: LaneStatus,
}

#[derive(PartialEq, Eq)]
pub enum LaneStatus {
    Done,
    TimedOut,
    Failed(String),
}

/// Aggregated ship decision.
pub struct ShipReport {
    pub feature_id: String,
    pub decision: ShipDecision,
    pub lanes: Vec<LaneResult>,        // all lane results (pass + fail)
    pub blocking_findings: Vec<FanOutFinding>,
}

#[derive(PartialEq, Eq)]
pub enum ShipDecision { Ship, Hold }

/// A single finding surfaced by a review lane.
#[derive(Clone)]
pub struct FanOutFinding {
    pub lane: &'static str,
    // Reuses the existing Severity enum from core::review to avoid duplication
    pub severity: crate::core::review::Severity,
    pub message: String,
    pub remediation: String,
}
```

> **Note on `ReviewSeverity`**: the original draft referenced a `ReviewSeverity` type that does not exist. The corrected plan reuses `crate::core::review::Severity` (`Critical | High | Medium | Low | Info`) which already has the correct variants and `Display` impl.

### New files

| File | Role |
|------|------|
| `src/cli/ship.rs` | `solidspec ship [id]` command handler |
| `src/core/fan_out.rs` | Fan-out engine: types, lane building, thread orchestration, score extraction, ship gate, report formatting |
| `tests/ship.rs` | Integration tests |

### Changed files

| File | Change |
|------|--------|
| `src/cli/mod.rs` | Add `pub mod ship`; add `Ship` variant to `Commands`; add dispatch arm |
| `src/core/mod.rs` | Add `pub mod fan_out` *(missing from original draft)* |
| `src/agents/invoker.rs` | Add `invoke_agent_with_prompt(agent_id, prompt, project_root)` — the existing `invoke_agent` takes a phase name and builds the prompt internally; fan-out needs to supply its own prompt *(missing from original draft)* |
| `src/config/mod.rs` | Add `FanOutConfig`; add `fan_out: FanOutConfig` to `RootConfig` |
| `schemas/spec-driven/schema.yaml` | Add `ship` artifact (requires: analyze, review) — artifact count becomes **9** |
| `schemas/intent-driven/schema.yaml` | Same — artifact count becomes **11** |
| `src/core/schema.rs` | Update `parse_spec_driven_schema` test: `artifacts.len() == 9`; add `intent_driven_schema_has_ship_artifact` test |
| `src/core/pipeline.rs` | Add `"ship"` to `should_skip()` (skips when `ship-report.md` exists); add `phase_type("ship") → Auto`; **do NOT add `"ship"` to `PHASES` or `PHASES_IDSD` arrays** — `solidspec ship` is a standalone command, not a pipeline phase |

---

## 4. The Four Review Lanes

Each lane has a **specialized deep-focus prompt** that explicitly prohibits assessing other dimensions.

### Lane 1 — Code Review

**Cluster:** Completeness, Clarity, Consistency, Maintainability  
**Threshold:** 70 (default)

**Artifacts read:** `spec.md`, `plan.md`, `tasks.md`

**Checks:**
- Every `FR-###` in spec.md appears in plan.md (traceability)
- No placeholder text (`[TODO]`, `[TBD]`, `[To be filled]`, `[Brief Title]`) in any artifact
- Plan architecture decisions have explicit rationale
- Tasks are independently completable (each task has a clear deliverable)
- Entity names, requirement IDs, and user story labels are consistent across all three artifacts

**Prompt template:**

```
Read the project context from .solidspec/AGENT.md.
Feature: {feature_id} — find specs/{feature_dir}/

You are performing a CODE REVIEW. Focus ONLY on:
- Requirement completeness: every FR-### in spec.md is addressed in plan.md and tasks.md
- Clarity: no placeholder text remains in any artifact
- Consistency: entity names, FR-IDs, US-labels are consistent across spec/plan/tasks
- Maintainability: plan decisions are justified; tasks are each independently deliverable

DO NOT assess security, performance, or test coverage.

For each issue found, state:
  SEVERITY: CRITICAL | HIGH | MEDIUM | LOW
  LOCATION: <file and section>
  PROBLEM: <what is wrong>
  FIX: <what to change>

Score the feature 0–100 on code quality. Deduct: 10×CRITICAL, 5×HIGH, 2×MEDIUM, 0.5×LOW.
End your response with exactly: SCORE: {N}
```

---

### Lane 2 — Security Audit

**Cluster:** Security  
**Threshold:** 80 (default — higher; a single Critical security finding always blocks regardless of score)  
**Critical override:** `block_on_critical = true` applies globally but is enforced unconditionally for the Security lane regardless of config.

**Artifacts read:** `spec.md`, `plan.md`, `contracts/api.md` (if present)

**Checks:**
- Auth/session requirements have explicit security constraints (token expiry, rotation, scope)
- Data entities handling PII are identified and their storage/transmission policy stated
- API contracts don't expose internal surrogate IDs to the client
- No hardcoded credentials or connection strings in plan decisions
- OWASP Top 10 considered for each user-facing requirement: injection, broken auth, sensitive data exposure, security misconfiguration, XSS, IDOR
- Rate limiting addressed for any unauthenticated endpoint implied by the spec

**Prompt template:**

```
Read the project context from .solidspec/AGENT.md.
Feature: {feature_id} — find specs/{feature_dir}/

You are performing a SECURITY AUDIT. Focus ONLY on:
- Authentication and authorization: are constraints explicit in spec and plan?
- PII handling: identified entities with personal data have a stated storage/transmission policy
- API contract security: no internal IDs exposed, no sensitive data in URLs
- OWASP Top 10 coverage for user-facing requirements
- Hardcoded credentials or secrets in any artifact
- Rate limiting for unauthenticated endpoints

DO NOT assess code quality, test coverage, or performance.

For each issue found, state:
  SEVERITY: CRITICAL | HIGH | MEDIUM | LOW
  LOCATION: <file and section>
  PROBLEM: <what is wrong>
  FIX: <what to change>

Score the feature 0–100 on security posture. Deduct: 10×CRITICAL, 5×HIGH, 2×MEDIUM, 0.5×LOW.
End your response with exactly: SCORE: {N}
```

---

### Lane 3 — Test Coverage

**Cluster:** Testability  
**Threshold:** 70 (default)

**Artifacts read:** `spec.md`, `tasks.md`, `tests/` directory

**Checks:**
- Every Given/When/Then acceptance scenario in spec.md has a corresponding test scaffold file
- No test scaffold is blank or entirely `STATUS: NOT IMPLEMENTED` for a feature marked complete in tasks.md
- Tasks reference user stories (`[US#]`) so test traceability can be built
- Edge cases listed in spec.md appear in test scaffolds
- Test file names are meaningful and map to user stories

**Prompt template:**

```
Read the project context from .solidspec/AGENT.md.
Feature: {feature_id} — find specs/{feature_dir}/

You are performing a TEST COVERAGE review. Focus ONLY on:
- Every Given/When/Then scenario in spec.md has a test scaffold in tests/
- Test scaffolds are not all STATUS: NOT IMPLEMENTED for completed features
- Tasks reference user stories ([US1], [US2], etc.) for traceability
- Edge cases from spec.md appear in test scaffolds
- Test descriptions match the acceptance criteria they verify

DO NOT assess code quality, security, or performance.

For each gap found, state:
  SEVERITY: CRITICAL | HIGH | MEDIUM | LOW
  LOCATION: <file and section>
  PROBLEM: <what is missing or wrong>
  FIX: <what to add or change>

Score the feature 0–100 on test coverage. Deduct: 10×CRITICAL, 5×HIGH, 2×MEDIUM, 0.5×LOW.
End your response with exactly: SCORE: {N}
```

---

### Lane 4 — Performance

**Cluster:** Performance/Scalability  
**Threshold:** 60 (default — lower; scalability is often hardest to assess at spec/plan level without load data)

**Artifacts read:** `spec.md`, `plan.md`, `data-model.md` (if present)

**Checks:**
- Any entity collection implied by requirements has a pagination strategy in plan.md
- Success criteria with measurable performance targets (latency, throughput) are addressed in the plan
- Data model choices are justified for the expected access patterns
- No unbounded queries implied by the spec (e.g., "show all tasks" with no page size)
- Caching strategy stated for read-heavy requirements
- Any bulk import/export requirement has a chunking or streaming strategy

**Prompt template:**

```
Read the project context from .solidspec/AGENT.md.
Feature: {feature_id} — find specs/{feature_dir}/

You are performing a PERFORMANCE review. Focus ONLY on:
- Pagination strategy for any entity collection in spec.md and plan.md
- Measurable performance targets in success criteria addressed in plan
- Data model access patterns justified for expected load
- Caching strategy for read-heavy requirements
- Chunking/streaming for bulk import or export operations
- Unbounded queries (e.g., "list all X" with no page size) flagged

DO NOT assess code quality, security, or test coverage.

For each issue found, state:
  SEVERITY: CRITICAL | HIGH | MEDIUM | LOW
  LOCATION: <file and section>
  PROBLEM: <what is missing or risky>
  FIX: <what to add to plan.md or data-model.md>

Score the feature 0–100 on performance readiness. Deduct: 10×CRITICAL, 5×HIGH, 2×MEDIUM, 0.5×LOW.
End your response with exactly: SCORE: {N}
```

---

## 5. Ship Gate Logic

```
ship = true
blocking_findings = []

for each lane_result in results:
    if lane_result.status == TimedOut:
        ship = false
        blocking_findings.push("Lane '{label}' timed out — review manually")
        continue

    if lane_result.status == Failed(msg):
        ship = false
        blocking_findings.push("Lane '{label}' agent failed: {msg}")
        continue

    // Critical always blocks, regardless of threshold or block_on_critical config,
    // for the Security lane. For other lanes, block_on_critical from config applies.
    has_critical = lane_result.findings.iter().any(|f| f.severity == Critical)
    if has_critical && (lane_result.lane_id == "security" || config.block_on_critical):
        ship = false
        for f in lane_result.findings where severity == Critical:
            blocking_findings.push(f)

    if lane_result.score < lane.threshold:
        ship = false
        for f in lane_result.findings where severity in [Critical, High]:
            blocking_findings.push(f)

decision = if ship { SHIP } else { HOLD }
```

**Partial results**: lanes that complete before a timeout contribute their results regardless of what other lanes do.

**Edge cases and their decisions:**

| Situation | Decision | Rationale |
|-----------|----------|-----------|
| One lane times out, three pass | HOLD | Unknown state is never shippable |
| One lane fails (agent crash), three pass | HOLD | Same as timeout — unknown state |
| `--ignore-timeout` flag + one timeout | Uses partial results from 3 passing lanes | Opt-in escape hatch; not default |
| `--lane code,security`, both pass | SHIP (partial) | Partial gate; report notes incomplete coverage |
| All lanes fail | HOLD + "all reviews unavailable" message | |
| No findings, all scores ≥ thresholds | SHIP | |
| Score above threshold but Critical finding present (non-Security lane, `block_on_critical = false`) | SHIP | Critical finding shown in report; not a gate |

---

## 6. CLI Command: `solidspec ship`

```bash
# Run all 4 review lanes concurrently — ship decision when done
solidspec ship [id]

# Run only specified lanes (comma-separated: code, security, tests, perf)
solidspec ship 001 --lane security,tests

# CI mode: exit code 1 when HOLD, 0 when SHIP
solidspec ship 001 --fail-on-hold

# Override agent for a specific lane
solidspec ship 001 --security-agent gemini

# Heuristic-only mode (no AI agents invoked; uses existing preflight_review)
solidspec ship 001 --no-agent

# Preview planned lanes and thresholds without invoking any agent
solidspec ship 001 --dry-run

# Override per-lane timeout in seconds (default: 300)
solidspec ship 001 --timeout 120

# Treat timed-out lanes as non-blocking (not recommended for production)
solidspec ship 001 --ignore-timeout
```

### Clap struct (in `Commands`)

```rust
/// Run concurrent parallel fan-out review and produce a SHIP / HOLD decision
Ship {
    /// Feature ID (e.g., 001) — auto-detected if omitted
    feature_id: Option<String>,

    /// Run only these lanes: comma-separated subset of code,security,tests,perf
    #[arg(long, value_delimiter = ',')]
    lane: Vec<String>,

    /// Exit with code 1 when decision is HOLD (for CI gates)
    #[arg(long)]
    fail_on_hold: bool,

    /// Override agent for the security lane
    #[arg(long)]
    security_agent: Option<String>,

    /// Override agent for the code review lane
    #[arg(long)]
    code_agent: Option<String>,

    /// Override agent for the test coverage lane
    #[arg(long)]
    tests_agent: Option<String>,

    /// Override agent for the performance lane
    #[arg(long)]
    perf_agent: Option<String>,

    /// Use heuristic checks only (no AI agents)
    #[arg(long)]
    no_agent: bool,

    /// Preview planned lanes without executing
    #[arg(long)]
    dry_run: bool,

    /// Per-lane timeout in seconds (default: 300)
    #[arg(long, default_value = "300")]
    timeout: u64,

    /// Treat timed-out lanes as non-blocking
    #[arg(long)]
    ignore_timeout: bool,
},
```

### Terminal output

```
Ship Assessment: 001-task-manager

Launching 4 review lanes (concurrent)...
  ✓ Code Review    (claude)   87/100  done in 14.2s
  ✓ Security Audit (gemini)   91/100  done in 22.5s
  ✗ Test Coverage  (claude)   63/100  done in 11.8s  ← below threshold (70)
  ✗ Performance    (claude)   55/100  done in 16.3s  ← below threshold (60)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Ship Decision: HOLD

  Blocking issues (2 lanes failed thresholds):
  [Tests/HIGH]   3 acceptance scenarios have no test scaffold
  [Tests/HIGH]   tasks.md has 4 tasks not linked to user stories
  [Perf/HIGH]    TaskList entity has no pagination strategy
  [Perf/MEDIUM]  No caching specified for read-heavy task queries
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Report: specs/001-task-manager/ship-report.md
Re-run after fixes: solidspec ship 001 --lane tests,perf
```

### `ship-report.md` format

```markdown
# Ship Report: 001-task-manager

<!-- ship: false -->
<!-- generated: 2026-06-03T14:22:01Z -->

**Decision**: HOLD

## Lane Scores

| Lane | Agent | Score | Threshold | Status |
|------|-------|-------|-----------|--------|
| Code Review | claude | 87/100 | 70 | ✓ Pass |
| Security Audit | gemini | 91/100 | 80 | ✓ Pass |
| Test Coverage | claude | 63/100 | 70 | ✗ Fail |
| Performance | claude | 55/100 | 60 | ✗ Fail |

## Blocking Findings

### Test Coverage

- **[HIGH]** 3 acceptance scenarios in spec.md have no test scaffold
  *Fix*: Run `solidspec tests 001` and ensure each Given/When/Then scenario maps to a test file.

- **[HIGH]** Tasks T006–T009 are not linked to any user story ([US#] tag missing)
  *Fix*: Add `[US1]`, `[US2]`, etc. tags to relevant tasks in tasks.md.

### Performance

- **[HIGH]** TaskList entity has no pagination strategy in plan.md
  *Fix*: Add a pagination section specifying page size and cursor or offset strategy.

- **[MEDIUM]** No caching strategy for read-heavy task queries
  *Fix*: Document cache invalidation policy in plan.md or data-model.md.

## Re-run

```bash
solidspec ship 001 --lane tests,perf
```
```

---

## 7. Configuration

### `solidspec.toml` `[fan_out]` section

```toml
[fan_out]
# Override the agent used per lane. Defaults to [ai].default_agent if omitted.
code_agent     = "claude"
security_agent = "gemini"     # use a security-specialised agent
tests_agent    = "claude"
perf_agent     = "claude"

# Per-lane thresholds (0–100). A lane score below its threshold → HOLD.
code_threshold     = 70
security_threshold = 80
tests_threshold    = 70
perf_threshold     = 60

# Per-lane timeout in seconds.
timeout = 300

# When true, any Critical finding in any lane blocks shipping regardless of score.
block_on_critical = true
```

### `FanOutConfig` Rust struct (corrected)

```rust
// src/config/mod.rs

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FanOutConfig {
    // Agent overrides: None means "use default_agent"
    #[serde(default)]
    pub code_agent: Option<String>,
    #[serde(default)]
    pub security_agent: Option<String>,
    #[serde(default)]
    pub tests_agent: Option<String>,
    #[serde(default)]
    pub perf_agent: Option<String>,

    // Thresholds
    #[serde(default = "default_code_threshold")]
    pub code_threshold: u8,       // 70
    #[serde(default = "default_security_threshold")]
    pub security_threshold: u8,   // 80
    #[serde(default = "default_tests_threshold")]
    pub tests_threshold: u8,      // 70
    #[serde(default = "default_perf_threshold")]
    pub perf_threshold: u8,       // 60

    #[serde(default = "default_fanout_timeout")]
    pub timeout: u64,             // 300

    #[serde(default = "default_true")]
    pub block_on_critical: bool,  // true
}

impl Default for FanOutConfig {
    fn default() -> Self {
        Self {
            code_agent: None,
            security_agent: None,
            tests_agent: None,
            perf_agent: None,
            code_threshold: 70,
            security_threshold: 80,
            tests_threshold: 70,
            perf_threshold: 60,
            timeout: 300,
            block_on_critical: true,
        }
    }
}

fn default_code_threshold() -> u8 { 70 }
fn default_security_threshold() -> u8 { 80 }
fn default_tests_threshold() -> u8 { 70 }
fn default_perf_threshold() -> u8 { 60 }
fn default_fanout_timeout() -> u64 { 300 }
```

> **Correction from original draft**: the fields were typed as `Option<String>` with `#[serde(default = "default_agent_str")]`. A serde default function must return the exact field type — `Option<String>` fields must use `#[serde(default)]` (which defaults to `None`), not a named function returning `String`. The corrected struct uses `Option<String>` with `#[serde(default)]`.

---

## 8. `invoke_agent_with_prompt` — Required Invoker Extension

The existing `invoker::invoke_agent(agent_id, phase, feature_dir_name, project_root, description, context)` builds the prompt internally from a phase name. Fan-out supplies its own specialized prompt, so a new function is required:

```rust
// src/agents/invoker.rs — new public function

/// Invoke an agent with a fully pre-built prompt string.
/// Used by the fan-out engine to supply lane-specific prompts.
pub fn invoke_agent_with_prompt(
    agent_id: &str,
    prompt: &str,
    project_root: &Path,
) -> InvokeResult {
    let agent = match crate::agents::config::find_agent(agent_id) {
        Some(a) => a,
        None => return InvokeResult::NotAvailable {
            reason: format!("Unknown agent '{agent_id}'"),
        },
    };

    if agent.cli_binary.is_empty() {
        return InvokeResult::NotAvailable {
            reason: format!("{} has no CLI binary", agent.name),
        };
    }

    let binary = match crate::agents::registry::find_binary(agent.cli_binary) {
        Some(b) => b,
        None => return InvokeResult::NotAvailable {
            reason: format!("{} binary '{}' not found in PATH", agent.name, agent.cli_binary),
        },
    };

    let mut cmd = std::process::Command::new(&binary);
    cmd.current_dir(project_root)
       .arg(agent.cli_prompt_flag)
       .arg(prompt);
    for flag in agent.cli_extra_flags {
        cmd.arg(flag);
    }

    // Same try_wait timeout loop as invoke_agent()
    run_with_timeout(cmd, /* timeout_secs */ 300)
}
```

This function is the mirror of the existing `invoke_agent` but accepts a pre-built prompt string rather than generating one from a phase name. It reuses the same timeout loop, binary discovery, and `InvokeResult` type.

---

## 9. Progress Reporting via Channel

```rust
// src/core/fan_out.rs — run_fan_out()

pub fn run_fan_out(
    lanes: Vec<ReviewLane>,
    project_root: PathBuf,   // owned PathBuf, not &Path — required for move into thread
    no_agent: bool,
    config: &FanOutConfig,
) -> Vec<LaneResult> {
    let (tx, rx) = std::sync::mpsc::channel::<LaneResult>();

    for lane in lanes {
        let tx = tx.clone();
        let root = project_root.clone();  // clone per thread
        std::thread::spawn(move || {
            let result = run_lane(lane, &root, no_agent);
            tx.send(result).ok();
        });
    }
    drop(tx); // close sender — rx.iter() terminates when all threads finish

    let mut results = Vec::new();
    for result in rx {
        print_lane_result(&result);  // prints as each lane completes
        results.push(result);
    }
    results
}

/// Print a single lane result line to stdout.
fn print_lane_result(r: &LaneResult) {
    let icon = match r.status {
        LaneStatus::Done if r.score >= /* threshold */ 0 => "✓",
        LaneStatus::TimedOut => "⏱",
        LaneStatus::Failed(_) => "✗",
        _ => "✗",
    };
    let status_note = match &r.status {
        LaneStatus::TimedOut => " ← timed out".to_string(),
        LaneStatus::Failed(e) => format!(" ← failed: {e}"),
        LaneStatus::Done => String::new(),
    };
    println!(
        "  {} {:<18} ({})   {}/100  done in {:.1}s{}",
        icon,
        r.lane_label,
        r.agent_id,
        r.score,
        r.duration_ms as f64 / 1000.0,
        status_note
    );
}
```

> **Correction from original draft**: `run_lane(&lane, &project_root, no_agent)` passed a reference into a `move` closure — this is a lifetime error if `project_root` is a local `&Path`. The corrected code uses `PathBuf` (owned) and clones it once per thread.

---

## 10. Schema Integration

```yaml
# Addition to schemas/spec-driven/schema.yaml  (artifact count: 8 → 9)
  - id: ship
    generates: ["ship-report.md"]
    requires: ["analyze", "review"]
    instruction: "Run 'solidspec ship' to execute parallel fan-out review (code, security, tests, performance) and produce a SHIP/HOLD decision. ship-report.md contains per-lane scores and blocking findings."
    template: null
```

The same addition applies to `schemas/intent-driven/schema.yaml` (artifact count: 10 → 11).

**`solidspec ship` is a standalone command, not a pipeline phase.** It is not added to `PHASES` or `PHASES_IDSD` constants. The `ship` DAG artifact lets `solidspec status` show it as `▶ ready` once analyze and review are done, and lets `solidspec pipeline --only ship` trigger it without hardcoding it in the linear phase array.

---

## 11. Phased Development Plan

### Phase 1 — Foundation: types, config, invoker extension *(est. 2h)*

Deliver the data model and invoker extension with no execution logic. All other phases build on this.

| ID | Task | File |
|----|------|------|
| P1-T1 | Add `FanOutConfig` struct (corrected types) + Default impl + unit tests | `src/config/mod.rs` |
| P1-T2 | Add `fan_out: FanOutConfig` to `RootConfig`; round-trip TOML test | `src/config/mod.rs` |
| P1-T3 | Define `ReviewLane`, `LaneResult`, `LaneStatus`, `ShipReport`, `ShipDecision`, `FanOutFinding` | `src/core/fan_out.rs` (new) |
| P1-T4 | Add `pub mod fan_out` | `src/core/mod.rs` |
| P1-T5 | Implement `invoke_agent_with_prompt(agent_id, prompt, project_root)` reusing existing timeout loop | `src/agents/invoker.rs` |
| P1-T6 | Unit test: `invoke_agent_with_prompt` with unknown agent → `NotAvailable`; with agent missing binary → `NotAvailable` | `src/agents/invoker.rs` |
| P1-T7 | Run `cargo fmt` + `cargo clippy -- -D warnings` | — |

**Acceptance criteria:**
- `FanOutConfig` deserializes correctly from TOML with all defaults applied when section is absent
- `pub mod fan_out` compiles with all type definitions present
- `invoke_agent_with_prompt` compiles and passes its unit tests
- All existing 402 tests continue to pass

---

### Phase 2 — Heuristic lane scoring (`--no-agent` mode) *(est. 2h)*

Implement the heuristic lane scoring path using existing `preflight_review()` and `analyze_feature()`. This makes `--no-agent` useful rather than producing placeholder zeros.

| ID | Task | File |
|----|------|------|
| P2-T1 | Implement `score_from_heuristics(lane_id, feature_dir, project_root)` — calls `preflight_review()` + `analyze_feature()`, filters findings by dimension cluster, applies penalty formula | `src/core/fan_out.rs` |
| P2-T2 | Implement `run_lane_no_agent(lane, feature_dir, project_root)` → `LaneResult` | `src/core/fan_out.rs` |
| P2-T3 | Unit tests: code lane from spec with placeholders → score < 100; security lane from spec with auth requirements and no security section → score penalised; all-clean spec → score ≥ threshold | `src/core/fan_out.rs` |
| P2-T4 | Run `cargo fmt` + `cargo clippy -- -D warnings` | — |

**Acceptance criteria:**
- `score_from_heuristics("code", feature_dir, project_root)` produces a `u8` score 0–100 derived from heuristic findings
- A spec with 2 HIGH findings scores 90 (100 − 5 − 5) in the code lane
- A spec with no auth security section scores below 80 in the security lane
- No placeholder zeros — every `--no-agent` run produces a meaningful score

---

### Phase 3 — Agent-based lane execution and score extraction *(est. 3h)*

Implement the agent invocation path and the score extraction fallback chain.

| ID | Task | File |
|----|------|------|
| P3-T1 | Implement `build_lanes(config, feature_dir, default_agent)` — creates 4 `ReviewLane` structs with full prompt strings and thresholds from `FanOutConfig` | `src/core/fan_out.rs` |
| P3-T2 | Implement `extract_score(stdout)` — regex `SCORE:\s*(\d{1,3})` on last 200 chars; fallback: count finding keywords (CRITICAL/HIGH/MEDIUM/LOW) in text and apply penalty | `src/core/fan_out.rs` |
| P3-T3 | Implement `parse_findings_from_output(stdout, lane_id)` — extract `SEVERITY: X\nLOCATION: Y\nPROBLEM: Z\nFIX: W` blocks from agent output | `src/core/fan_out.rs` |
| P3-T4 | Implement `run_lane_with_agent(lane, project_root, timeout_secs)` — calls `invoke_agent_with_prompt`; wraps in `try_wait` loop; parses score and findings; returns `LaneResult` | `src/core/fan_out.rs` |
| P3-T5 | Unit tests: `extract_score("...SCORE: 87")` → 87; `extract_score("no score line")` → fallback derivation; `extract_score("SCORE: 999")` → clamped to 100 | `src/core/fan_out.rs` |
| P3-T6 | Unit test: `parse_findings_from_output` extracts 2 findings from well-formed agent output | `src/core/fan_out.rs` |
| P3-T7 | Run `cargo fmt` + `cargo clippy -- -D warnings` | — |

**Acceptance criteria:**
- `extract_score` correctly parses `SCORE: N` for N in 0–100; clamps out-of-range values; falls back gracefully when the line is absent
- `parse_findings_from_output` returns a `Vec<FanOutFinding>` matching the SEVERITY/LOCATION/PROBLEM/FIX blocks in the output
- `build_lanes` creates exactly 4 lanes with correct IDs, thresholds from config, and non-empty prompt strings

---

### Phase 4 — Fan-out orchestration and ship gate *(est. 2h)*

Wire concurrent execution, channel-based progress reporting, and the aggregation logic.

| ID | Task | File |
|----|------|------|
| P4-T1 | Implement `run_lane(lane, project_root, no_agent, timeout_secs)` — dispatches to `run_lane_with_agent` or `run_lane_no_agent` | `src/core/fan_out.rs` |
| P4-T2 | Implement `run_fan_out(lanes, project_root, no_agent, config)` — thread spawning with `PathBuf` clones, mpsc channel, `drop(tx)`, collect results via `rx` | `src/core/fan_out.rs` |
| P4-T3 | Implement `print_lane_result(result)` — formats completion line including icon, label, agent, score, duration, status note | `src/core/fan_out.rs` |
| P4-T4 | Implement `aggregate_results(lane_configs, results, block_on_critical, ignore_timeout)` → `ShipReport` | `src/core/fan_out.rs` |
| P4-T5 | Implement `format_ship_report(report)` — renders Markdown with `<!-- ship: X -->` header, lane score table, blocking findings per lane, re-run hint | `src/core/fan_out.rs` |
| P4-T6 | Unit tests: all lanes pass → Ship; one below threshold → Hold; Critical in security lane → Hold regardless of score; timeout → Hold; `ignore_timeout = true` + timeout → uses partial results; all fail → Hold with "all reviews unavailable" message | `src/core/fan_out.rs` |
| P4-T7 | Unit test: `format_ship_report` output contains `<!-- ship: false -->`, lane table, and at least one blocking finding section | `src/core/fan_out.rs` |
| P4-T8 | Run `cargo fmt` + `cargo clippy -- -D warnings` | — |

**Acceptance criteria:**
- `run_fan_out` with 4 lanes produces exactly 4 `LaneResult` entries
- Ship gate produces HOLD for each of: below-threshold score, Critical security finding, timed-out lane
- `format_ship_report` produces valid Markdown with machine-readable header
- Thread spawning uses owned `PathBuf` — no lifetime errors

---

### Phase 5 — CLI command, schema, and status integration *(est. 3h)*

Expose the engine as a CLI command and wire it into the DAG artifact system.

| ID | Task | File |
|----|------|------|
| P5-T1 | Implement `cli::ship::run(feature_id, lane_filter, fail_on_hold, no_agent, dry_run, timeout, ignore_timeout, agent_overrides)` | `src/cli/ship.rs` (new) |
| P5-T2 | Dry-run mode: print planned lanes table (ID, agent, threshold) without executing | `src/cli/ship.rs` |
| P5-T3 | `--lane` parsing: split by comma, validate each against `["code", "security", "tests", "perf"]`, error on unknown value | `src/cli/ship.rs` |
| P5-T4 | Write `ship-report.md`; print summary to stdout; `--fail-on-hold` → `std::process::exit(1)` on Hold | `src/cli/ship.rs` |
| P5-T5 | Add `pub mod ship` + `Ship { ... }` variant to `Commands` enum + dispatch arm | `src/cli/mod.rs` |
| P5-T6 | Add `ship` artifact to `spec-driven` schema (count: 8→9) | `schemas/spec-driven/schema.yaml` |
| P5-T7 | Add `ship` artifact to `intent-driven` schema (count: 10→11) | `schemas/intent-driven/schema.yaml` |
| P5-T8 | Update `parse_spec_driven_schema` test: `artifacts.len() == 9`; add `intent_driven_ship_artifact` test | `src/core/schema.rs` |
| P5-T9 | Add `"ship"` to `should_skip()`: skip when `ship-report.md` exists; add `phase_type("ship") → Auto`; add skip reason string | `src/core/pipeline.rs` |
| P5-T10 | Run `cargo fmt` + `cargo clippy -- -D warnings` | — |

**Acceptance criteria:**
- `solidspec ship --dry-run` prints all 4 lanes with agents and thresholds; no files created
- `solidspec ship --lane code` runs only the code lane; output contains "Code Review" and not "Security Audit"
- `ship-report.md` written to `specs/<feature>/ship-report.md` and contains `<!-- ship:`
- `solidspec status 001` shows `ship` as a DAG node with `▶ ready` when analyze + review are done
- Schema artifact count tests pass

---

### Phase 6 — Integration tests *(est. 2h)*

| ID | Test | Description |
|----|------|-------------|
| P6-T1 | `ship_dry_run_shows_all_lanes` | `--dry-run` output contains "Code Review", "Security Audit", "Test Coverage", "Performance" and each threshold |
| P6-T2 | `ship_no_agent_creates_report_with_real_scores` | `--no-agent` creates `ship-report.md` with non-zero scores derived from heuristics, not placeholder zeros |
| P6-T3 | `ship_lane_filter_runs_subset` | `--lane code,security` → report contains exactly 2 lane entries; "Test Coverage" not in output |
| P6-T4 | `ship_fail_on_hold_exits_nonzero` | With a HOLD decision, `--fail-on-hold` returns exit code 1; `--no-fail-on-hold` returns 0 |
| P6-T5 | `ship_report_written_to_feature_dir` | After `solidspec ship --no-agent`, `ship-report.md` exists and contains `<!-- ship:` |
| P6-T6 | `ship_unknown_lane_errors` | `--lane code,unknown` exits with error listing valid lane IDs |
| P6-T7 | `ship_fails_without_spec_md` | Running `solidspec ship` in a project with no `specs/` fails with a clear error |
| P6-T8 | `ship_decision_ship_when_all_lanes_pass` | A clean feature (no findings) → `ship-report.md` contains `<!-- ship: true -->` |
| P6-T9 | `status_shows_ship_artifact_after_review` | After running analyze + review, `solidspec status --schema spec-driven` shows `ship` as `▶ ready` |
| P6-T10 | Run `cargo fmt` + `cargo clippy -- -D warnings` | — |

---

## 12. Non-Goals for v0.4.0

- **NOT** replacing `solidspec analyze` or `solidspec review` — they remain unchanged and independent
- **NOT** adding `"ship"` to `PHASES` or `PHASES_IDSD` arrays — ship is a standalone command, not a linear pipeline phase
- **NOT** streaming agent stdout in real time (agents run to completion; result is posted to channel)
- **NOT** distributed execution across machines (all threads run locally)
- **NOT** sharing intermediate findings between lanes (each lane is fully independent)
- **NOT** a `--format json` output flag (deferred to v0.5.0 pending score extraction experience)
- **NOT** supporting more than 4 lanes for v0.4.0

---

## 13. Open Questions — Resolved

| # | Question | Decision |
|---|----------|----------|
| 1 | Score extraction is fragile | Use `SCORE:\s*(\d{1,3})` regex on last 200 bytes. Fallback: derive from CRITICAL/HIGH/MEDIUM/LOW keyword counts in output. `--format json` deferred to v0.5.0. |
| 2 | Lane weighting vs independent thresholds | Keep independent per-threshold gates for v0.4.0. Simpler reasoning, easier to configure. Lane weighting is a v0.5.0 concern if teams request it. |
| 3 | Partial re-run overwrites or appends ship-report | Always overwrites for v0.4.0. The `<!-- generated: ... -->` timestamp allows audit via git log. |
| 4 | IDSD integration: security lane includes drift score | Out of scope for v0.4.0. The security lane reads spec + plan only. Drift score is available in `analyze` output independently. |

---

## 14. Acceptance Criteria

- [ ] `solidspec ship [id]` runs 4 review lanes concurrently via `std::thread::spawn`
- [ ] Terminal prints each lane result as it completes (real-time via mpsc channel), not all at once
- [ ] `--dry-run` prints planned lanes and thresholds; no files created or modified
- [ ] `--lane code,security` runs only the specified lanes; unknown lane ID → descriptive error
- [ ] Ship gate returns HOLD when any lane score is below its threshold
- [ ] Ship gate returns HOLD when any Critical finding exists in the Security lane
- [ ] A timed-out lane always produces HOLD (unless `--ignore-timeout`)
- [ ] `--no-agent` derives scores from `preflight_review()` heuristics — no placeholder zeros
- [ ] `ship-report.md` written with `<!-- ship: true/false -->` header
- [ ] `--fail-on-hold` exits code 1 on HOLD; exits code 0 on SHIP
- [ ] `[fan_out]` in `solidspec.toml` overrides per-lane agents and thresholds
- [ ] `solidspec status` shows `ship` as a DAG artifact node (▶ ready after analyze + review)
- [ ] `spec-driven` schema has 9 artifacts; `intent-driven` schema has 11 artifacts; schema tests pass
- [ ] `invoke_agent_with_prompt` added to `invoker.rs`; existing `invoke_agent` unchanged
- [ ] At least 18 unit/integration tests (Phase 1–6 tasks above)
- [ ] Passes `cargo clippy -- -D warnings` and `cargo fmt`

---

## 15. Success Metrics

| Metric | Target |
|--------|--------|
| Review wall-clock time vs sequential | ≤ 35% of sequential time (4 lanes in parallel vs 4 lanes in sequence) |
| Heuristic score accuracy (`--no-agent`) | Code lane score ≤ 80 for a spec with ≥ 2 HIGH findings; ≥ 90 for a clean spec |
| Ship gate accuracy | Zero false SHIPs on features with known Critical findings in test suite |
| Report generation time | `ship-report.md` written within 2s of the last lane completing |
| CI integration | `--fail-on-hold` exit code tested in at least one integration test |
| Config coverage | All 4 thresholds and 4 agent overrides tested via unit tests |
