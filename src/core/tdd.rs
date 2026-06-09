#![allow(dead_code)]

use std::path::Path;

use anyhow::Result;

// ── RED report scaffolding ────────────────────────────────────────────────────

/// Build the `tdd-red-report.md` template that the agent will fill in during
/// the RED phase. Structures the report around vertical slices: interface design
/// first, then a tracer bullet (first AC), then a cycle table for the rest.
pub fn scaffold_red_report(feature_dir: &Path, feature_id: &str) -> Result<String> {
    let criteria = extract_acceptance_criteria(feature_dir);
    let tasks_summary = extract_task_summary(feature_dir);

    let (tracer_bullet_section, cycles_section) = build_cycle_sections(&criteria);

    let report = format!(
        "# TDD Red Report — {feature_id}\n\
         \n\
         **Phase**: RED — vertical slices, NOT horizontal batching\n\
         **Status**: [ ] pending\n\
         \n\
         ---\n\
         \n\
         ## Step 1 — Interface Design\n\
         \n\
         _Complete before writing any test code._\n\
         \n\
         **Public APIs to test** (function names / method signatures):\n\
         _[list what tests will call — e.g. `checkout(cart, payment) -> Result`]_\n\
         \n\
         **Dependency injection plan**:\n\
         _[what external deps are accepted as parameters vs created internally]_\n\
         \n\
         **Mock boundaries** — mock ONLY these:\n\
         - [ ] External HTTP / API clients\n\
         - [ ] Database (prefer a real test DB; mock only if unavoidable)\n\
         - [ ] Clock / random sources / file I/O\n\
         \n\
         **Do NOT mock**: your own modules, internal collaborators, anything you control.\n\
         \n\
         **Framework detected**: _[e.g. jest, cargo test, pytest, go test]_\n\
         _(If detection fails — no Cargo.toml / package.json / pyproject.toml / go.mod — STOP and record the failure here.)_\n\
         \n\
         ---\n\
         \n\
         ## Step 2 — Tracer Bullet (First Cycle)\n\
         \n\
         Write ONE test for the most critical behavior. Make it compile and fail for the\n\
         right reason before writing more tests.\n\
         \n\
         {tracer_bullet_section}\n\
         \n\
         **Test name**: `_[function_name_describing_behavior]_`\n\
         **Failure reason**: _[what error message / assertion failure proves the code doesn't exist yet]_\n\
         **Status**: [ ] RED (compiles, fails correctly)\n\
         \n\
         ---\n\
         \n\
         ## Step 3 — Remaining Test Cycles\n\
         \n\
         Write one test per behavior. Each must compile and fail before moving to the next.\n\
         One AC may produce multiple rows if it covers multiple distinct behaviors.\n\
         \n\
         {cycles_section}\n\
         \n\
         ---\n\
         \n\
         ## Step 4 — Test Quality Checklist\n\
         \n\
         Before filling Test Results below, verify every test:\n\
         \n\
         - [ ] Describes observable behavior, not internal implementation steps\n\
         - [ ] Calls public APIs only — no private methods, no direct DB queries to verify results\n\
         - [ ] Would survive a complete internal refactor without modification\n\
         - [ ] Has one logical assertion (or one coherent behavior group)\n\
         - [ ] Does not mock internal collaborators (only system boundaries)\n\
         \n\
         ---\n\
         \n\
         ## Test Results (RED state)\n\
         \n\
         ```\n\
         Total tests written : _[N]_\n\
         Tests FAILING       : _[N]_  ← must equal total for pure RED\n\
         Tests passing       : _[0]_  ← if > 0, list them below\n\
         ```\n\
         \n\
         **Unexpectedly passing tests** (already-implemented behavior — exclude from GREEN phase):\n\
         _[test names, or \"none\"]_\n\
         \n\
         ---\n\
         \n\
         ## Task Scope\n\
         \n\
         {tasks_summary}\n\
         \n\
         ---\n\
         \n\
         ## Coverage Thresholds (targets for GREEN + REFACTOR phases)\n\
         \n\
         | Scope | Target |\n\
         |-------|--------|\n\
         | Unit tests | 90% |\n\
         | Integration tests | 80% |\n\
         | Security-critical paths | 100% |\n\
         \n\
         ---\n\
         \n\
         ## Notes\n\
         \n\
         _Agent: note edge cases, boundary conditions, and security paths covered by the tests._\n"
    );

    Ok(report)
}

/// Split acceptance criteria into a tracer bullet (first item) and a cycle table (rest).
fn build_cycle_sections(criteria: &[String]) -> (String, String) {
    if criteria.is_empty() {
        let tracer = "_[no acceptance criteria found in spec.md — add them under \
                      `## Acceptance Criteria` or within each user story]_"
            .to_string();
        let cycles = "_[no remaining cycles — all criteria are covered by the tracer bullet \
                      or spec.md has no acceptance criteria]_"
            .to_string();
        return (tracer, cycles);
    }

    let tracer = format!("**Behavior**: {}", criteria[0]);

    let cycles = if criteria.len() < 2 {
        "_[all criteria covered by the tracer bullet above]_".to_string()
    } else {
        let header = "| Cycle | Acceptance Criterion | Test Name | RED | GREEN |\n\
                      |-------|---------------------|-----------|-----|-------|"
            .to_string();
        let rows: Vec<String> = criteria[1..]
            .iter()
            .enumerate()
            .map(|(i, c)| format!("| {} | {} | `_[name]_` | [ ] | [ ] |", i + 2, c))
            .collect();
        format!("{}\n{}", header, rows.join("\n"))
    };

    (tracer, cycles)
}

// ── REFACTOR report scaffolding ───────────────────────────────────────────────

/// Build the `tdd-refactor-report.md` template for the REFACTOR phase.
pub fn scaffold_refactor_report(feature_dir: &Path, feature_id: &str) -> Result<String> {
    let pending = count_pending_tasks(feature_dir);
    let warning = if pending > 0 {
        format!(
            "> ⚠ Warning: {pending} pending task(s) remain in tasks.md. \
             Confirm all tests are GREEN before proceeding.\n\n"
        )
    } else {
        String::new()
    };

    let report = format!(
        "# TDD Refactor Report — {feature_id}\n\
         \n\
         **Phase**: REFACTOR\n\
         **Status**: [ ] pending\n\
         \n\
         {warning}\
         ---\n\
         \n\
         ## Pre-Refactor Checklist\n\
         \n\
         - [ ] Full test suite is GREEN (run it now to confirm before touching any code)\n\
         - [ ] No pending tasks in tasks.md\n\
         - [ ] tdd-red-report.md is complete\n\
         - [ ] Interface decisions from Step 1 of tdd-red-report.md are respected\n\
         \n\
         ---\n\
         \n\
         ## Refactor Candidates\n\
         \n\
         Work through in priority order. Check off what you find and apply:\n\
         \n\
         - [ ] **Duplication** → extract to a shared function or class\n\
         - [ ] **Long methods** → extract private helpers \
         _(keep tests targeting the public interface, not the helpers)_\n\
         - [ ] **Shallow modules** → deepen: reduce public methods, hide complexity inside\n\
         - [ ] **Feature envy** → move logic to where the data lives\n\
         - [ ] **Primitive obsession** → introduce value objects for domain concepts\n\
         - [ ] **Interface creep** → verify: public surface area must not grow during refactor\n\
         - [ ] **Revealed problems** → existing code that the new code exposes as problematic\n\
         \n\
         ---\n\
         \n\
         ## Refactoring Changes\n\
         \n\
         _One row per change. Run the full test suite after each row — must stay GREEN._\n\
         \n\
         | File | Refactor Type | Before | After | Tests |\n\
         |------|--------------|--------|-------|-------|\n\
         | _example.rs_ | _Duplication_ | _copy-pasted block_ | _extracted `helper_fn()`_ | GREEN |\n\
         \n\
         ---\n\
         \n\
         ## Complexity Delta\n\
         \n\
         ```\n\
         Before: _[describe duplication, nesting depth, public method count]_\n\
         After : _[describe improvements — fewer methods, less nesting, removed duplication]_\n\
         ```\n\
         \n\
         ---\n\
         \n\
         ## Post-Refactor Test Run\n\
         \n\
         ```\n\
         Command : _[e.g. cargo test / npm test / pytest]_\n\
         Result  : _[PASS / FAIL — must be PASS]_\n\
         Total   : _[N tests]_\n\
         Passing : _[N]_\n\
         Failing : _[0]_\n\
         ```\n\
         \n\
         ---\n\
         \n\
         ## Definition of Done\n\
         \n\
         - [ ] All tests remain GREEN after refactoring\n\
         - [ ] No behavior changes introduced\n\
         - [ ] No features added\n\
         - [ ] Public interface surface area did not grow\n\
         - [ ] tdd-refactor-report.md complete\n"
    );

    Ok(report)
}

// ── RED report parsing ────────────────────────────────────────────────────────

pub struct RedReport {
    pub tests_found: usize,
    pub tests_failing: usize,
    pub framework: String,
}

/// Parse a completed `tdd-red-report.md` for test counts and framework.
/// Returns zeros / "unknown" for any field that can't be parsed.
pub fn parse_red_report(report_path: &Path) -> Result<RedReport> {
    let content = std::fs::read_to_string(report_path)?;

    // Use split_once so framework names containing ':' (e.g. "node:test") are preserved.
    let framework = content
        .lines()
        .find(|l| l.starts_with("**Framework detected**:"))
        .and_then(|l| l.split_once(':').map(|(_, r)| r))
        .map(|s| s.trim().trim_matches('_').to_string())
        .filter(|s| !s.is_empty() && s != "[e.g. jest, cargo test, pytest, go test]")
        .unwrap_or_else(|| "unknown".to_string());

    let tests_found = parse_count_line(&content, "Total tests written");
    let tests_failing = parse_count_line(&content, "Tests FAILING");

    Ok(RedReport {
        tests_found,
        tests_failing,
        framework,
    })
}

fn parse_count_line(content: &str, label: &str) -> usize {
    content
        .lines()
        .find(|l| l.contains(label))
        .and_then(|l| l.split_once(':').map(|(_, r)| r))
        .and_then(|v| {
            v.trim()
                .trim_matches('_')
                .split_whitespace()
                .next()
                .and_then(|n| n.parse().ok())
        })
        .unwrap_or(0)
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn extract_acceptance_criteria(feature_dir: &Path) -> Vec<String> {
    let spec_path = feature_dir.join("spec.md");
    let Ok(content) = std::fs::read_to_string(&spec_path) else {
        return vec![];
    };

    let mut criteria = Vec::new();
    let mut in_section = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## Acceptance Criteria")
            || trimmed.starts_with("### Acceptance Criteria")
        {
            in_section = true;
            continue;
        }
        // Only a same-level (##) heading closes the section; sub-headers (###) stay inside.
        if in_section && trimmed.starts_with("## ") {
            break;
        }
        if in_section && (trimmed.starts_with("- ") || trimmed.starts_with("* ")) {
            let item = trimmed
                .trim_start_matches('-')
                .trim_start_matches('*')
                .trim()
                .to_string();
            if !item.is_empty() {
                criteria.push(item);
            }
        }
    }

    // Fallback: look for GIVEN/WHEN/THEN acceptance patterns in user stories
    if criteria.is_empty() {
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("GIVEN:") || t.starts_with("WHEN:") || t.starts_with("THEN:") {
                criteria.push(t.to_string());
            }
        }
    }

    criteria
}

fn extract_task_summary(feature_dir: &Path) -> String {
    let tasks_path = feature_dir.join("tasks.md");
    let Ok(content) = std::fs::read_to_string(&tasks_path) else {
        return "_[tasks.md not yet generated]_".to_string();
    };

    let pending = content.matches("- [ ] T").count();
    let done = content.matches("- [x] T").count() + content.matches("- [X] T").count();
    format!("**{pending} pending** / {done} done (from tasks.md)")
}

fn count_pending_tasks(feature_dir: &Path) -> usize {
    let tasks_path = feature_dir.join("tasks.md");
    std::fs::read_to_string(&tasks_path)
        .map(|c| c.matches("- [ ] T").count())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write_spec_with_ac(dir: &std::path::Path) {
        std::fs::write(
            dir.join("spec.md"),
            "# Feature\n\
             ## Acceptance Criteria\n\
             - Users can log in with valid credentials\n\
             - Invalid credentials return 401\n\
             - Locked accounts are rejected\n\
             ## Other\n\
             Some other content.\n",
        )
        .unwrap();
    }

    #[test]
    fn scaffold_red_report_contains_ac_items() {
        let dir = TempDir::new().unwrap();
        write_spec_with_ac(dir.path());
        std::fs::write(
            dir.path().join("tasks.md"),
            "- [ ] T001 Setup\n- [ ] T002 Auth\n",
        )
        .unwrap();

        let report = scaffold_red_report(dir.path(), "001-login").unwrap();
        assert!(report.contains("Users can log in with valid credentials"));
        assert!(report.contains("Invalid credentials return 401"));
        assert!(report.contains("Locked accounts are rejected"));
        assert!(report.contains("Coverage Thresholds"));
    }

    #[test]
    fn scaffold_red_report_graceful_without_spec() {
        let dir = TempDir::new().unwrap();
        let report = scaffold_red_report(dir.path(), "001-login").unwrap();
        assert!(report.contains("no acceptance criteria found"));
    }

    #[test]
    fn scaffold_refactor_report_warns_with_pending_tasks() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("tasks.md"),
            "- [ ] T001 Unfinished\n- [x] T002 Done\n",
        )
        .unwrap();

        let report = scaffold_refactor_report(dir.path(), "001-login").unwrap();
        assert!(report.contains("Warning"), "must warn about pending tasks");
        assert!(report.contains("1 pending"));
    }

    #[test]
    fn scaffold_refactor_report_no_warning_when_all_done() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("tasks.md"), "- [x] T001 Done\n").unwrap();

        let report = scaffold_refactor_report(dir.path(), "001-login").unwrap();
        assert!(!report.contains("Warning"));
    }

    #[test]
    fn parse_red_report_extracts_counts() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("tdd-red-report.md"),
            "# TDD Red Report\n\
             **Framework detected**: cargo test\n\
             ```\n\
             Total tests written : 5\n\
             Tests FAILING       : 5\n\
             Tests passing       : 0\n\
             ```\n",
        )
        .unwrap();

        let r = parse_red_report(&dir.path().join("tdd-red-report.md")).unwrap();
        assert_eq!(r.tests_found, 5);
        assert_eq!(r.tests_failing, 5);
        assert_eq!(r.framework, "cargo test");
    }

    #[test]
    fn parse_red_report_preserves_colon_in_framework_name() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("tdd-red-report.md"),
            "# TDD Red Report\n\
             **Framework detected**: node:test\n\
             ```\n\
             Total tests written : 3\n\
             Tests FAILING       : 3\n\
             Tests passing       : 0\n\
             ```\n",
        )
        .unwrap();

        let r = parse_red_report(&dir.path().join("tdd-red-report.md")).unwrap();
        assert_eq!(
            r.framework, "node:test",
            "colon in framework name must not be truncated"
        );
        assert_eq!(r.tests_found, 3);
    }

    #[test]
    fn scaffold_red_report_has_interface_design_section() {
        let dir = TempDir::new().unwrap();
        let report = scaffold_red_report(dir.path(), "001-login").unwrap();
        assert!(
            report.contains("Interface Design"),
            "must have interface design section"
        );
        assert!(
            report.contains("Mock boundaries"),
            "must have mock boundaries guidance"
        );
        assert!(
            report.contains("Tracer Bullet"),
            "must have tracer bullet section"
        );
        assert!(
            report.contains("Test Quality Checklist"),
            "must have quality checklist"
        );
        assert!(
            report.contains("Unexpectedly passing tests"),
            "must have unexpectedly passing section"
        );
    }

    #[test]
    fn scaffold_red_report_tracer_bullet_uses_first_ac() {
        let dir = TempDir::new().unwrap();
        write_spec_with_ac(dir.path());

        let report = scaffold_red_report(dir.path(), "001-login").unwrap();
        // First AC becomes the tracer bullet
        assert!(
            report.contains("**Behavior**: Users can log in with valid credentials"),
            "first AC must be the tracer bullet"
        );
        // Remaining ACs appear in the cycle table
        assert!(
            report.contains("Invalid credentials return 401"),
            "second AC must be in cycle table"
        );
        assert!(
            report.contains("Locked accounts are rejected"),
            "third AC must be in cycle table"
        );
    }

    #[test]
    fn scaffold_red_report_single_ac_has_no_remaining_cycles() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("spec.md"),
            "# Feature\n## Acceptance Criteria\n- Only one criterion\n",
        )
        .unwrap();

        let report = scaffold_red_report(dir.path(), "001-login").unwrap();
        assert!(
            report.contains("**Behavior**: Only one criterion"),
            "single AC must be tracer"
        );
        assert!(
            report.contains("all criteria covered by the tracer bullet"),
            "no remaining cycles for single AC"
        );
    }

    #[test]
    fn scaffold_refactor_report_has_candidates_checklist() {
        let dir = TempDir::new().unwrap();
        let report = scaffold_refactor_report(dir.path(), "001-login").unwrap();
        assert!(
            report.contains("Refactor Candidates"),
            "must have candidates section"
        );
        assert!(
            report.contains("Duplication"),
            "must list duplication candidate"
        );
        assert!(
            report.contains("Shallow modules"),
            "must list shallow module candidate"
        );
        assert!(
            report.contains("Feature envy"),
            "must list feature envy candidate"
        );
        assert!(
            report.contains("Interface creep"),
            "must warn about interface growth"
        );
        assert!(
            report.contains("Refactor Type"),
            "changes table must have refactor type column"
        );
    }

    #[test]
    fn extract_criteria_handles_subsection_headers() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("spec.md"),
            "# Feature\n\
             ## Acceptance Criteria\n\
             - First criterion\n\
             ### Implementation Notes\n\
             Some note here.\n\
             - Second criterion\n\
             ## Other Section\n\
             Ignored.\n",
        )
        .unwrap();

        let criteria = extract_acceptance_criteria(dir.path());
        assert!(
            criteria.contains(&"First criterion".to_string()),
            "first criterion must be captured"
        );
        assert!(
            criteria.contains(&"Second criterion".to_string()),
            "criteria after a ### sub-header must still be captured"
        );
        assert_eq!(
            criteria.len(),
            2,
            "must not capture items from ## Other Section"
        );
    }
}
