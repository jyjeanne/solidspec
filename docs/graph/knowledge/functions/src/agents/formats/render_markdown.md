---
type: Rust Function
title: render_markdown
resource: src/agents/formats.rs#L24-L26
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/agents/formats/render_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/markdown_has_frontmatter_delimiters
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render_markdown(description: &str, body: &str) -> String`

# Called by

- [render_command](../../../../functions/src/agents/formats/render_command.md)
- [markdown_has_frontmatter_delimiters](../../../../functions/src/agents/formats/markdown_has_frontmatter_delimiters.md)