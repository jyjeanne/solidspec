---
type: Rust Module
title: config
resource: src/config/mod.rs#L1-L527
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-core-errors-solidspecerror
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

- [RootConfig](../../classes/src/config/RootConfig.md)
- [ProjectConfig](../../classes/src/config/ProjectConfig.md)
- [AiConfig](../../classes/src/config/AiConfig.md)
- [default](../../functions/src/config/AiConfig/default/default.md)
- [GitConfig](../../classes/src/config/GitConfig.md)
- [default](../../functions/src/config/GitConfig/default/default.md)
- [TemplatesConfig](../../classes/src/config/TemplatesConfig.md)
- [default](../../functions/src/config/TemplatesConfig/default/default.md)
- [ContextConfig](../../classes/src/config/ContextConfig.md)
- [ContextRules](../../classes/src/config/ContextRules.md)
- [PipelineConfig](../../classes/src/config/PipelineConfig.md)
- [default](../../functions/src/config/PipelineConfig/default/default.md)
- [agent_for_phase](../../functions/src/config/PipelineConfig/agent_for_phase.md)
- [validate](../../functions/src/config/PipelineConfig/validate.md)
- [default_version](../../functions/src/config/default_version.md)
- [default_agent](../../functions/src/config/default_agent.md)
- [default_true](../../functions/src/config/default_true.md)
- [default_override_dir](../../functions/src/config/default_override_dir.md)
- [default_schema](../../functions/src/config/default_schema.md)
- [default_code_threshold](../../functions/src/config/default_code_threshold.md)
- [default_security_threshold](../../functions/src/config/default_security_threshold.md)
- [default_tests_threshold](../../functions/src/config/default_tests_threshold.md)
- [default_perf_threshold](../../functions/src/config/default_perf_threshold.md)
- [default_fanout_timeout](../../functions/src/config/default_fanout_timeout.md)
- [FanOutConfig](../../classes/src/config/FanOutConfig.md)
- [default](../../functions/src/config/FanOutConfig/default/default.md)
- [new](../../functions/src/config/RootConfig/new.md)
- [load](../../functions/src/config/RootConfig/load.md)
- [save](../../functions/src/config/RootConfig/save.md)
- [ProjectInternalConfig](../../classes/src/config/ProjectInternalConfig.md)
- [CatalogList](../../classes/src/config/CatalogList.md)
- [save](../../functions/src/config/ProjectInternalConfig/save.md)
- [InitOptions](../../classes/src/config/InitOptions.md)
- [save](../../functions/src/config/InitOptions/save.md)
- [find_project_root](../../functions/src/config/find_project_root.md)
- [project_default_schema](../../functions/src/config/project_default_schema.md)
- [new_config_has_correct_defaults](../../functions/src/config/new_config_has_correct_defaults.md)
- [round_trip_serialize_deserialize](../../functions/src/config/round_trip_serialize_deserialize.md)
- [load_valid_config](../../functions/src/config/load_valid_config.md)
- [load_malformed_toml_returns_error](../../functions/src/config/load_malformed_toml_returns_error.md)
- [load_missing_file_returns_error](../../functions/src/config/load_missing_file_returns_error.md)
- [save_and_reload](../../functions/src/config/save_and_reload.md)
- [defaults_when_optional_fields_missing](../../functions/src/config/defaults_when_optional_fields_missing.md)
- [find_project_root_finds_solidspec_toml](../../functions/src/config/find_project_root_finds_solidspec_toml.md)
- [find_project_root_returns_none_at_root](../../functions/src/config/find_project_root_returns_none_at_root.md)
- [fanout_config_defaults_when_section_absent](../../functions/src/config/fanout_config_defaults_when_section_absent.md)
- [fanout_config_round_trips_toml](../../functions/src/config/fanout_config_round_trips_toml.md)

# Imports

- `std::path::{Path, PathBuf}`
- `anyhow::{Context, Result}`
- `serde::{Deserialize, Serialize}`
- `crate::core::errors::SolidSpecError`
- `super::*`
- `tempfile::TempDir`

# Member of

- [solidspec](../../packages/solidspec.md)