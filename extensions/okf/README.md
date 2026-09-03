# `okf` extension

Scaffolds an `okf.toml` for [okf-rs](https://github.com/jyjeanne/okf-rs) so a project **built with SolidSpec**
can generate its own Open Knowledge Format (OKF) knowledge-graph bundle — a git-native, Markdown+YAML
call-graph of the codebase that AI agents can query (`okf-rs search`/`explore`/`graph`, or via `okf-mcp`)
instead of re-reading source files cold during `plan`/`analyze`/`review`.

This is step 2 of the plan in [`docs/okf-rs-integration-plan.md`](../../docs/okf-rs-integration-plan.md):
scaffolding only, opt-in, never on the critical path. It does not touch SolidSpec's own pipeline phases —
later steps in that plan cover wiring the bundle into `plan`/`analyze`/`review`.

## What it does

On `solidspec init` (via the `after_init` hook), if `okf-rs` is on `PATH`:

- Writes `okf.toml` (via `okf-rs init --no-agent-files`, bundle output at `.solidspec/knowledge/`)
- Adds `.okf-cache.json` (okf-rs's incremental-index cache) to `.gitignore`

If `okf-rs` isn't installed, or `okf.toml` already exists, it prints a message and does nothing else — it
never fails `solidspec init`. It doesn't generate the bundle itself; run `okf-rs generate` yourself when
you're ready (see below).

## Install (into a project scaffolded by SolidSpec)

```bash
# One-time: install okf-rs itself (not on crates.io — install from git)
cargo install --git https://github.com/jyjeanne/okf-rs okf-cli

# From inside your SolidSpec project, pointing at wherever you have this
# extension's source (e.g. a checkout of the solidspec repo):
solidspec extension add /path/to/solidspec/extensions/okf --dev
```

Installing it after `solidspec init` has already run won't retroactively fire the hook — either re-run
`solidspec init` in the same directory, or just run the two commands the hook itself would have run:

```bash
okf-rs init . --output .solidspec/knowledge --no-agent-files
echo '.okf-cache.json' >> .gitignore
```

## Using the bundle

```bash
okf-rs generate                                          # build/refresh the bundle
okf-rs search "some symbol" --ranked                      # find a concept
okf-rs explore <concept-id>                                # signature, callers, callees, blast radius
okf-rs graph stats                                         # topology overview
okf-rs impact <base-ref> HEAD                              # structural blast-radius before a refactor PR
```

`okf-rs <subcommand> --help` covers the full surface (`validate`, `diff`, `review`, `docs`, the `okf-mcp`
server, ...).
