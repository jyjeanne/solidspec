---
type: Rust Module
title: evidence
resource: src/core/evidence.rs#L1-L397
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-intent-parser-self-intentstatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/solidspec
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [EvidenceCriterionResult](../../../classes/src/core/evidence/EvidenceCriterionResult.md)
- [EvidenceReport](../../../classes/src/core/evidence/EvidenceReport.md)
- [collect_evidence](../../../functions/src/core/evidence/collect_evidence.md)
- [update_intent_status](../../../functions/src/core/evidence/update_intent_status.md)
- [format_evidence_report](../../../functions/src/core/evidence/format_evidence_report.md)
- [write_intent](../../../functions/src/core/evidence/write_intent.md)
- [write_test_file](../../../functions/src/core/evidence/write_test_file.md)
- [baseline_all_not_implemented](../../../functions/src/core/evidence/baseline_all_not_implemented.md)
- [satisfied_criterion_detected](../../../functions/src/core/evidence/satisfied_criterion_detected.md)
- [satisfaction_rate_100_gives_satisfied_status](../../../functions/src/core/evidence/satisfaction_rate_100_gives_satisfied_status.md)
- [low_satisfaction_gives_drifted_status](../../../functions/src/core/evidence/low_satisfaction_gives_drifted_status.md)
- [no_tests_dir_returns_baseline](../../../functions/src/core/evidence/no_tests_dir_returns_baseline.md)
- [update_intent_status_rewrites_status_line](../../../functions/src/core/evidence/update_intent_status_rewrites_status_line.md)
- [update_intent_status_preserves_trailing_newline](../../../functions/src/core/evidence/update_intent_status_preserves_trailing_newline.md)
- [format_report_contains_table_and_header](../../../functions/src/core/evidence/format_report_contains_table_and_header.md)

# Imports

- `std::path::Path`
- `anyhow::Result`
- `super::intent_parser::{self, IntentStatus}`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)