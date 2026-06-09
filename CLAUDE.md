# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release

# Test (all)
cargo test

# Test (single test by name)
cargo test test_name

# Test (single integration test file)
cargo test --test pipeline

# Lint
cargo clippy -- -D warnings

# Format check
cargo fmt --check

# Run the CLI
cargo run -- <subcommand>
```

The project enforces `nonstandard_style = deny` and `redundant_clone = deny` via `Cargo.toml` lints. Clippy must pass cleanly.

## Architecture

SolidSpec is a Rust CLI tool (Rust 2024 edition) that scaffolds Specification-Driven Development (SDD) workflows. It generates artifact files that AI coding agents fill in, guided by slash commands registered per-agent.

### Module layout

- **`src/cli/`** — One module per subcommand. `mod.rs` defines `Cli` (clap `Parser`) and the `Commands` enum, then dispatches to each module's `run()` function.
- **`src/core/`** — All business logic, no I/O dependencies on CLI layer:
  - `artifact_graph.rs` — DAG engine (Kahn's algorithm) for artifact dependency resolution and completion detection via filesystem.
  - `schema.rs` — Loads workflow schemas (YAML → `WorkflowSchema` → `ArtifactGraph`). 3-level resolution: project-local `.solidspec/workflows/<name>/schema.yaml` → built-in → default.
  - `pipeline.rs` — Phase list constants, skip logic (checks for existing artifact files), phase type (auto vs. handoff), and log writing.
  - `feature.rs` — Feature ID resolution (4-level: explicit arg → `SOLIDSPEC_FEATURE` env var → git branch pattern `\d{3}-.*` → latest `specs/` dir).
  - `spec_parser.rs`, `task_generator.rs`, `test_generator.rs`, `analyzer.rs`, `review.rs` — Parse and generate the per-phase artifacts.
  - `constitution.rs` — Reads `.solidspec/constitution.md` for architecture guardrails.
- **`src/agents/`** — Manages 20 AI agent integrations:
  - `config.rs` — Static `AGENTS` table mapping agent IDs to their command directory, file format (`Markdown`/`Toml`), CLI binary, argument placeholder, etc.
  - `registry.rs` — Detects agents in a repo, writes/deletes command files with per-agent format differences (flat `.md`/`.toml` for most; directory-based `SKILL.md` for Kimi/Vibe/OpenCode; dual `.agent.md` + `.prompt.md` for Copilot).
  - `invoker.rs` — Executes the agent CLI non-interactively for automated pipeline runs.
  - `formats.rs` — Renders command file bodies for each format; `guardrails.rs` appends a compliance footer to all command bodies.
- **`src/config/`** — `solidspec.toml` (`RootConfig`) and `.solidspec/config.toml` (`ProjectInternalConfig`). `find_project_root()` walks up from cwd looking for `solidspec.toml` or `.solidspec/`.
- **`src/templates/`** — Tera (Jinja2-compatible) rendering. Templates and shell scripts are `include_str!`-embedded in the binary at compile time. `resolver.rs` applies a 3-level override: project-local → embedded default. Templates are never overwritten on copy; scripts always overwrite.
- **`src/extensions/`**, **`src/presets/`** — Plugin/preset catalog systems, loaded from project-internal config.

### Workflow schemas

Four built-in schemas in `schemas/`:
- `spec-driven` (default) — 8 artifacts: spec → clarify → plan → tasks → tests → implement → analyze → review
- `minimal` — 4 artifacts
- `security-first` — adds a `security-review` artifact between plan and tasks
- `intent-driven` — IDSD mode, adds `intent` as phase 0 before spec

Custom schemas live at `.solidspec/workflows/<name>/schema.yaml`.

### Feature directories

All feature artifacts live under `specs/<NNN>-<slug>/` (e.g., `specs/001-auth-system/`). Key files per feature: `spec.md`, `plan.md`, `tasks.md`, `tests/`, `analysis-report.md`, `review-report.md`, optionally `intent.md`.

### Agent command registration

`solidspec init` auto-detects which agents are present (by checking their config dir or CLI binary) and writes slash-command files to each agent's command directory. The `implement` phase is always a `Handoff` (requires human confirmation in interactive pipeline runs); all other phases are `Auto`.

### Testing

Unit tests live in `#[cfg(test)]` blocks inside each source file and use `tempfile::TempDir` for isolated filesystem state. Integration tests are in `tests/pipeline.rs` and use `assert_cmd` + `predicates` to run the compiled binary end-to-end.
