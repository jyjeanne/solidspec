//! Integration tests for `solidspec schemas` — the discovery command that
//! lists every available workflow schema and, since the /spcx:* command
//! restructure, points at the actual /spcx:<short>:* slash commands each
//! one registers (src/agents/spcx.rs's schema_short_name).

use predicates::prelude::*;

mod common;
use common::init_project;

#[test]
fn schemas_lists_every_builtin_with_its_spcx_short_name() {
    let dir = init_project();

    common::solidspec()
        .args(["schemas"])
        .current_dir(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("spec-driven"))
        .stdout(predicate::str::contains("/spcx:sdd:new"))
        .stdout(predicate::str::contains("/spcx:min:new"))
        .stdout(predicate::str::contains("/spcx:tdd:new"))
        .stdout(predicate::str::contains("/spcx:sec:new"));
}
