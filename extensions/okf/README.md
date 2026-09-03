# `okf` extension

Generates an Open Knowledge Format (OKF) knowledge-graph bundle for a project **built with SolidSpec**
(not SolidSpec itself) — a git-native, Markdown+YAML call-graph of the codebase that AI agents can query
instead of re-reading source files cold during `plan`/`analyze`/`review`.

As of `v0.2.0` this is fully native: `solidspec okf generate`/`solidspec okf validate` wrap
[okf-rs](https://github.com/jyjeanne/okf-rs)'s `okf-core`/`okf-analyzer`/`okf-generator`/`okf-validator`
library crates in-process (pinned git dependencies in `Cargo.toml`, see `src/core/okf.rs`) — there is no
external `okf-rs` binary to install or detect. (`okf-rs search`/`explore`/`graph`/`impact`/the `okf-mcp`
server are not vendored — those still need the external CLI; see
[`docs/okf-rs-integration-plan.md`](../../docs/okf-rs-integration-plan.md) for what's still outstanding.)

## What it does

On `solidspec init` (via the `after_init` hook): runs `solidspec okf generate` against the project, writing
the bundle to `.solidspec/knowledge/`, and adds `.okf-cache.json` (the incremental-index cache) to
`.gitignore`.

Best-effort: if `solidspec` itself isn't resolvable from the hook's shell (e.g. run via `cargo run` rather
than an installed binary) or generation fails for any reason, it prints a message and does nothing else —
it never fails `solidspec init`.

## Install (into a project scaffolded by SolidSpec)

```bash
solidspec extension add /path/to/solidspec/extensions/okf --dev
```

Installing it after `solidspec init` has already run won't retroactively fire the hook — either re-run
`solidspec init` in the same directory, or just run what the hook itself would have run:

```bash
solidspec okf generate . --output .solidspec/knowledge
echo '.okf-cache.json' >> .gitignore
```

## Using the bundle

```bash
solidspec okf generate . --output .solidspec/knowledge   # build/refresh the bundle
solidspec okf validate .solidspec/knowledge --ci          # conformance check (CI-gatable)
```

For search/explore/graph queries against the bundle, the external `okf-rs` CLI (or `okf-mcp` for agent
use) is still needed today — `cargo install --git https://github.com/jyjeanne/okf-rs okf-cli`, then
`okf-rs search|explore|graph ... .solidspec/knowledge`. Bringing those in-process too is tracked as
follow-up work in `docs/okf-rs-integration-plan.md`.
