# SolidSpec Project Guidelines

## What This Project Is

SolidSpec is a Rust CLI tool (`solidspec`) implementing Specification-Driven Development (SDD) — it transforms feature descriptions into structured specs, plans, tasks, and test scaffolds, then orchestrates AI agents to implement them.

See [docs/ARCHITECTURE.md](../docs/ARCHITECTURE.md) for full system architecture and data flows.

## Build, Test, Lint, Format

```bash
# Build
cargo build --release

# All tests (inline unit + assert_cmd integration tests)
cargo test

# Single module's tests
cargo test --lib <module_name>   # e.g. cargo test --lib spec_parser

# Run locally
cargo run -- [subcommand]

# Lint (warnings are errors in CI)
cargo clippy -- -D warnings

# Format check (CI gate — no rustfmt.toml, uses defaults)
cargo fmt --check
```

Rust edition: **2024**. No Makefile or justfile — Cargo is the sole build tool.

## Architecture

The codebase has a strict layered separation:

| Layer | Location | Rule |
|-------|----------|------|
| CLI | `src/cli/` | **No business logic** — thin handlers only; delegate to core/agents/templates |
| Domain | `src/core/` | Pure logic, no CLI imports — library-ready |
| Agent integration | `src/agents/` | 21 AI agents, data-driven config, CLI invocation with fallback to handoff |
| Templating | `src/templates/` | Tera rendering; 4-layer resolution (see below) |
| Config | `src/config/` | TOML-based; `RootConfig` + `PipelineConfig` |
| Extensions/Presets | `src/extensions/`, `src/presets/` | CRUD + cross-platform hook execution |

Each `src/cli/` file maps 1-to-1 to a subcommand. Never add business logic there.

## Key Conventions

**Errors:** Use `SolidSpecError` (`src/core/errors.rs`) with `thiserror`. Every variant **except `Validation`** must include a `fix: String` field with a human-actionable suggestion. The `fix` text is shown in the error Display output. Use `anyhow::Result` in fallible functions.

**CLI parsing:** Clap derive macros (`#[derive(Parser)]`). Global `--debug` flag lives on the root `Cli` struct.

**Serialization:** `serde` with `#[derive(Serialize, Deserialize)]` for all config/manifest types.

**Embedded templates:** Use `include_str!()` to embed templates from `templates/` into the binary. Template resolution priority:
```
1. .solidspec/templates/overrides/     ← project-level tweaks (highest)
2. .solidspec/presets/<id>/templates/
3. .solidspec/extensions/<id>/templates/
4. Binary-embedded defaults            ← fallback
```

**Runtime config locations:**
- `solidspec.toml` — project config (root)
- `.solidspec/` — constitution, templates, extensions, presets, internal state
- `.solidspec/project-config.json` — `ProjectInternalConfig` (internal state, not user-facing)

**Feature resolution** cascades: explicit CLI arg → env var → current git branch → latest `specs/` directory.

**Tests:** Unit tests live in inline `#[cfg(test)] mod tests` blocks in each source file (`cargo test --bin solidspec`). Integration tests that invoke the CLI binary live in `tests/` (`cargo test --test <name>`), using `assert_cmd` + `predicates` + `tempfile`. Tests using `Command::cargo_bin("solidspec")` **must** be in `tests/` — `CARGO_BIN_EXE_solidspec` is only set there.

**Shell scripts:** Bash/PS scripts in `scripts/` are **not standalone** — they are embedded in the binary via `include_str!()` in `src/templates/mod.rs` and extracted at runtime.

## Module Organization

- Max 2 directory levels under `src/`
- `mod.rs` re-exports public symbols; sub-files are focused on a single concern
- `src/core/` modules: `spec_parser`, `task_generator`, `test_generator`, `constitution`, `analyzer`, `review`, `git`, `feature`, `errors`, `token`, `vscode`, `pipeline`, `artifact_graph` (DAG engine, Kahn's algorithm), `schema` (workflow schema loading, 3-level resolution), `change` (delta spec parser, archive merge)

## Documentation

Don't duplicate — link:
- Architecture details → [docs/ARCHITECTURE.md](../docs/ARCHITECTURE.md)
- SDD methodology → [docs/solid-specification.md](../docs/solid-specification.md)
- Pipeline orchestration → [docs/multi-agent-pipeline.md](../docs/multi-agent-pipeline.md)
- Test scaffold generation → [docs/spec-to-test-generation.md](../docs/spec-to-test-generation.md)
- Feature backlog → [docs/KILLER_FEATURE_IDEAS.md](../docs/KILLER_FEATURE_IDEAS.md)
