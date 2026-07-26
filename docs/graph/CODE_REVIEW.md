# Graph-Driven Code Review — Prioritized Improvements

Generated from the graphify knowledge graph (`docs/graph/graph.json`, 1,601 nodes / 3,597 edges / 81 communities, built from commit `3263089`) combined with targeted code reading. Grounding checks: `cargo clippy --all-targets` passes clean, full `cargo test` suite passes.

How the graph was used:
- `graphify god-nodes` → architectural hubs (highest-degree nodes)
- `.graphify_analysis.json` → low-cohesion communities, cross-community bridge nodes, isolated nodes
- `graphify affected` / `explain` → blast radius of the hub symbols
- Node/edge mining of `graph.json` → cross-module coupling, duplicate symbols

Severity scale: **P1** = correctness/behavior risk, fix first · **P2** = structural debt with real maintenance cost · **P3** = hygiene, fix opportunistically.

---

## P1 — Correctness / behavior risk

### 1. ✅ FIXED — `should_skip` silently never skipped custom-schema artifacts
`src/core/pipeline.rs` hardcoded a phase→file match ending in `_ => false`. Custom/overridden artifact `generates` declarations (`schemas/*/schema.yaml`, `.solidspec/workflows/<name>/schema.yaml`) were silently ignored, so overridden or unmatched artifacts were **re-run on every pipeline invocation**.

**Fix applied:** `should_skip` now takes the resolved `&ArtifactGraph` and, for any phase without a dedicated content-aware check (`clarify`, `implement`, `apex`, `analyze`, `tdd-tests` — kept as overrides because file existence alone is insufficient or actively wrong for them), defaults to `ArtifactGraph::generates_present`, a new helper factored out of `detect_completion`'s glob/dir/file matching. `src/cli/pipeline.rs` now loads the schema graph once per run and passes it through. Regression test: `pipeline_dry_run_respects_custom_schema_generates_override` (`tests/pipeline.rs`) — a project-local override adding `research.md` to the `plan` artifact's `generates` is now honored (phase reruns until the extra file exists). New unit tests: `should_skip_tests_when_dir_nonempty_via_schema_generates`, `should_skip_unknown_phase_defaults_false_when_absent_from_schema` (`src/core/pipeline.rs`).

*Follow-up not in scope of this fix:* `filter_phases` still selects the phase-name list by a hardcoded match on the schema **name** (`spec-driven`/`intent-driven`/`apex-driven`/`intent-apex`/`tdd-driven`), not from the resolved schema's own artifact list — so a schema with a genuinely new phase name (e.g. `security-first`'s `security-review`) still can't be driven through `solidspec pipeline`, and `execute_phase`'s exhaustive dispatch has no generic executor for an arbitrary artifact id. Making the whole pipeline schema-driven end-to-end is a larger, separate change.

### 2. ✅ FIXED — panic paths in `status` rendering
`src/cli/status.rs` used `graph.nodes.get(id).unwrap()` in the cycle fallback and `.expect("artifact missing from states map")` per row — a malformed custom schema (dependency cycle) turned a user input error into a panic instead of a message.

**Fix applied:** on a topological-order error, `status` now prints a warning to stderr (`Warning: schema '<name>' has an invalid dependency graph (<cause>); showing artifacts in unspecified order.`) and continues instead of crashing; missing state entries render as `? unknown` instead of panicking. Regression test: `status_warns_instead_of_panicking_on_cyclic_schema` (`tests/status.rs`) — a project-local schema with a `spec ↔ plan` cycle now exits successfully with a warning and a populated table.

---

## P2 — Structural debt (graph hotspots)

### 3. ✅ FIXED — `register_commands` bypassed the template system with ~250 lines of inline prompts
`src/agents/registry.rs` embedded all ~11 slash-command bodies as inline `format!` strings inside one giant match. Users could not override command bodies the way they can override spec/plan/tasks templates.

**Fix applied:** the 11 named-phase bodies now live as `include_str!`-embedded files under `templates/commands/<phase>.md` (`specify.md`, `clarify.md`, `plan.md`, `tasks.md`, `implement.md`, `tests.md`, `analyze.md`, `review.md`, `apex.md`, `tdd-tests.md`, `tdd-refactor.md`), each containing the canonical `$ARGUMENTS` placeholder. `register_commands` now calls a single `command_body(cmd_name, project_root)` that returns the right body — reducing the match in `registry.rs` from a ~250-line inline-`format!` block to one line per phase. Bodies were extracted by running the actual pre-refactor binary and capturing byte-exact output (not hand-transcribed), then verified with a full `diff -rq` between pre- and post-refactor `init` output for both a Markdown agent (claude) and a TOML agent (gemini, exercising the `{{args}}` placeholder path) — output is byte-identical. Net: registry.rs -122 lines.

**Bonus delivered:** `command_body()` checks `.solidspec/templates/overrides/commands/<phase>.md` first — a project can now override any command's body, documented in CLAUDE.md. This is a single-layer override (not the full preset/extension resolver chain in `templates/resolver.rs`, which would have required threading `preset_priorities` through `register_all`/`init`/`upgrade` call sites — out of scope for this fix). Regression tests: `project_local_override_wins_over_embedded_command_body`, `no_override_falls_back_to_embedded_default`, `command_body_generic_fallback_for_unknown_phase` (`src/agents/registry.rs`).

### 4. ✅ FIXED — `fan_out.rs` mixed four concerns; lane prompts were ~70% copy-paste
The four lane prompts (`code_review_prompt`, `security_audit_prompt`, `test_coverage_prompt`, `performance_prompt`) shared the same frame (context preamble, SEVERITY/LOCATION/PROBLEM/FIX block, scoring rubric) and differed only in the focus bullet list. The scoring rubric ("10 per CRITICAL, 5 per HIGH…") was additionally duplicated in `apply_penalty_formula` and `derive_score_from_keywords`.

**Fix applied:**
- Replaced the four prompt functions with one `lane_prompt(feat, spec: &LaneSpec)` builder plus a `LANE_SPECS: &[LaneSpec]` table holding only what differs per lane (title, focus bullets, "not assessed" phrase, finding noun, problem/fix hints, score aspect). `build_lanes` now maps over `LANE_SPECS` instead of four repeated `ReviewLane { ... }` literals. Verified byte-identical prompt output against the pre-refactor functions (captured via a temporary test, diffed, then removed) for all 4 lanes.
- Added `penalty_weight(severity) -> f64` as the single source of truth for the scoring rubric; both `apply_penalty_formula` (heuristic path) and `derive_score_from_keywords` (agent-output fallback path) now call it instead of each hardcoding the CRITICAL/HIGH/MEDIUM/LOW weights.
- Split `format_ship_report` into `src/core/fan_out/report.rs` (adjacent submodule, re-exported as `fan_out::format_ship_report` so `cli/ship.rs` is unchanged), separating report rendering from lane orchestration/scoring/aggregation.

All existing fan_out tests (37) plus the ship integration suite pass unchanged — this was a pure refactor, no behavior change.

### 5. ✅ FIXED — integration-test helpers copy-pasted across 6 files
The graph's #1 and #2 god nodes were **test helpers**, not production code: `init_project()` / `solidspec()` / `create_feature()` were re-implemented in `tests/apex.rs`, `tests/ship.rs`, `tests/tdd.rs`, `tests/change.rs`, `tests/traceability.rs`, `tests/pipeline.rs` (53 + 29 edges on the two `init_project` variants alone).

**Fix applied:** added `tests/common/mod.rs` (standard Cargo convention — not compiled as its own test binary) exporting `solidspec()`, `init_project()` (the CI-robust variant that pre-creates `.claude/`, since the divergence was a real robustness gap, not just style drift), and `first_feature_dir()`. `apex.rs`/`ship.rs`/`tdd.rs` now import these directly; `change.rs`/`traceability.rs`/`pipeline.rs` keep their file-local convenience wrappers (different call signatures — e.g. `solidspec(dir)` binding `current_dir` upfront, or a combined init+specify helper) but those wrappers now delegate to `common::solidspec()` internally instead of duplicating `Command::cargo_bin(...)`. Repeated inline "find first feature dir under specs/" `read_dir` blocks (7 occurrences across `pipeline.rs`/`ship.rs`/`traceability.rs`) were replaced with `common::first_feature_dir()`. Net: -103 lines across the 6 files; all 646 tests still pass.

`create_feature()` was deliberately **not** unified — each suite's fixture content differs (different `plan.md`/`tasks.md`/`spec.md` bodies with suite-specific markers like `[US1]` or acceptance-criteria sections that other tests assert against), so merging them risked silently changing fixtures other tests depend on.

### 6. ✅ FIXED (partial) — `src/core/review.rs` was a monolithic check pipeline
`preflight_review` (hub, 22 edges) drove ten independent `check_*` functions plus scoring plus report formatting all in one 1,392-line file.

**Fix applied:** split into `src/core/review/checks.rs` (the 10 heuristic checks + their regexes, `pub(crate)`) and `src/core/review/report.rs` (`format_review_report`, re-exported so `crate::core::review::format_review_report` — used by `cli/review.rs` — is unchanged). `review.rs` itself now holds only the type definitions, `preflight_review` orchestration, and `score_dimensions`. Tests were split alongside their functions (checks.rs's own suite grew from 0 to 12 tests moved from the old monolithic test module; report.rs got 1; review.rs kept the 9 orchestration/integration tests). All 22 tests still pass, same count as before — pure relocation, no behavior change. Net: 1 file (1,392 lines) → 3 files (533 + 799 + 160 lines), each with a single, obvious responsibility.

**Not done:** the "slice of fn pointers" check registry. The 10 checks have genuinely heterogeneous signatures (`fn(&str) -> Vec<ReviewFinding>`, `fn(&ParsedSpec) -> Vec<ReviewFinding>`, `fn(&ParsedSpec, &str, &str) -> Vec<ReviewFinding>`, and `review_intent_alignment` returns a different type entirely, `(Vec<ReviewFinding>, f64)`). Forcing them into one dynamic-dispatch registry would mean wrapping every call site in a boxed closure inside `preflight_review` purely to satisfy a uniform signature — more indirection for a fixed, small set of checks that isn't a plugin surface today. The file split already delivers the stated discoverability goal (each check is now easy to find, read, and unit-test in isolation); the registry is deferred as a genuine future item if/when checks need to become pluggable via `src/extensions/`.

### 7. ✅ FIXED — duplicated agent-binary resolution between registry and invoker
`check_cli_available` (`src/agents/registry.rs`) and `supports_cli` / binary lookup in `src/agents/invoker.rs` both resolved agent CLIs via `find_binary`. `invoke_agent` and `invoke_agent_with_prompt` also duplicated the spawn/poll/timeout loop.

**Investigation turned up more than expected:** `check_cli_available` had **zero callers anywhere in the codebase** — not even its own tests. It was pure dead code, invisible to clippy's `dead_code` lint only because `registry.rs` has a blanket `#![allow(dead_code)]`. Deleted outright rather than merged.

**Fix applied:**
- `invoker.rs` gained `resolve_agent_cli(agent_id) -> Result<(&AgentConfig, PathBuf), String>`, consolidating the unknown-agent / no-CLI-support / binary-not-found checks that were previously repeated verbatim in both `invoke_agent` and `invoke_agent_with_prompt`. `supports_cli` now is just `resolve_agent_cli(agent_id).is_ok()`.
- `run_agent_cli` (interactive, stdio-inherited so the agent's output streams live to the terminal) and `run_agent_cli_capture` (used by parallel fan-out lanes, which pipe+buffer stdout in a reader thread specifically to avoid interleaving concurrent agents' output and to let the caller parse SCORE/SEVERITY from it) were **not** collapsed into one function — they have a genuinely different, deliberate stdio strategy for different call contexts (foreground single agent vs. background parallel lanes), confirmed by reading how each is invoked. Instead, the two pieces that actually were identical — building the per-agent argument list (codex/kimi/default cases) and the spawn-poll-timeout loop — were extracted into shared `build_agent_args()` and `wait_with_timeout()` helpers used by both.
- Removed a stale `#[allow(dead_code)]` on `invoke_agent_with_prompt` left over from before `ship`/fan-out was wired into the binary — it's genuinely called now.

Net: -64 lines across `invoker.rs` + `registry.rs`. All 649 tests pass unchanged.

---

## P3 — Hygiene

### 8. ✅ RESOLVED — `.opencode/skills/ai-spec-review-skill` vs. Rust review logic
112 graph nodes come from `.opencode/` Python scripts (`review_spec.py`: `detect_security_gaps()`, etc.) that looked, from the graph alone, like a duplicate of `src/core/review.rs`'s checks.

**Investigation:** the skill directory has its own `LICENSE`, `README.md`, and `CONTRIBUTING.md` — it's a vendored, independently-versioned (v2.1.0) third-party skill, not code this project authored. It performs a 16-dimension AI-agent-driven review (security, architecture, dependencies, UX, ...) with no knowledge of SolidSpec's artifact conventions, genuinely different in kind from `solidspec review`'s fast, deterministic, no-LLM heuristics scoped to SDD artifacts. Making it shell out to `solidspec review` would mean rewriting a vendored third-party tool to match a narrower, unrelated purpose.

**Resolution:** documented as intentionally independent in CLAUDE.md ("Vendored agent skills vs. `solidspec review`" section) rather than merged.

### 9. ✅ FIXED — shell scripts had no direct test coverage, no shellcheck gate
`scripts/check-prerequisites.sh`, `common.sh`, `create-new-feature.sh` appeared as weakly-connected graph components — the graph can't see `Command::new("bash")` invocations, so this was a partial false positive: `create-new-feature.sh` and two `common.sh` functions already had execution tests in `src/templates/mod.rs`'s `bash_execution` test module. The real gaps were `check-prerequisites.sh`, `setup-plan.sh`, and `update-agent-context.sh`, which had none, and no shellcheck gate in CI.

**Fix applied:** added 5 new tests in `src/templates/mod.rs::tests::bash_execution` exercising all three previously-uncovered scripts (pass/fail paths for `check-prerequisites.sh`, file creation + idempotency for `setup-plan.sh`, feature-status listing for `update-agent-context.sh`). Added a `shellcheck` job to `.github/workflows/ci.yml` (`shellcheck -x --severity=warning scripts/bash/*.sh scripts/generate-graph.sh`) — verified clean locally before adding the gate.

### 10. ✅ AUDITED — `#[allow(dead_code)]` accumulation
Systematically audited all 28 occurrences (16 file-level blanket `#![allow(dead_code)]`, 12 item-level) by removing each and rebuilding to see exactly what surfaced, rather than guessing.

**Deleted outright** (confirmed genuinely dead — zero callers anywhere, including tests): `ArtifactGraph::artifact_ids()`, `ContextConfig`'s unused prompt-formatting methods (`as_prompt_section`/`rules_for_phase`/`as_phase_prompt` — the config struct's data is loaded from `solidspec.toml` but this half-built prompt-injection feature was never wired up), `DeltaSpec.raw` and `ChangeInfo.dir` (write-only fields, small blast radius), `agents::config::all_agent_ids()`, `intent_parser::EvidenceCriterion` (superseded by `evidence::EvidenceCriterionResult`), `TestFramework.slug_style` (genuinely vestigial — JS/TS `describe`/`it` blocks are generated from the raw story title directly, never through `slugify`, so the field-driven style switch was never actually consulted).

**Deduplicated instead of just allowed:** `fan_out::run_lane_no_agent` was reimplementing `score_from_heuristics` inline instead of calling it — now calls it, making the allow unnecessary and removing ~10 duplicated lines.

**Kept with a narrowed, documented allow** (real functionality, tested, just not yet wired to a caller — each now says exactly why and names the likely future consumer): `SchemaInfo`/`WorkflowSchema::builtin::names()`/`list_available_schemas()` (untested-by-CLI schema listing, candidate `solidspec schema list`), `ExtensionRegistry::get`/`update`, `agents::registry::unregister_apex_skill`/`unregister_commands` (candidate `solidspec agent remove`), `AgentConfig.requires_cli`, `ArtifactNode.instruction`/`.template` (tied to the already-noted P1.1 schema-driven-pipeline follow-up), `cli::ux`'s whole `Step`/`StepTracker` progress-UI module, `core::vscode`'s `.vscode/settings.json` merger, `PhaseStatus::Pending`/`Running`, `tdd::RedReport`/`parse_red_report`, `core::token::resolve_github_token`, `IntentSpec`'s mostly-unused fields, `Constitution.raw`/`Gate.article`/`.checks`, `DeltaModification.previous_text`, `errors::SolidSpecError::Init`, `SlugStyle::Preserved`.

Net: file-level blanket allows down from 16 to 2 (both now module-doc-commented, not silent); every remaining allow states in one line why the code exists and isn't used yet. All 654 tests pass; this was a mix of safe deletions and documentation, no behavior changes.

### 11. Keep the graph fresh
`docs/graph/GRAPH_REPORT.md` records the commit it was built from. Regenerate with `./scripts/generate-graph.sh` after structural changes, or install git hooks (`graphify hook install`) to update automatically. A stale graph gives stale review signals.

---

## Suggested order of work

| # | Item | Effort | Payoff | Status |
|---|------|--------|--------|--------|
| 1 | Schema-driven `should_skip` (P1.1) | M | Fixes silent re-runs for custom workflows, deletes duplication | ✅ Fixed |
| 2 | De-panic `status` (P1.2) | S | User-input errors stop crashing the CLI | ✅ Fixed |
| 3 | Shared test helpers (P2.5) | S | Removes the two biggest god nodes in the graph | ✅ Fixed |
| 4 | Command bodies → templates (P2.3) | M | Unlocks user-overridable prompts, shrinks registry.rs | ✅ Fixed |
| 5 | Fan-out prompt/scoring dedup (P2.4) | M | Single source of truth for scoring rubric | ✅ Fixed |
| 6 | Split review.rs (P2.6) | M | Extensible check registry | ✅ Fixed (file split done; fn-pointer registry deferred, see note) |
| 7 | Agent CLI resolution dedup (P2.7) | S | Removes inferred-edge ambiguity the graph flagged | ✅ Fixed |
| 8+ | P3 items | S each | Opportunistic | |
