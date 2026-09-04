#!/usr/bin/env sh
# okf extension — after_init hook
#
# Generates an Open Knowledge Format (OKF) knowledge-graph bundle for this
# project using SolidSpec's own native `solidspec okf generate` command —
# no external `okf-rs` binary involved (it's vendored in-process as pinned
# library crates, see src/core/okf.rs and Cargo.toml). AI agents can then
# be pointed at the bundle (.solidspec/knowledge/) instead of re-reading
# source files cold.
#
# Best-effort only: never fails `solidspec init` — if `solidspec` itself
# somehow isn't resolvable (e.g. run via `cargo run`/a relative path
# rather than an installed binary), or generation fails for any reason,
# this just skips with a message.
set -eu

PROJECT_ROOT="${PROJECT_ROOT:-.}"

if ! command -v solidspec >/dev/null 2>&1; then
  echo "okf extension: 'solidspec' not found on PATH — skipping OKF bundle generation."
  echo "  Run 'solidspec okf generate' manually once it is."
  exit 0
fi

if solidspec okf generate "$PROJECT_ROOT" --output "$PROJECT_ROOT/.solidspec/knowledge"; then
  GITIGNORE="$PROJECT_ROOT/.gitignore"
  if [ -f "$GITIGNORE" ] && ! grep -qxF '.okf-cache.json' "$GITIGNORE"; then
    printf '\n# okf-rs incremental-index cache\n.okf-cache.json\n' >> "$GITIGNORE"
  fi
else
  echo "okf extension: 'solidspec okf generate' failed — see output above; continuing."
fi
