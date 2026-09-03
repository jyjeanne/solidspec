---
type: Rust Function
title: extract_skill
resource: src/core/apex.rs#L66-L129
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/apex/write_script
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/agents/registry/register_apex_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/extract_skill_creates_all_directories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/extract_skill_writes_all_step_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/extract_skill_writes_all_template_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/extract_skill_writes_scripts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/apex/extract_skill_is_idempotent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn extract_skill(target_dir: &Path) -> Result<()>`

# Calls

- [write_script](../../../../functions/src/core/apex/write_script.md)

# Called by

- [register_apex_skill](../../../../functions/src/agents/registry/register_apex_skill.md)
- [extract_skill_creates_all_directories](../../../../functions/src/core/apex/extract_skill_creates_all_directories.md)
- [extract_skill_writes_all_step_files](../../../../functions/src/core/apex/extract_skill_writes_all_step_files.md)
- [extract_skill_writes_all_template_files](../../../../functions/src/core/apex/extract_skill_writes_all_template_files.md)
- [extract_skill_writes_scripts](../../../../functions/src/core/apex/extract_skill_writes_scripts.md)
- [extract_skill_is_idempotent](../../../../functions/src/core/apex/extract_skill_is_idempotent.md)