---
type: Rust Module
title: test_generator
resource: src/core/test_generator.rs#L1-L859
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-lazylock
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-spec-parser-parsedspec
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-spec-parser-parsedspec-userstory
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

- [AcceptanceScenario](../../../classes/src/core/test_generator/AcceptanceScenario.md)
- [extract_scenarios](../../../functions/src/core/test_generator/extract_scenarios.md)
- [parse_single_scenario](../../../functions/src/core/test_generator/parse_single_scenario.md)
- [FrameworkId](../../../classes/src/core/test_generator/FrameworkId.md)
- [TestFramework](../../../classes/src/core/test_generator/TestFramework.md)
- [SlugStyle](../../../classes/src/core/test_generator/SlugStyle.md)
- [detect_framework](../../../functions/src/core/test_generator/detect_framework.md)
- [framework_from_name](../../../functions/src/core/test_generator/framework_from_name.md)
- [slugify](../../../functions/src/core/test_generator/slugify.md)
- [render_test_file](../../../functions/src/core/test_generator/render_test_file.md)
- [render_jest](../../../functions/src/core/test_generator/render_jest.md)
- [render_pytest](../../../functions/src/core/test_generator/render_pytest.md)
- [render_cargo](../../../functions/src/core/test_generator/render_cargo.md)
- [render_go](../../../functions/src/core/test_generator/render_go.md)
- [render_generic](../../../functions/src/core/test_generator/render_generic.md)
- [test_file_name](../../../functions/src/core/test_generator/test_file_name.md)
- [sample_spec](../../../functions/src/core/test_generator/sample_spec.md)
- [extract_scenarios_from_spec](../../../functions/src/core/test_generator/extract_scenarios_from_spec.md)
- [scenario_splits_given_when_then](../../../functions/src/core/test_generator/scenario_splits_given_when_then.md)
- [scenario_with_missing_when_skipped](../../../functions/src/core/test_generator/scenario_with_missing_when_skipped.md)
- [empty_spec_returns_no_scenarios](../../../functions/src/core/test_generator/empty_spec_returns_no_scenarios.md)
- [detect_jest_from_package_json](../../../functions/src/core/test_generator/detect_jest_from_package_json.md)
- [detect_typescript_with_tsconfig](../../../functions/src/core/test_generator/detect_typescript_with_tsconfig.md)
- [detect_cargo_test](../../../functions/src/core/test_generator/detect_cargo_test.md)
- [detect_pytest](../../../functions/src/core/test_generator/detect_pytest.md)
- [detect_go_test](../../../functions/src/core/test_generator/detect_go_test.md)
- [detect_generic_when_no_files](../../../functions/src/core/test_generator/detect_generic_when_no_files.md)
- [framework_from_name_override](../../../functions/src/core/test_generator/framework_from_name_override.md)
- [slugify_underscores](../../../functions/src/core/test_generator/slugify_underscores.md)
- [slugify_strips_special_chars](../../../functions/src/core/test_generator/slugify_strips_special_chars.md)
- [slugify_preserved_keeps_spaces](../../../functions/src/core/test_generator/slugify_preserved_keeps_spaces.md)
- [slugify_max_80_chars](../../../functions/src/core/test_generator/slugify_max_80_chars.md)
- [slugify_long_multibyte_title_does_not_panic](../../../functions/src/core/test_generator/slugify_long_multibyte_title_does_not_panic.md)
- [test_file_name_long_multibyte_title_does_not_panic](../../../functions/src/core/test_generator/test_file_name_long_multibyte_title_does_not_panic.md)
- [jest_template_valid_syntax](../../../functions/src/core/test_generator/jest_template_valid_syntax.md)
- [pytest_template_valid_syntax](../../../functions/src/core/test_generator/pytest_template_valid_syntax.md)
- [cargo_template_valid_syntax](../../../functions/src/core/test_generator/cargo_template_valid_syntax.md)
- [go_template_valid_syntax](../../../functions/src/core/test_generator/go_template_valid_syntax.md)
- [generic_template_valid](../../../functions/src/core/test_generator/generic_template_valid.md)
- [test_file_name_format](../../../functions/src/core/test_generator/test_file_name_format.md)
- [test_file_name_truncated](../../../functions/src/core/test_generator/test_file_name_truncated.md)

# Imports

- `std::path::Path`
- `std::sync::LazyLock`
- `regex::Regex`
- `super::spec_parser::ParsedSpec`
- `super::*`
- `crate::core::spec_parser::{ParsedSpec, UserStory}`
- `tempfile::TempDir`

# Member of

- [solidspec](../../../packages/solidspec.md)