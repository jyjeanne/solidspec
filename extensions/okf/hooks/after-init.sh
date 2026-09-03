#!/usr/bin/env sh
# okf extension — after_init hook
#
# Scaffolds an okf.toml so this project can generate an Open Knowledge
# Format (OKF) knowledge-graph bundle with okf-rs
# (https://github.com/jyjeanne/okf-rs), which AI agents can then query
# (search/explore/graph/impact, or via okf-mcp) instead of re-reading
# source files cold.
#
# Best-effort only: never fails `solidspec init` — if okf-rs isn't
# installed, or anything else goes wrong, this just skips with a message.
set -eu

PROJECT_ROOT="${PROJECT_ROOT:-.}"

if ! command -v okf-rs >/dev/null 2>&1; then
  echo "okf extension: okf-rs not found on PATH — skipping OKF bundle setup."
  echo "  Install: cargo install --git https://github.com/jyjeanne/okf-rs okf-cli"
  exit 0
fi

if [ -f "$PROJECT_ROOT/okf.toml" ]; then
  echo "okf extension: okf.toml already exists, leaving it as-is."
  exit 0
fi

# --no-agent-files: SolidSpec already manages CLAUDE.md/AGENTS.md/
# .github/copilot-instructions.md via its own agent registry (src/agents/);
# don't let okf-rs also write to them.
okf-rs init "$PROJECT_ROOT" --output .solidspec/knowledge --no-agent-files

GITIGNORE="$PROJECT_ROOT/.gitignore"
if [ -f "$GITIGNORE" ] && ! grep -qxF '.okf-cache.json' "$GITIGNORE"; then
  printf '\n# okf-rs incremental-index cache\n.okf-cache.json\n' >> "$GITIGNORE"
fi

echo "okf extension: wrote okf.toml (bundle output: .solidspec/knowledge)."
echo "  Run 'okf-rs generate' to build the bundle, then 'okf-rs search|explore|graph ...' to query it."
