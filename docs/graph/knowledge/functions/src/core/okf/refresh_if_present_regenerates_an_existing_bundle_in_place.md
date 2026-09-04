---
type: Rust Function
title: refresh_if_present_regenerates_an_existing_bundle_in_place
resource: src/core/okf.rs#L210-L232
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/core/okf/write_sample_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/okf/refresh_if_present
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn refresh_if_present_regenerates_an_existing_bundle_in_place()`

# Calls

- [write_sample_project](../../../../functions/src/core/okf/write_sample_project.md)
- [refresh_if_present](../../../../functions/src/core/okf/refresh_if_present.md)