use std::collections::HashMap;
use std::sync::LazyLock;

use anyhow::{Context, Result};
use regex::Regex;

static CHK_ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"CHK(\d{3})").expect("invalid checklist id regex"));

use crate::config;
use crate::core::feature;
use crate::presets::manager as preset_manager;
use crate::templates;
use crate::templates::resolver;

pub fn run(feature_id: Option<&str>, append: bool) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a SolidSpec project. Run 'solidspec init' first.")?;

    let feature_dir_name = feature::resolve_feature(feature_id, &project_root)?;
    let feature_dir = project_root.join("specs").join(&feature_dir_name);
    let checklists_dir = feature_dir.join("checklists");
    std::fs::create_dir_all(&checklists_dir)?;

    let checklist_path = checklists_dir.join("requirements.md");

    if append && checklist_path.exists() {
        // Append mode: find last CHK ID and continue
        let existing = std::fs::read_to_string(&checklist_path)?;
        let last_id = find_last_chk_id(&existing);
        if last_id >= 999 {
            anyhow::bail!("Checklist ID overflow: all 999 CHK IDs used.");
        }
        let next_id = last_id + 1;

        println!("Appending to checklist (continuing from CHK{:03})", next_id);

        let new_items = generate_append_items(next_id);
        let mut content = existing;
        content.push_str(&format!("\n## Additional Checks (appended)\n\n{new_items}"));
        std::fs::write(&checklist_path, content)?;
    } else {
        // Create mode
        println!("Generating checklist: {feature_dir_name}");

        let root_config = config::RootConfig::load(&project_root.join("solidspec.toml"))?;
        let vars = HashMap::from([
            ("feature_name".to_string(), feature_dir_name.clone()),
            ("branch_name".to_string(), feature_dir_name),
            (
                "date".to_string(),
                chrono::Local::now().format("%Y-%m-%d").to_string(),
            ),
            ("project_name".to_string(), root_config.project.name),
        ]);

        let preset_priorities =
            preset_manager::get_preset_priorities(&project_root).unwrap_or_default();
        let (checklist_tmpl, _) =
            resolver::load_template("checklist-template.md", &project_root, &preset_priorities)
                .unwrap_or_else(|e| {
                    log::warn!("Failed to load checklist template, using default: {e}");
                    (
                        templates::embedded::CHECKLIST_TEMPLATE.to_string(),
                        resolver::TemplateSource::EmbeddedDefault,
                    )
                });
        let content = templates::render(&checklist_tmpl, &vars)?;
        std::fs::write(&checklist_path, content)?;
    }

    println!("  Checklist at {}", checklist_path.display());
    Ok(())
}

fn find_last_chk_id(content: &str) -> u32 {
    let mut max_id: u32 = 0;
    for caps in CHK_ID_RE.captures_iter(content) {
        if let Ok(num) = caps[1].parse::<u32>() {
            max_id = max_id.max(num);
        }
    }
    max_id
}

fn generate_append_items(start_id: u32) -> String {
    let items = [
        "Are all edge cases from user stories covered?",
        "Is error handling defined for each functional requirement?",
        "Are performance criteria measurable and bounded?",
    ];

    items
        .iter()
        .enumerate()
        .filter(|(i, _)| start_id + *i as u32 <= 999) // don't generate IDs past 999
        .map(|(i, q)| format!("- [ ] CHK{:03} {q} [completeness]", start_id + i as u32))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_last_chk_id_from_content() {
        let content = "- [ ] CHK001 first\n- [ ] CHK002 second\n- [x] CHK015 last\n";
        assert_eq!(find_last_chk_id(content), 15);
    }

    #[test]
    fn find_last_chk_id_empty() {
        assert_eq!(find_last_chk_id("no checklist items here"), 0);
    }

    #[test]
    fn append_items_start_from_given_id() {
        let items = generate_append_items(16);
        assert!(items.contains("CHK016"));
        assert!(items.contains("CHK017"));
        assert!(items.contains("CHK018"));
    }

    #[test]
    fn append_continues_from_last_id() {
        let existing = "- [ ] CHK001 first\n- [ ] CHK015 last\n";
        let last = find_last_chk_id(existing);
        assert_eq!(last, 15);
        let items = generate_append_items(last + 1);
        assert!(items.contains("CHK016"));
    }

    #[test]
    fn checklist_items_match_format() {
        let items = generate_append_items(1);
        for line in items.lines() {
            assert!(line.starts_with("- [ ] CHK"), "Bad format: {line}");
            assert!(line.contains('['), "Missing dimension bracket: {line}");
        }
    }
}
