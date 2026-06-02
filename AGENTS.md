# AGENTS.md

## Build, test, lint, format

```bash
cargo build --release     # build
cargo test                # all tests (inline #[cfg(test)] + assert_cmd integrations)
cargo clippy -- -D warnings  # gate in CI — warnings are errors
cargo fmt --check            # gate in CI — default rustfmt, no rustfmt.toml
```

No Makefile, no justfile — Cargo is the only task runner.

## Architecture constraints

- **Strict layering**: `src/cli/` must contain **no business logic** — thin clap handlers only. All logic lives in `src/core/`, which must **never import from `src/cli/`**.
- **Max 2 directory levels** under `src/`. Each `src/cli/*.rs` maps to one subcommand.
- **Single crate** (no workspace), Rust **edition 2024**.

## Error convention

`SolidSpecError` (`src/core/errors.rs:6`) — every variant except `Validation` **must** include a `fix: String` field with a human-actionable suggestion. The `fix` appears in the error Display output.

## Template system

- Templates are embedded in the binary via `include_str!()` from `templates/`.
- Rendered with **Tera** (autoescaping **disabled** — output is Markdown, not HTML).
- 4-layer resolution: `overrides/` > `presets/<id>/` > `extensions/<id>/` > embedded defaults.

## Feature ID resolution (4-level cascade)

1. Explicit CLI argument
2. `SOLIDSPEC_FEATURE` env var
3. Current git branch (if matches `\d{3}-*`)
4. Latest `specs/` directory

## Agent config

20+ AI agents are defined as a **data-driven `const` table** in `src/agents/config.rs`. To add a new agent, edit that table — no code changes needed elsewhere.

## Tests

- **Unit tests**: inline `#[cfg(test)] mod tests` blocks inside each source file.
- **Integration tests**: in `tests/` directory (`tests/change.rs`, `tests/check.rs`, `tests/completions.rs`, `tests/pipeline.rs`, `tests/status.rs`), using `assert_cmd` + `predicates` + `tempfile`. These require `CARGO_BIN_EXE_solidspec` which is only available to files in `tests/`.
- Run a single test file: `cargo test --test <name>` (e.g., `cargo test --test pipeline`).
- Run inline unit tests: `cargo test --bin solidspec`.

## Shell scripts

Bash/PS scripts in `scripts/` are **not standalone** — they are embedded in the binary via `include_str!()` in `src/templates/mod.rs` and extracted at runtime.

## Documentation references

- Architecture: `docs/ARCHITECTURE.md`
- Copilot-specific instructions: `.github/copilot-instructions.md`
- Agent command definitions: `.github/agents/*.agent.md`
