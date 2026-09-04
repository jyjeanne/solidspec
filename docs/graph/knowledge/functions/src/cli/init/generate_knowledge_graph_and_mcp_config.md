---
type: Rust Function
title: generate_knowledge_graph_and_mcp_config
resource: src/cli/init.rs#L144-L166
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/cli/init/write_okf_mcp_config
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/init/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn generate_knowledge_graph_and_mcp_config(project_dir: &Path)`

# Calls

- [write_okf_mcp_config](../../../../functions/src/cli/init/write_okf_mcp_config.md)

# Called by

- [run](../../../../functions/src/cli/init/run.md)