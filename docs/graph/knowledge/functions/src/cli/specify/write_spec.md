---
type: Rust Function
title: write_spec
resource: src/cli/specify.rs#L151-L210
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/schema/is_intent_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/resolver/load_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/validate_spec_quality
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_spec( feature_dir: &std::path::Path, checklists_dir: &std::path::Path, vars: &HashMap<String, String>, schema: &str, project_root: &std::path::Path, preset_priorities: &[(String, u32)], ) -> Result<()>`

# Calls

- [is_intent_schema](../../../../functions/src/core/schema/is_intent_schema.md)
- [load_template](../../../../functions/src/templates/resolver/load_template.md)
- [render](../../../../functions/src/templates/render.md)
- [validate_spec_quality](../../../../functions/src/core/spec_parser/validate_spec_quality.md)