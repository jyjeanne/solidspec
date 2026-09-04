---
type: Rust Function
title: write_okf_mcp_config
resource: src/cli/init.rs#L172-L202
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/intent_parser/IntentStatus/from_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/init/generate_knowledge_graph_and_mcp_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_okf_mcp_config(project_dir: &Path) -> Result<()>`

# Calls

- [from_str](../../../../functions/src/core/intent_parser/IntentStatus/from_str.md)

# Called by

- [generate_knowledge_graph_and_mcp_config](../../../../functions/src/cli/init/generate_knowledge_graph_and_mcp_config.md)