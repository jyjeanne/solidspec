//! Native Open Knowledge Format (OKF) knowledge-graph generation, in-process.
//!
//! Wraps `okf-core`/`okf-analyzer`/`okf-generator`/`okf-validator` — vendored
//! as pinned git dependencies (see `Cargo.toml`) from
//! <https://github.com/jyjeanne/okf-rs> — rather than shelling out to a
//! separately-installed `okf-rs` binary. This is step 2 of
//! `docs/okf-rs-integration-plan.md`, upgraded from "detect an external
//! binary" to full library integration per an explicit choice to avoid an
//! external-binary dependency entirely.
//!
//! Only the subset needed to generate and validate a bundle is vendored
//! (not `okf-search`/`okf-watch`/`okf-docs`/`okf-mcp`/`okf-enrich`, which
//! pull in tantivy, a file watcher, a PDF renderer, and an OpenAI-compatible
//! HTTP client respectively) — `okf-rs search`/`explore`/`graph`/`impact`
//! still need the external CLI or `okf-mcp` for now.

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use anyhow::{Context, Result};
use okf_parser::ConceptKind;

/// Filename for okf-analyzer's incremental-index cache, at the project root
/// — same name/location `okf-rs generate` itself uses, so a project that
/// switches between the native command and the external CLI reuses one
/// cache rather than invalidating it.
const CACHE_FILE: &str = ".okf-cache.json";

/// Default OKF bundle location for a SolidSpec-scaffolded project — the
/// convention `solidspec init`'s auto-generation (`src/cli/init.rs`), the
/// `okf` extension (`extensions/okf/`), `solidspec okf generate`/`validate`'s
/// own CLI defaults, and `analyzer::structural_cross_check` all agree on,
/// so nothing needs to be told twice where a project's bundle lives.
pub const DEFAULT_BUNDLE_DIR: &str = ".solidspec/knowledge";

#[derive(Debug, Clone, Default)]
pub struct GenerateReport {
    pub total_concepts: usize,
    pub by_kind: BTreeMap<ConceptKind, usize>,
    pub files_parsed: usize,
    pub files_reused_from_cache: usize,
}

/// Analyzes `project_root` and writes an OKF bundle to `output_dir`.
///
/// LSP-backed disambiguation (`okf-rs generate --lsp`) is intentionally not
/// exposed here: it spawns real language-server processes, which is a very
/// different operational shape (external processes, per-language servers)
/// than the rest of this module's promise of no external tools.
pub fn generate(project_root: &Path, output_dir: &Path) -> Result<GenerateReport> {
    let project = okf_core::Project::load(project_root)
        .with_context(|| format!("failed to scan project at {}", project_root.display()))?;

    let cache_path = project.root.join(CACHE_FILE);
    let mut cache = okf_analyzer::AnalysisCache::load(&cache_path);

    let (result, stats) = okf_analyzer::analyze_with_cache_lsp(&project, &mut cache, false)
        .context("failed to analyze project")?;

    let source_revision = okf_core::git::head_revision(&project.root);
    okf_generator::write_bundle(&result.concepts, output_dir, source_revision.as_deref())
        .with_context(|| format!("failed to write OKF bundle to {}", output_dir.display()))?;

    cache
        .save(&cache_path)
        .context("failed to save the incremental-index cache")?;

    let mut by_kind: BTreeMap<ConceptKind, usize> = BTreeMap::new();
    for concept in &result.concepts {
        *by_kind.entry(concept.kind).or_default() += 1;
    }

    Ok(GenerateReport {
        total_concepts: result.concepts.len(),
        by_kind,
        files_parsed: stats.reparsed,
        files_reused_from_cache: stats.reused,
    })
}

/// Runs every OKF bundle-conformance check against `bundle_dir`.
///
/// Returns the raw [`okf_validator::ValidationReport`] — use
/// [`validation_should_fail`] to apply `--ci`-equivalent gating on top.
pub fn validate(bundle_dir: &Path) -> Result<okf_validator::ValidationReport> {
    okf_validator::validate_bundle(bundle_dir)
        .with_context(|| format!("failed to validate bundle at {}", bundle_dir.display()))
}

/// Whether `report` should fail a run, given `--ci`'s promotion of
/// orphaned-concept warnings to failures. Mirrors `okf-rs validate --ci`
/// exactly (see okf-rs issue #23: every other warning class stays
/// advisory even under `--ci`).
pub fn validation_should_fail(report: &okf_validator::ValidationReport, ci: bool) -> bool {
    report.has_errors() || (ci && report.has_warning_of_kind(okf_validator::IssueKind::Orphan))
}

/// A lightweight, read-only index over an already-generated OKF bundle —
/// "does this file/symbol actually appear in the knowledge graph" — built
/// by reading the bundle's own Markdown+YAML files back
/// (`okf_parser::read_bundle`) rather than re-running extraction. This is
/// the native equivalent of `okf-rs search`/`explore` for the one query
/// `analyzer::structural_cross_check` needs (membership, not ranked
/// search) — see `docs/kg-workflow-vision-gap-analysis.md`'s
/// recommendation #1.
pub struct BundleIndex {
    files: HashSet<String>,
    symbols: HashSet<String>,
}

impl BundleIndex {
    /// Loads the bundle at `bundle_dir`. Returns `Ok(None)` (not an error)
    /// when the directory doesn't exist — a project that never ran
    /// `solidspec okf generate` is not a failure, just nothing to check
    /// against.
    pub fn load(bundle_dir: &Path) -> Result<Option<Self>> {
        if !bundle_dir.exists() {
            return Ok(None);
        }
        let concepts = okf_parser::read_bundle(bundle_dir)
            .with_context(|| format!("failed to read OKF bundle at {}", bundle_dir.display()))?;

        let mut files = HashSet::new();
        let mut symbols = HashSet::new();
        for concept in &concepts {
            files.insert(concept.location.file.clone());
            symbols.insert(concept.name.clone());
        }
        Ok(Some(Self { files, symbols }))
    }

    /// Whether any concept in the bundle was extracted from `path` (project
    /// root-relative, `/`-separated — the same form `Location::file` stores).
    pub fn has_file(&self, path: &str) -> bool {
        self.files.contains(path)
    }

    /// Whether any concept in the bundle is named exactly `name`.
    pub fn has_symbol(&self, name: &str) -> bool {
        self.symbols.contains(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write_sample_project(dir: &Path) {
        std::fs::write(
            dir.join("lib.rs"),
            "pub struct Widget;\n\nimpl Widget {\n    pub fn new() -> Self {\n        Widget\n    }\n}\n",
        )
        .unwrap();
    }

    #[test]
    fn bundle_index_load_returns_none_when_bundle_missing() {
        let dir = TempDir::new().unwrap();
        let missing = dir.path().join("nope");
        assert!(BundleIndex::load(&missing).unwrap().is_none());
    }

    #[test]
    fn bundle_index_knows_generated_files_and_symbols() {
        let project_dir = TempDir::new().unwrap();
        write_sample_project(project_dir.path());
        let output_dir = TempDir::new().unwrap();
        generate(project_dir.path(), output_dir.path()).unwrap();

        let index = BundleIndex::load(output_dir.path()).unwrap().unwrap();
        assert!(index.has_file("lib.rs"));
        assert!(index.has_symbol("Widget"));
        assert!(index.has_symbol("new"));
        assert!(!index.has_file("nonexistent.rs"));
        assert!(!index.has_symbol("TotallyMadeUpSymbol"));
    }

    #[test]
    fn generate_writes_a_bundle_with_expected_concepts() {
        let project_dir = TempDir::new().unwrap();
        write_sample_project(project_dir.path());
        let output_dir = TempDir::new().unwrap();

        let report = generate(project_dir.path(), output_dir.path()).unwrap();

        assert!(report.total_concepts > 0);
        assert!(output_dir.path().join("index.md").exists());
        assert!(
            project_dir.path().join(CACHE_FILE).exists(),
            "the incremental-index cache should be written alongside the source"
        );
    }

    #[test]
    fn generate_is_incremental_on_a_second_run() {
        let project_dir = TempDir::new().unwrap();
        write_sample_project(project_dir.path());
        let output_dir = TempDir::new().unwrap();

        let first = generate(project_dir.path(), output_dir.path()).unwrap();
        assert_eq!(first.files_reused_from_cache, 0);

        let second = generate(project_dir.path(), output_dir.path()).unwrap();
        assert_eq!(
            second.files_reused_from_cache, first.files_parsed,
            "an unchanged file should be reused from cache on the next run"
        );
    }

    #[test]
    fn validate_reports_no_issues_for_a_freshly_generated_bundle() {
        let project_dir = TempDir::new().unwrap();
        write_sample_project(project_dir.path());
        let output_dir = TempDir::new().unwrap();
        generate(project_dir.path(), output_dir.path()).unwrap();

        let report = validate(output_dir.path()).unwrap();

        assert!(
            !validation_should_fail(&report, true),
            "{:?}",
            report.issues
        );
    }
}
