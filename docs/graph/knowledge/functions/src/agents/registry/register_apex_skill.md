---
type: Rust Function
title: register_apex_skill
resource: src/agents/registry.rs#L388-L396
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/registry/apex_skill_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/extract_skill
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/register_all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_apex_skill_creates_files_for_claude
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/register_apex_skill_returns_false_for_unsupported_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_apex_skill_removes_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_apex_skill_directory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn register_apex_skill(agent_id: &str, project_root: &Path) -> Result<bool>`

# Calls

- [apex_skill_dir](../../../../functions/src/agents/registry/apex_skill_dir.md)
- [extract_skill](../../../../functions/src/core/apex/extract_skill.md)

# Called by

- [register_all](../../../../functions/src/agents/registry/register_all.md)
- [register_apex_skill_creates_files_for_claude](../../../../functions/src/agents/registry/register_apex_skill_creates_files_for_claude.md)
- [register_apex_skill_returns_false_for_unsupported_agent](../../../../functions/src/agents/registry/register_apex_skill_returns_false_for_unsupported_agent.md)
- [unregister_apex_skill_removes_directory](../../../../functions/src/agents/registry/unregister_apex_skill_removes_directory.md)
- [unregister_removes_apex_skill_directory](../../../../functions/src/agents/registry/unregister_removes_apex_skill_directory.md)