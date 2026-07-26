#!/usr/bin/env bash
# Regenerate the codebase knowledge graph in docs/graph/ using graphify
# (https://github.com/Graphify-Labs/graphify).
#
# Extraction is pure local tree-sitter AST parsing (--code-only): no LLM
# calls, no API key, nothing leaves the machine.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/docs/graph"

if ! command -v graphify >/dev/null 2>&1; then
  echo "error: graphify not found on PATH." >&2
  echo "Install it with: uv tool install graphifyy   (or: pipx install graphifyy)" >&2
  exit 1
fi

STAGING="$(mktemp -d)"
trap 'rm -rf "$STAGING"' EXIT

graphify extract "$ROOT" --code-only --cargo --out "$STAGING"
graphify cluster-only "$STAGING" --no-label

mkdir -p "$OUT"
# The report header names the staging dir; rewrite it to the repo name.
sed -i "s|$STAGING|solidspec|g" "$STAGING/graphify-out/GRAPH_REPORT.md"
cp "$STAGING/graphify-out/graph.json" \
   "$STAGING/graphify-out/graph.html" \
   "$STAGING/graphify-out/GRAPH_REPORT.md" \
   "$STAGING/graphify-out/manifest.json" \
   "$STAGING/graphify-out/.graphify_analysis.json" \
   "$OUT/"

echo "Knowledge graph regenerated in docs/graph/"
echo "  - docs/graph/graph.json        queryable graph (graphify query/explain/affected --graph docs/graph/graph.json)"
echo "  - docs/graph/graph.html        interactive visualization (open in a browser)"
echo "  - docs/graph/GRAPH_REPORT.md   summary report"
