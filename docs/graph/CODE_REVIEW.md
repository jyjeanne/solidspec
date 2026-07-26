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

### 1. `should_skip` silently never skips custom-schema artifacts
`src/core/pipeline.rs:102` hardcodes a phase→file match (`"specify" => spec.md`, …) ending in `_ => false`. Meanwhile every schema already declares its outputs in `generates:` (`schemas/*/schema.yaml`), and `ArtifactGraph` (`src/core/artifact_graph.rs`) already does filesystem completion detection from those declarations. Consequences:
- Any artifact from a custom schema (`.solidspec/workflows/<name>/schema.yaml`) falls through to `_ => false`, so completed phases are **re-run on every pipeline invocation** — silently, and expensive when phases invoke an agent CLI.
- Two parallel completion mechanisms (pipeline skip list vs. artifact-graph state) can disagree; the graph shows `pipeline.rs` and `schema.rs`/`artifact_graph.rs` in separate low-cohesion communities bridged only through the CLI layer.

**Action:** derive default skip behavior from `SchemaArtifact.generates` (file exists → skip), keeping the special cases (`clarify`, `implement`, `analyze`, `apex`) as content-aware overrides. Delete the duplicated mapping.

### 2. Panic paths in `status` rendering
`src/cli/status.rs:49-63` — when `topological_order()` fails (cycle in a user-authored schema), the fallback rebuilds the list with `graph.nodes.get(id).unwrap()`, and each row uses `expect("artifact missing from states map")`. A malformed custom schema is exactly the case where these invariants are weakest, and it turns a user input error into a panic. Same pattern inside Kahn's algorithm itself (`src/core/artifact_graph.rs:100-115` — acceptable as internal invariants, but worth an error message).

**Action:** in `status`, surface the cycle to the user ("workflow schema has a dependency cycle: …") instead of silently falling back, and render missing state as `unknown` rather than panicking.

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

| # | Item | Effort | Payoff |
|---|------|--------|--------|
| 1 | Schema-driven `should_skip` (P1.1) | M | Fixes silent re-runs for custom workflows, deletes duplication |
| 2 | De-panic `status` (P1.2) | S | User-input errors stop crashing the CLI |
| 3 | Shared test helpers (P2.5) | S | Removes the two biggest god nodes in the graph |
| 4 | Command bodies → templates (P2.3) | M | Unlocks user-overridable prompts, shrinks registry.rs |
| 5 | Fan-out prompt/scoring dedup (P2.4) | M | Single source of truth for scoring rubric |
| 6 | Split review.rs (P2.6) | M | Extensible check registry |
| 7 | Agent CLI resolution dedup (P2.7) | S | Removes inferred-edge ambiguity the graph flagged |
| 8+ | P3 items | S each | Opportunistic |
