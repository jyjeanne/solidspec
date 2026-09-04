---
type: Rust Module
title: evidence
resource: tests/evidence.rs#L1-L165
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/assert-cmd-command
    resolved_by: tree-sitter
    confidence: exact
  - target: external/predicates-prelude
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

- [setup_project](../../functions/tests/evidence/setup_project.md)
- [write_intent](../../functions/tests/evidence/write_intent.md)
- [write_implemented_test](../../functions/tests/evidence/write_implemented_test.md)
- [evidence_fails_without_intent_md](../../functions/tests/evidence/evidence_fails_without_intent_md.md)
- [evidence_prints_criteria_table_and_writes_report](../../functions/tests/evidence/evidence_prints_criteria_table_and_writes_report.md)
- [evidence_report_contains_criteria_table](../../functions/tests/evidence/evidence_report_contains_criteria_table.md)
- [evidence_update_rewrites_intent_status](../../functions/tests/evidence/evidence_update_rewrites_intent_status.md)
- [status_shows_evidence_artifact_in_idsd_schema](../../functions/tests/evidence/status_shows_evidence_artifact_in_idsd_schema.md)

# Imports

- `assert_cmd::Command`
- `predicates::prelude::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../packages/solidspec.md)