---
type: Rust Function
title: run
resource: src/cli/plan.rs#L13-L247
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/find_project_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/feature/resolve_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/spec_parser/parse_spec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/load_constitution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/task_generator/Task/format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/schema/is_intent_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/intent_parser/parse_intent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/presets/manager/get_preset_priorities
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/resolver/load_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/templates/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/check_plan_compliance
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/constitution/check_intent_constraints
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run(feature_id: Option<&str>, schema: Option<&str>) -> Result<()>`

# Calls

- [find_project_root](../../../../functions/src/config/find_project_root.md)
- [resolve_feature](../../../../functions/src/core/feature/resolve_feature.md)
- [parse_spec](../../../../functions/src/core/spec_parser/parse_spec.md)
- [load_constitution](../../../../functions/src/core/constitution/load_constitution.md)
- [format](../../../../functions/src/core/task_generator/Task/format.md)
- [is_intent_schema](../../../../functions/src/core/schema/is_intent_schema.md)
- [parse_intent](../../../../functions/src/core/intent_parser/parse_intent.md)
- [get_preset_priorities](../../../../functions/src/presets/manager/get_preset_priorities.md)
- [load_template](../../../../functions/src/templates/resolver/load_template.md)
- [render](../../../../functions/src/templates/render.md)
- [check_plan_compliance](../../../../functions/src/core/constitution/check_plan_compliance.md)
- [check_intent_constraints](../../../../functions/src/core/constitution/check_intent_constraints.md)