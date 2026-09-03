---
type: Rust Function
title: unregister_commands
resource: src/agents/registry.rs#L295-L354
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/agents/formats/kimi_command_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/formats/standard_command_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_apex_skill
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/unregister_removes_copilot_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_kimi_dirs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_vibe_dirs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_opencode_skills
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_apex_skill_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/agents/registry/unregister_removes_claude_spcx_directory_contents
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn unregister_commands(project_root: &Path, agent: &AgentConfig) -> Result<()>`

# Calls

- [kimi_command_name](../../../../functions/src/agents/formats/kimi_command_name.md)
- [standard_command_name](../../../../functions/src/agents/formats/standard_command_name.md)
- [unregister_apex_skill](../../../../functions/src/agents/registry/unregister_apex_skill.md)

# Called by

- [unregister_removes_copilot_files](../../../../functions/src/agents/registry/unregister_removes_copilot_files.md)
- [unregister_removes_kimi_dirs](../../../../functions/src/agents/registry/unregister_removes_kimi_dirs.md)
- [unregister_removes_vibe_dirs](../../../../functions/src/agents/registry/unregister_removes_vibe_dirs.md)
- [unregister_removes_opencode_skills](../../../../functions/src/agents/registry/unregister_removes_opencode_skills.md)
- [unregister_removes_apex_skill_directory](../../../../functions/src/agents/registry/unregister_removes_apex_skill_directory.md)
- [unregister_removes_claude_spcx_directory_contents](../../../../functions/src/agents/registry/unregister_removes_claude_spcx_directory_contents.md)