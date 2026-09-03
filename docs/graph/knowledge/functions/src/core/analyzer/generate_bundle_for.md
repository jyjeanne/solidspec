---
type: Rust Function
title: generate_bundle_for
resource: src/core/analyzer.rs#L901-L905
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/core/analyzer/structural_cross_check_flags_unknown_backtick_symbol
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/structural_cross_check_accepts_a_real_symbol
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/structural_cross_check_flags_existing_file_missing_from_a_stale_bundle
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/structural_cross_check_ignores_files_that_do_not_exist_yet
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/core/analyzer/format_report_includes_structural_section_with_a_bundle
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn generate_bundle_for(project_root: &Path) -> std::path::PathBuf`

# Called by

- [structural_cross_check_flags_unknown_backtick_symbol](../../../../functions/src/core/analyzer/structural_cross_check_flags_unknown_backtick_symbol.md)
- [structural_cross_check_accepts_a_real_symbol](../../../../functions/src/core/analyzer/structural_cross_check_accepts_a_real_symbol.md)
- [structural_cross_check_flags_existing_file_missing_from_a_stale_bundle](../../../../functions/src/core/analyzer/structural_cross_check_flags_existing_file_missing_from_a_stale_bundle.md)
- [structural_cross_check_ignores_files_that_do_not_exist_yet](../../../../functions/src/core/analyzer/structural_cross_check_ignores_files_that_do_not_exist_yet.md)
- [format_report_includes_structural_section_with_a_bundle](../../../../functions/src/core/analyzer/format_report_includes_structural_section_with_a_bundle.md)