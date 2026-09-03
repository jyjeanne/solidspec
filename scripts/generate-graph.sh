#!/usr/bin/env bash
# Regenerate the codebase knowledge graph in docs/graph/ using okf-rs
# (https://github.com/jyjeanne/okf-rs).
#
# Extraction is pure local tree-sitter AST parsing: no LLM calls, no API
# key, nothing leaves the machine. The bundle is ordinary Markdown files
# with YAML frontmatter, so regeneration diffs are readable in a PR like
# any other file change.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/docs/graph"
BUNDLE="$OUT/knowledge"

if ! command -v okf-rs >/dev/null 2>&1; then
  echo "error: okf-rs not found on PATH." >&2
  echo "Install it with: cargo install --git https://github.com/jyjeanne/okf-rs okf-cli" >&2
  exit 1
fi

okf-rs generate "$ROOT" -o "$BUNDLE"
okf-rs validate "$BUNDLE" --ci

mkdir -p "$OUT"
{
  echo "# Graph Report - solidspec ($(date -u +%Y-%m-%d))"
  echo
  echo "Built from commit: \`$(git -C "$ROOT" rev-parse --short HEAD)\`"
  echo
  echo '## Topology (`okf-rs graph stats`)'
  echo
  echo '```'
  okf-rs graph stats "$BUNDLE"
  echo '```'
  echo
  echo '## Coverage (`okf-rs coverage`)'
  echo
  echo '```'
  okf-rs coverage "$BUNDLE"
  echo '```'
} > "$OUT/GRAPH_REPORT.md"

echo "Knowledge graph regenerated in docs/graph/"
echo "  - docs/graph/knowledge/       OKF bundle: one Markdown+YAML file per concept, cross-linked"
echo "  - docs/graph/GRAPH_REPORT.md  topology + coverage summary"
echo
echo "Query it with okf-rs search/explore/graph/diff/impact --output docs/graph/knowledge"
echo "(or just -o/positional bundle arg — see 'okf-rs <subcommand> --help')."
