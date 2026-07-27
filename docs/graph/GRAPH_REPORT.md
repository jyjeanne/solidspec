# Graph Report - solidspec  (2026-07-27)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 1665 nodes · 3776 edges · 86 communities (71 shown, 15 thin omitted)
- Extraction: 93% EXTRACTED · 7% INFERRED · 0% AMBIGUOUS · INFERRED: 255 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `ad06724c`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Community 0
- Community 1
- Community 2
- Community 3
- Community 4
- Community 5
- Community 6
- Community 7
- Community 8
- Community 9
- Community 10
- Community 11
- Community 12
- Community 13
- Community 14
- Community 15
- Community 16
- Community 17
- Community 18
- Community 19
- Community 20
- Community 21
- Community 22
- Community 23
- Community 24
- Community 25
- Community 26
- Community 27
- Community 28
- Community 29
- Community 30
- Community 31
- Community 32
- Community 33
- Community 34
- Community 35
- Community 36
- Community 37
- Community 38
- Community 39
- Community 40
- Community 41
- Community 42
- Community 43
- Community 44
- Community 45
- Community 46
- Community 47
- Community 48
- Community 49
- Community 50
- Community 51
- Community 53
- Community 54
- Community 55
- Community 56
- Community 57
- Community 58
- Community 59
- Community 60
- Community 61
- Community 62
- Community 63
- Community 64
- Community 65
- Community 66
- Community 67
- Community 68
- Community 70
- Community 71
- Community 72
- Community 73
- Community 74
- Community 75
- Community 76

## God Nodes (most connected - your core abstractions)
1. `solidspec()` - 121 edges
2. `init_project()` - 106 edges
3. `first_feature_dir()` - 30 edges
4. `find_agent()` - 25 edges
5. `create_feature()` - 24 edges
6. `register_commands()` - 23 edges
7. `graph_for()` - 23 edges
8. `ParsedSpec` - 21 edges
9. `parse_intent_content()` - 20 edges
10. `create_feature()` - 20 edges

## Surprising Connections (you probably didn't know these)
- `tdd_tests_appears_in_help()` --calls--> `solidspec()`  [INFERRED]
  tests/tdd.rs → tests/traceability.rs
- `tdd_refactor_appears_in_help()` --calls--> `solidspec()`  [INFERRED]
  tests/tdd.rs → tests/traceability.rs
- `tdd_tests_help_shows_dry_run_flag()` --calls--> `solidspec()`  [INFERRED]
  tests/tdd.rs → tests/traceability.rs
- `tdd_refactor_help_shows_dry_run_flag()` --calls--> `solidspec()`  [INFERRED]
  tests/tdd.rs → tests/traceability.rs
- `tdd_tests_fails_outside_project()` --calls--> `solidspec()`  [INFERRED]
  tests/tdd.rs → tests/traceability.rs

## Import Cycles
- None detected.

## Communities (86 total, 15 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.05
Nodes (84): Child, ExitStatus, AgentConfig, find_agent(), Option, compliance_footer(), compliance_footer_contains_both_sections(), String (+76 more)

### Community 1 - "Community 1"
Cohesion: 0.06
Nodes (62): aggregate_results(), all_lanes_failed_returns_hold_with_message(), all_lanes_pass_returns_ship(), all_lanes_timed_out_with_ignore_timeout_returns_hold(), apply_penalty_formula(), build_lanes(), build_lanes_creates_four_lanes(), build_lanes_prompts_contain_feature_name() (+54 more)

### Community 2 - "Community 2"
Cohesion: 0.08
Nodes (57): fire_hooks(), fire_hooks_skips_disabled_extensions(), fire_hooks_skips_missing_file(), Path, add_dev_installs_extension(), add_extension_dev(), add_without_manifest_errors(), build_entry() (+49 more)

### Community 3 - "Community 3"
Cohesion: 0.06
Nodes (56): analyze_never_skipped(), filter_all_phases(), filter_apex_driven_apex_at_correct_position(), filter_apex_driven_from_tasks_to_analyze(), filter_apex_driven_has_apex_not_implement(), filter_apex_driven_only_apex(), filter_existing_schemas_unchanged_by_apex_addition(), filter_from_after_to_errors() (+48 more)

### Community 4 - "Community 4"
Cohesion: 0.08
Nodes (54): ambiguous_language_flagged(), check_ambiguous_language(), check_cross_references(), check_placeholders(), check_requirement_quality(), check_scenario_coverage(), check_section_completeness(), check_security_hints() (+46 more)

### Community 5 - "Community 5"
Cohesion: 0.08
Nodes (57): create_feature(), first_feature_dir_after_specify(), full_tdd_workflow_scaffold_is_consistent(), init_registers_tdd_refactor_command_for_claude(), init_registers_tdd_tests_command_for_claude(), pipeline_dry_run_from_tdd_tests_skips_earlier_phases(), pipeline_dry_run_only_tdd_tests_shows_one_phase(), pipeline_dry_run_shows_handoff_label_for_tdd_phases() (+49 more)

### Community 6 - "Community 6"
Cohesion: 0.08
Nodes (40): Default, AiConfig, CatalogList, ContextConfig, ContextRules, default_agent(), default_code_threshold(), default_fanout_timeout() (+32 more)

### Community 7 - "Community 7"
Cohesion: 0.10
Nodes (44): add_preset(), add_preset_copies_files_and_registers(), add_same_preset_twice_errors(), copy_dir_recursive(), create_preset_source(), get_preset_priorities(), info_missing_returns_none(), info_preset() (+36 more)

### Community 8 - "Community 8"
Cohesion: 0.06
Nodes (38): append_continues_from_last_id(), append_items_start_from_given_id(), checklist_items_match_format(), find_last_chk_id(), generate_append_items(), Option, Result, String (+30 more)

### Community 9 - "Community 9"
Cohesion: 0.09
Nodes (47): baseline_all_not_implemented(), collect_evidence(), EvidenceCriterionResult, EvidenceReport, format_evidence_report(), format_report_contains_table_and_header(), low_satisfaction_gives_drifted_status(), no_tests_dir_returns_baseline() (+39 more)

### Community 10 - "Community 10"
Cohesion: 0.11
Nodes (48): build_solidspec_context(), context_counts_uppercase_checked_tasks_as_done(), context_includes_fr_lines(), context_includes_pending_tasks_only(), context_includes_user_scenarios(), context_missing_all_files_produces_placeholders(), context_missing_spec_produces_placeholder(), context_plan_not_truncated_when_under_limit() (+40 more)

### Community 11 - "Community 11"
Cohesion: 0.13
Nodes (36): all_artifacts_in_default_graph_are_reachable(), all_frs_orphaned_when_no_tasks_md(), ArtifactGraph, ArtifactNode, ArtifactState, build_trace_graph(), compute_states_shows_blocked_when_deps_missing(), default_graph_has_eight_artifacts() (+28 more)

### Community 12 - "Community 12"
Cohesion: 0.10
Nodes (41): apex_driven_apex_requires_tasks(), apex_driven_converts_to_valid_graph(), apex_driven_has_apex_not_implement(), apex_driven_schema_has_ship_artifact(), by_name(), intent_apex_converts_to_valid_graph(), intent_apex_evidence_requires_apex_not_implement(), intent_apex_has_apex_not_implement() (+33 more)

### Community 13 - "Community 13"
Cohesion: 0.11
Nodes (43): ClarificationMarker, empty_spec_handled(), extract_acceptance_scenarios(), extract_clarification_markers(), extract_entities(), extract_entities_with_descriptions(), extract_entities_with_empty_description(), extract_requirements() (+35 more)

### Community 14 - "Community 14"
Cohesion: 0.11
Nodes (40): AcceptanceScenario, cargo_template_valid_syntax(), detect_cargo_test(), detect_framework(), detect_generic_when_no_files(), detect_go_test(), detect_jest_from_package_json(), detect_pytest() (+32 more)

### Community 15 - "Community 15"
Cohesion: 0.08
Nodes (32): dry_run_does_not_write_file(), fails_outside_project(), fails_when_feature_dir_missing(), init(), Option, Path, Result, run() (+24 more)

### Community 16 - "Community 16"
Cohesion: 0.10
Nodes (38): build_template_vars(), HashMap, Option, Result, String, run(), Option, Result (+30 more)

### Community 17 - "Community 17"
Cohesion: 0.14
Nodes (35): archive_change(), archive_nonexistent_change_errors(), ChangeInfo, ChangeMetadata, ChangeStatus, create_and_archive_change_roundtrip(), create_change(), DeltaModification (+27 more)

### Community 18 - "Community 18"
Cohesion: 0.11
Nodes (39): apex_auto_detects_feature_id(), apex_command_appears_in_help(), apex_context_file_contains_feature_id(), apex_context_file_contains_pending_tasks(), apex_context_generation_is_idempotent(), apex_context_includes_fr_requirements_from_spec(), apex_context_only_writes_file_without_instructions(), apex_dry_run_prints_would_write_and_creates_no_file() (+31 more)

### Community 19 - "Community 19"
Cohesion: 0.11
Nodes (30): all(), all_bash_scripts_are_nonempty(), all_powershell_scripts_are_nonempty(), bash_scripts(), check_prerequisites_fails_when_constitution_missing(), check_prerequisites_passes_for_complete_project(), copy_embedded_scripts(), copy_embedded_scripts_creates_files() (+22 more)

### Community 20 - "Community 20"
Cohesion: 0.15
Nodes (27): AnalysisReport, analyze_does_not_modify_files(), analyze_feature(), compute_drift(), constitution_violation_is_critical(), drift_detects_unsatisfied_criteria(), drift_score_100_when_all_criteria_uncovered(), drift_zero_at_baseline_all_not_implemented() (+19 more)

### Community 21 - "Community 21"
Cohesion: 0.13
Nodes (21): empty_commands_list_errors(), ExtensionCommand, ExtensionConfig, ExtensionInfo, ExtensionManifest, ExtensionProvides, ExtensionRequires, hook_referencing_undeclared_command_errors() (+13 more)

### Community 22 - "Community 22"
Cohesion: 0.14
Nodes (23): Result, run(), create_directory_structure(), generate_agent_file(), generate_constitution(), resolve_project_dir(), Option, Path (+15 more)

### Community 23 - "Community 23"
Cohesion: 0.18
Nodes (25): build_cycle_sections(), count_pending_tasks(), extract_acceptance_criteria(), extract_criteria_handles_subsection_headers(), extract_task_summary(), parse_count_line(), parse_red_report(), parse_red_report_extracts_counts() (+17 more)

### Community 24 - "Community 24"
Cohesion: 0.16
Nodes (22): AgentFormat, adjust_script_paths(), adjust_script_paths_replaces(), already_adjusted_paths_not_double_adjusted(), kimi_command_name(), markdown_has_frontmatter_delimiters(), no_double_replacement(), opencode_skill_has_name_and_description() (+14 more)

### Community 25 - "Community 25"
Cohesion: 0.17
Nodes (18): description_over_200_chars_errors(), invalid_id_with_uppercase_errors(), invalid_semver_errors(), invalid_version_specifier_errors(), parse_valid_manifest(), PresetInfo, PresetManifest, PresetProvides (+10 more)

### Community 26 - "Community 26"
Cohesion: 0.20
Nodes (21): auth_with_documented_mitigation_has_no_authn_finding(), auth_without_mitigation_raises_high_finding(), clean_plan_with_no_sensitive_topics_has_no_findings(), format_report_clean_says_no_concerns_detected(), format_report_renders_markdown_with_severity_sections(), format_security_review(), missing_plan_returns_error(), OwaspCheck (+13 more)

### Community 27 - "Community 27"
Cohesion: 0.22
Nodes (12): Into, Option, Self, String, Vec, Step, step_detail_text(), step_tracker_add_and_update() (+4 more)

### Community 28 - "Community 28"
Cohesion: 0.18
Nodes (9): create_issue(), detect_security_gaps(), load_spec(), main(), Unit tests for review_spec.py, Write content to a temp file and return its path., TestDetectSecurityGaps, TestLoadSpec (+1 more)

### Community 29 - "Community 29"
Cohesion: 0.25
Nodes (18): check_intent_constraints(), check_plan_compliance(), Constitution, custom_constitution_with_only_simplicity(), Gate, gate_evaluation_fails_with_violation_details(), gate_evaluation_passes_when_no_violations(), GateResult (+10 more)

### Community 30 - "Community 30"
Cohesion: 0.15
Nodes (18): first_feature_dir(), Path, PathBuf, minimal_pipeline_no_agent_scaffolds_all_four_artifacts(), minimal_status_shows_only_four_artifacts_and_no_clarify_or_review(), minimal_tasks_require_only_spec_and_plan_no_security_review(), security_first_dry_run_previews_all_five_phases_without_executing(), security_first_pipeline_no_agent_scaffolds_all_five_artifacts() (+10 more)

### Community 31 - "Community 31"
Cohesion: 0.23
Nodes (3): basic_analysis(), todolist' should not trigger because \\b prevents partial match., TestBasicAnalysis

### Community 32 - "Community 32"
Cohesion: 0.23
Nodes (3): detect_testing_gaps(), latest' contains 'test' but is NOT a testing term., TestDetectTestingGaps

### Community 33 - "Community 33"
Cohesion: 0.22
Nodes (13): analyze_prints_traceability_chain_tree(), analyze_shows_intent_coverage_with_intent_md(), analyze_without_intent_md_omits_idsd_metrics(), evidence_update_reflects_in_intent_md_status(), idsd_pipeline_scaffold_creates_all_artifacts(), orphaned_requirement_produces_high_finding(), Command, Path (+5 more)

### Community 34 - "Community 34"
Cohesion: 0.15
Nodes (5): HashSet, cli_agents_have_requires_cli_true(), copilot_uses_agent_md_extension(), ide_agents_have_requires_cli_false(), kimi_uses_skill_md_extension()

### Community 37 - "Community 37"
Cohesion: 0.26
Nodes (12): create_feature(), Path, PathBuf, ship_decision_ship_when_all_lanes_pass(), ship_dry_run_shows_all_lanes(), ship_fail_on_hold_exits_nonzero(), ship_fails_without_spec_md(), ship_lane_filter_runs_subset() (+4 more)

### Community 38 - "Community 38"
Cohesion: 0.38
Nodes (11): AgentMode, check_agent_availability(), execute_phase(), invoke_or_handoff(), Option, Path, Result, String (+3 more)

### Community 39 - "Community 39"
Cohesion: 0.30
Nodes (11): full_pipeline_scaffold_generates_all_artifacts(), pipeline_dry_run_output_contains_dry_run_marker(), pipeline_dry_run_respects_custom_schema_generates_override(), pipeline_idsd_generates_intent_before_spec(), pipeline_intent_apex_uses_single_feature_dir(), pipeline_new_ignores_stale_feature_env_var(), pipeline_sdd_unchanged_no_intent_md(), pipeline_status_shows_artifact_table() (+3 more)

### Community 41 - "Community 41"
Cohesion: 0.36
Nodes (9): Shell, bash_completions_contain_subcommands(), fish_completions_generated(), generate_completions(), powershell_completions_generated(), Result, String, run() (+1 more)

### Community 42 - "Community 42"
Cohesion: 0.36
Nodes (9): all_phases_have_personas(), implement_persona_emphasizes_incremental(), Persona, persona_for_phase(), persona_prompt(), persona_prompts_are_nonempty(), review_persona_is_adversarial(), String (+1 more)

### Community 43 - "Community 43"
Cohesion: 0.25
Nodes (4): Option, Result, run(), run_fails_without_project_root()

### Community 44 - "Community 44"
Cohesion: 0.36
Nodes (8): Path, Result, run(), setup_project(), upgrade_preserves_constitution(), upgrade_preserves_overrides(), upgrade_preserves_specs(), upgrade_refreshes_templates()

### Community 47 - "Community 47"
Cohesion: 0.36
Nodes (7): Cli, Commands, Option, Result, String, Vec, run()

### Community 48 - "Community 48"
Cohesion: 0.39
Nodes (7): cli_flag_wins(), empty_string_treated_as_none(), none_flag_falls_through(), resolve_github_token(), Option, String, whitespace_trimmed()

### Community 50 - "Community 50"
Cohesion: 0.33
Nodes (3): find_feature_dir(), get_current_branch(), common.sh script

### Community 51 - "Community 51"
Cohesion: 0.33
Nodes (3): PathBuf, String, SolidSpecError

### Community 53 - "Community 53"
Cohesion: 0.47
Nodes (5): ChangeCommands, Option, Result, String, run()

### Community 54 - "Community 54"
Cohesion: 0.53
Nodes (5): change_archive_merges_deltas_and_moves_to_archive(), change_list_shows_active_changes(), change_propose_creates_directory_and_files(), init_project_with_feature(), Path

### Community 55 - "Community 55"
Cohesion: 0.90
Nodes (4): Find-FeatureDir(), Get-CurrentBranch(), Get-FeaturePaths(), Get-RepoRoot()

### Community 56 - "Community 56"
Cohesion: 0.50
Nodes (4): ExtensionCommands, Result, String, run()

### Community 57 - "Community 57"
Cohesion: 0.50
Nodes (4): PresetCommands, Result, String, run()

### Community 58 - "Community 58"
Cohesion: 0.50
Nodes (3): Option, Result, run()

### Community 59 - "Community 59"
Cohesion: 0.50
Nodes (3): Option, Result, run()

### Community 60 - "Community 60"
Cohesion: 0.50
Nodes (3): Option, Result, run()

### Community 61 - "Community 61"
Cohesion: 0.50
Nodes (3): Option, Result, run()

### Community 62 - "Community 62"
Cohesion: 0.67
Nodes (3): Option, Result, run()

### Community 63 - "Community 63"
Cohesion: 0.50
Nodes (3): Option, Result, run()

### Community 64 - "Community 64"
Cohesion: 0.50
Nodes (3): Option, Result, run()

### Community 65 - "Community 65"
Cohesion: 0.50
Nodes (3): Option, Result, run()

### Community 66 - "Community 66"
Cohesion: 0.50
Nodes (3): Option, Result, run()

## Knowledge Gaps
- **8 isolated node(s):** `check-prerequisites.sh script`, `common.sh script`, `create-new-feature.sh script`, `setup-plan.sh script`, `update-agent-context.sh script` (+3 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **15 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `ExtensionManifest` connect `Community 21` to `Community 2`?**
  _High betweenness centrality (0.030) - this node is a cross-community bridge._
- **Why does `solidspec()` connect `Community 18` to `Community 33`, `Community 5`, `Community 37`, `Community 54`, `Community 30`?**
  _High betweenness centrality (0.014) - this node is a cross-community bridge._
- **Are the 109 inferred relationships involving `solidspec()` (e.g. with `apex_auto_detects_feature_id()` and `apex_command_appears_in_help()`) actually correct?**
  _`solidspec()` has 109 INFERRED edges - model-reasoned connections that need verification._
- **Are the 95 inferred relationships involving `init_project()` (e.g. with `apex_auto_detects_feature_id()` and `apex_context_file_contains_feature_id()`) actually correct?**
  _`init_project()` has 95 INFERRED edges - model-reasoned connections that need verification._
- **Are the 27 inferred relationships involving `first_feature_dir()` (e.g. with `create_feature()` and `pipeline_apex_driven_runs_apex_when_no_finish_file()`) actually correct?**
  _`first_feature_dir()` has 27 INFERRED edges - model-reasoned connections that need verification._
- **Are the 18 inferred relationships involving `find_agent()` (e.g. with `resolve_agent_cli()` and `apex_command_file_contains_apex_workflow_text()`) actually correct?**
  _`find_agent()` has 18 INFERRED edges - model-reasoned connections that need verification._
- **What connects `check-prerequisites.sh script`, `common.sh script`, `create-new-feature.sh script` to the rest of the system?**
  _8 weakly-connected nodes found - possible documentation gaps or missing edges._