#!/usr/bin/env bash
# Regenerate the codebase knowledge graph in docs/graph/ using SolidSpec's
# own native `solidspec okf generate`/`validate` commands (src/core/okf.rs)
# — okf-rs's generator/validator crates are vendored as pinned git
# dependencies (Cargo.toml), so no external okf-rs binary is involved.
#
# Extraction is pure local tree-sitter AST parsing: no LLM calls, no API
# key, nothing leaves the machine. The bundle is ordinary Markdown files
# with YAML frontmatter, so regeneration diffs are readable in a PR like
# any other file change.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/docs/graph"
BUNDLE="$OUT/knowledge"

cargo build --quiet --manifest-path "$ROOT/Cargo.toml"
SOLIDSPEC="$ROOT/target/debug/solidspec"

# Clean rebuild: write_bundle only ever writes/updates files for concepts
# that still exist, so a renamed or removed function's old file would
# otherwise linger and turn up as a stale "orphaned concept" warning on the
# next `validate --ci`. The incremental-index *cache* (.okf-cache.json,
# untouched here) still skips re-parsing unchanged source files — only the
# already-cheap bundle write is redone from scratch.
rm -rf "$BUNDLE"

generate_output="$("$SOLIDSPEC" okf generate "$ROOT" --output "$BUNDLE")"
echo "$generate_output"
"$SOLIDSPEC" okf validate "$BUNDLE" --ci

mkdir -p "$OUT"
{
  echo "# Graph Report - solidspec ($(date -u +%Y-%m-%d))"
  echo
  echo "Built from commit: \`$(git -C "$ROOT" rev-parse --short HEAD)\`"
  echo
  echo '## Topology (`solidspec okf generate`)'
  echo
  echo '```'
  echo "$generate_output"
  echo '```'
} > "$OUT/GRAPH_REPORT.md"

echo "Knowledge graph regenerated in docs/graph/"
echo "  - docs/graph/knowledge/       OKF bundle: one Markdown+YAML file per concept, cross-linked"
echo "  - docs/graph/GRAPH_REPORT.md  topology summary"
echo
echo "search/explore/graph/diff/impact queries still need the external okf-rs CLI"
echo "(cargo install --git https://github.com/jyjeanne/okf-rs okf-cli) — see"
echo "docs/okf-rs-integration-plan.md for what's vendored natively so far."
