---
type: Rust Function
title: task_story_link_gaps
resource: src/core/review/checks.rs#L656-L661
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/spec_parser/parse_spec_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/check_task_story_links
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn task_story_link_gaps()`

# Calls

- [parse_spec_content](../../../../../functions/src/core/spec_parser/parse_spec_content.md)
- [check_task_story_links](../../../../../functions/src/core/review/checks/check_task_story_links.md)