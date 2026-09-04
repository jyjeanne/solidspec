---
type: Rust Function
title: render
resource: src/templates/mod.rs#L77-L99
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/checklist/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/generate_constitution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/init/generate_agent_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/intent/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/plan/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/specify/write_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/render_with_all_variables
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/render_empty_arguments_handled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/render_preserves_special_characters_in_markdown
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render(template_str: &str, vars: &HashMap<String, String>) -> Result<String>`

# Called by

- [run](../../../functions/src/cli/checklist/run.md)
- [generate_constitution](../../../functions/src/cli/init/generate_constitution.md)
- [generate_agent_file](../../../functions/src/cli/init/generate_agent_file.md)
- [run](../../../functions/src/cli/intent/run.md)
- [run](../../../functions/src/cli/plan/run.md)
- [write_spec](../../../functions/src/cli/specify/write_spec.md)
- [render_with_all_variables](../../../functions/src/templates/render_with_all_variables.md)
- [render_empty_arguments_handled](../../../functions/src/templates/render_empty_arguments_handled.md)
- [render_preserves_special_characters_in_markdown](../../../functions/src/templates/render_preserves_special_characters_in_markdown.md)