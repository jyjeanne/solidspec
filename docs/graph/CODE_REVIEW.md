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

### 3. `register_commands` bypasses the template system with ~250 lines of inline prompts
`src/agents/registry.rs:69-330` embeds all ~10 slash-command bodies as inline `format!` strings inside one giant match — while the project ships a Tera template system (`src/templates/`) with `include_str!` embedding and 3-level project-local overrides, plus a data-table precedent (`src/agents/personas.rs`). The graph flags `register_commands()` as a god node (20 edges) and `src/agents/registry.rs` (923 lines) as its own low-cohesion community. Users also cannot override command bodies the way they can override templates.

**Action:** move command bodies into embedded templates (or a static table like `personas.rs`), leaving `register_commands` as pure orchestration over `formats.rs`. Bonus: project-local overrides of command prompts come for free via `templates/resolver.rs`.

### 4. `fan_out.rs` (1,476 lines) mixes four concerns; lane prompts are ~70% copy-paste
The four lane prompts (`code_review_prompt`, `security_audit_prompt`, `test_coverage_prompt`, `performance_prompt`, `src/core/fan_out.rs:272-359`) share the same frame (context preamble, SEVERITY/LOCATION/PROBLEM/FIX block, scoring rubric) and differ only in the focus bullet list. The file also contains lane orchestration, score parsing, aggregation, and report formatting. `FanOutConfig` is one of the top cross-community bridge nodes (betweenness 0.034) — the file couples otherwise-unrelated communities. The scoring rubric ("10 per CRITICAL, 5 per HIGH…") is additionally duplicated in `apply_penalty_formula` and `derive_score_from_keywords`.

**Action:** one prompt-builder taking a focus-bullet list + lane name; single source of truth for the penalty weights; split report formatting into its own module.

### 5. Integration-test helpers copy-pasted across 6 files
The graph's #1 and #2 god nodes are **test helpers**, not production code: `init_project()` / `solidspec()` / `create_feature()` are re-implemented in `tests/apex.rs`, `tests/ship.rs`, `tests/tdd.rs`, `tests/change.rs`, `tests/traceability.rs`, `tests/pipeline.rs` (53 + 29 edges on the two `init_project` variants alone). Any change to `solidspec init` output ripples through six copies.

**Action:** extract `tests/common/mod.rs` with the shared helpers (standard Cargo pattern for integration tests).

### 6. `src/core/review.rs` (1,392 lines): monolithic check pipeline
`preflight_review` (hub, 22 edges) drives ten independent `check_*` functions plus scoring plus report formatting in one file. Checks are self-contained (each returns `Vec<ReviewFinding>`), which is good — but they're not discoverable or extensible, even though the project has an extensions system (`src/extensions/`).

**Action:** split into `review/checks.rs` + `review/report.rs`; register checks in a slice of fn pointers so adding one is a one-line change (and a future extension point).

### 7. Duplicated agent-binary resolution between registry and invoker
`check_cli_available` (`src/agents/registry.rs:487`) and `supports_cli` / binary lookup in `src/agents/invoker.rs` both resolve agent CLIs via `find_binary`; the graph shows 19 INFERRED edges converging on `find_agent()` from both modules (flagged in the analysis's "verify inferred" questions). `invoke_agent` and `invoke_agent_with_prompt` also duplicate the spawn/poll/timeout loop.

**Action:** single `resolve_agent_cli(agent) -> Option<PathBuf>` used by both modules; collapse the two invoke variants into one with a timeout parameter.

---

## P3 — Hygiene

### 8. `.opencode/skills/ai-spec-review-skill` duplicates the Rust review logic in Python
112 graph nodes come from `.opencode/` Python scripts (`review_spec.py`: `detect_security_gaps()`, etc.) that re-implement checks that exist in `src/core/review.rs`. Two implementations will drift.

**Action:** decide on one source of truth — either the skill shells out to `solidspec review`, or it's documented as intentionally independent.

### 9. Shell scripts are isolated graph nodes with no test coverage
`scripts/check-prerequisites.sh`, `common.sh`, `create-new-feature.sh` appear as weakly-connected components (analysis "isolated_nodes" question). They're embedded via `include_str!` so the graph can't see their consumers — but nothing in CI exercises them either.

**Action:** add a `shellcheck` step to CI and at least one integration test that runs a copied script.

### 10. `#[allow(dead_code)]` accumulation
11+ `#[allow(dead_code)]` markers (`src/core/change.rs`, `src/config/mod.rs`, `src/agents/invoker.rs:378`, `src/core/artifact_graph.rs:24`, …). Each one hides a field/function the graph also sees as low-degree. Periodically remove the allow and delete what no longer compiles.

### 11. Keep the graph fresh
`docs/graph/GRAPH_REPORT.md` records the commit it was built from. Regenerate with `./scripts/generate-graph.sh` after structural changes, or install git hooks (`graphify hook install`) to update automatically. A stale graph gives stale review signals.

---

## Suggested order of work

| # | Item | Effort | Payoff | Status |
|---|------|--------|--------|--------|
| 1 | Schema-driven `should_skip` (P1.1) | M | Fixes silent re-runs for custom workflows, deletes duplication | ✅ Fixed |
| 2 | De-panic `status` (P1.2) | S | User-input errors stop crashing the CLI | ✅ Fixed |
| 3 | Shared test helpers (P2.5) | S | Removes the two biggest god nodes in the graph | |
| 4 | Command bodies → templates (P2.3) | M | Unlocks user-overridable prompts, shrinks registry.rs | |
| 5 | Fan-out prompt/scoring dedup (P2.4) | M | Single source of truth for scoring rubric | |
| 6 | Split review.rs (P2.6) | M | Extensible check registry | |
| 7 | Agent CLI resolution dedup (P2.7) | S | Removes inferred-edge ambiguity the graph flagged | |
| 8+ | P3 items | S each | Opportunistic | |
