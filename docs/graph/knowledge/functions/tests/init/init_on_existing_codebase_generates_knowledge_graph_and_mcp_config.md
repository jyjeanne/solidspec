---
type: Rust Function
title: init_on_existing_codebase_generates_knowledge_graph_and_mcp_config
resource: tests/init.rs#L121-L151
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/init/with_claude_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/IntentStatus/from_str
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn init_on_existing_codebase_generates_knowledge_graph_and_mcp_config()`

# Calls

- [with_claude_dir](../../../functions/tests/init/with_claude_dir.md)
- [from_str](../../../functions/src/core/intent_parser/IntentStatus/from_str.md)