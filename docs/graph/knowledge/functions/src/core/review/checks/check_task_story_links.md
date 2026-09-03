---
type: Rust Function
title: check_task_story_links
resource: src/core/review/checks.rs#L291-L323
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/review/preflight_review
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/review/checks/task_story_link_gaps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn check_task_story_links( spec: &spec_parser::ParsedSpec, tasks_content: &str, ) -> Vec<ReviewFinding>`

# Called by

- [preflight_review](../../../../../functions/src/core/review/preflight_review.md)
- [task_story_link_gaps](../../../../../functions/src/core/review/checks/task_story_link_gaps.md)