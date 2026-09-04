---
type: Rust Function
title: unregister_apex_skill
resource: src/agents/registry.rs#L420-L427
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/registry/apex_skill_dir
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/unregister_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_apex_skill_removes_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_apex_skill_is_idempotent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn unregister_apex_skill(agent_id: &str, project_root: &Path) -> Result<()>`

# Calls

- [apex_skill_dir](../../../../functions/src/agents/registry/apex_skill_dir.md)

# Called by

- [unregister_commands](../../../../functions/src/agents/registry/unregister_commands.md)
- [unregister_apex_skill_removes_directory](../../../../functions/src/agents/registry/unregister_apex_skill_removes_directory.md)
- [unregister_apex_skill_is_idempotent](../../../../functions/src/agents/registry/unregister_apex_skill_is_idempotent.md)