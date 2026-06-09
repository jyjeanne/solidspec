//! DAG-based artifact graph engine.
//!
//! Replaces the linear pipeline with a dependency graph where artifacts
//! can be created in any order as long as their prerequisites are met.
//! Uses Kahn's algorithm for topological sort and filesystem-based
//! completion detection.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

/// A single artifact in the workflow graph.
#[derive(Debug, Clone)]
pub struct ArtifactNode {
    /// Unique identifier (e.g., "spec", "plan", "tasks")
    pub id: String,
    /// Glob patterns for output files (e.g., "spec.md", "tests/*")
    pub generates: Vec<String>,
    /// IDs of artifacts that must be completed first
    pub requires: Vec<String>,
    /// Human-readable instruction for creating this artifact
    #[allow(dead_code)]
    pub instruction: String,
    /// Optional template name to scaffold before instruction
    #[allow(dead_code)]
    pub template: Option<String>,
}

/// Directed acyclic graph of workflow artifacts.
#[derive(Debug, Clone)]
pub struct ArtifactGraph {
    pub nodes: HashMap<String, ArtifactNode>,
    /// Edges: (prerequisite_id, dependent_id) — prerequisite must complete first
    edges: Vec<(String, String)>,
}

/// Possible states for an artifact in the workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactState {
    /// Dependencies not yet met — cannot start
    Blocked { missing_deps: Vec<String> },
    /// All deps done, file not yet generated — ready to create
    Ready,
    /// Output file exists — artifact is complete
    Done,
}

impl ArtifactGraph {
    /// Build a graph from a list of artifact nodes. Validates for duplicate
    /// IDs and references to nonexistent dependencies.
    pub fn new(nodes: Vec<ArtifactNode>) -> Result<Self, String> {
        let mut node_map = HashMap::new();
        let mut edges = Vec::new();

        for node in &nodes {
            if node_map.contains_key(&node.id) {
                return Err(format!("Duplicate artifact ID: {}", node.id));
            }
            node_map.insert(node.id.clone(), node.clone());

            for req in &node.requires {
                if !nodes.iter().any(|n| n.id == *req) {
                    return Err(format!(
                        "Artifact '{}' depends on unknown artifact '{}'",
                        node.id, req
                    ));
                }
                edges.push((req.clone(), node.id.clone()));
            }
        }

        Ok(Self {
            nodes: node_map,
            edges,
        })
    }

    /// Get an artifact by ID.
    #[allow(dead_code)]
    pub fn get(&self, id: &str) -> Option<&ArtifactNode> {
        self.nodes.get(id)
    }

    /// All artifact IDs in the graph.
    #[allow(dead_code)]
    pub fn artifact_ids(&self) -> Vec<&str> {
        self.nodes.keys().map(|s| s.as_str()).collect()
    }

    /// Kahn's algorithm: returns artifacts in topological build order.
    /// Returns an error if the graph contains a cycle.
    pub fn topological_order(&self) -> Result<Vec<&ArtifactNode>, String> {
        let mut in_degree: HashMap<&str, usize> = HashMap::new();
        for id in self.nodes.keys() {
            in_degree.insert(id.as_str(), 0);
        }
        for (_prereq, dependent) in &self.edges {
            *in_degree.get_mut(dependent.as_str()).unwrap() += 1;
        }

        let mut queue: VecDeque<&str> = in_degree
            .iter()
            .filter(|(_, deg)| **deg == 0)
            .map(|(&id, _)| id)
            .collect();

        let mut order = Vec::new();
        while let Some(id) = queue.pop_front() {
            order.push(self.nodes.get(id).unwrap());

            for (prereq, dependent) in &self.edges {
                if *prereq == id {
                    let deg = in_degree.get_mut(dependent.as_str()).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(dependent);
                    }
                }
            }
        }

        if order.len() != self.nodes.len() {
            return Err("Cycle detected in artifact graph".into());
        }

        Ok(order)
    }

    /// Determine the state of each artifact given a set of completed artifact IDs.
    pub fn compute_states(&self, completed: &HashSet<String>) -> HashMap<String, ArtifactState> {
        let mut states = HashMap::new();

        for (id, node) in &self.nodes {
            if completed.contains(id) {
                states.insert(id.clone(), ArtifactState::Done);
            } else {
                let missing_deps: Vec<String> = node
                    .requires
                    .iter()
                    .filter(|req| !completed.contains(*req))
                    .cloned()
                    .collect();

                if missing_deps.is_empty() {
                    states.insert(id.clone(), ArtifactState::Ready);
                } else {
                    states.insert(id.clone(), ArtifactState::Blocked { missing_deps });
                }
            }
        }

        states
    }

    /// Check which artifacts are complete based on filesystem state.
    /// An artifact is considered Done if all of its `generates` glob patterns
    /// match at least one file under the feature directory.
    pub fn detect_completion(&self, feature_dir: &Path) -> HashSet<String> {
        let mut completed = HashSet::new();

        for (id, node) in &self.nodes {
            let all_present = node.generates.iter().all(|pattern| {
                let path = feature_dir.join(pattern);
                // Simple check: exact filename or non-empty directory
                if pattern.contains('*') {
                    // Glob: check if directory exists and is non-empty
                    if let Some(parent) = path.parent() {
                        parent.exists()
                            && std::fs::read_dir(parent)
                                .map(|mut d| d.next().is_some())
                                .unwrap_or(false)
                    } else {
                        false
                    }
                } else {
                    path.exists()
                }
            });

            if all_present {
                completed.insert(id.clone());
            }
        }

        completed
    }
}

// ── Traceability graph ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceLinkType {
    IntentToRequirement,
    RequirementToTask,
    TaskToTest,
}

#[derive(Debug, Clone)]
pub struct TraceLink {
    pub from_id: String,
    pub to_id: String,
    pub link_type: TraceLinkType,
}

/// Full traceability graph: Intent → Requirement → Task → Test.
#[derive(Debug, Clone)]
pub struct TraceGraph {
    pub links: Vec<TraceLink>,
    /// FR-XXX identifiers that appear in spec but have no task referencing them.
    pub orphaned_requirements: Vec<String>,
    /// FR-XXX → short description from spec.md
    pub requirement_texts: HashMap<String, String>,
    /// T-number → first line of task description from tasks.md
    pub task_texts: HashMap<String, String>,
}

impl TraceGraph {
    pub fn tasks_for_req(&self, req_id: &str) -> Vec<&str> {
        self.links
            .iter()
            .filter(|l| l.link_type == TraceLinkType::RequirementToTask && l.from_id == req_id)
            .map(|l| l.to_id.as_str())
            .collect()
    }

    pub fn tests_for_task(&self, task_id: &str) -> Vec<&str> {
        self.links
            .iter()
            .filter(|l| l.link_type == TraceLinkType::TaskToTest && l.from_id == task_id)
            .map(|l| l.to_id.as_str())
            .collect()
    }

    /// Render the chain as an ASCII tree string.
    pub fn format_tree(&self) -> String {
        let mut out = String::from("## Traceability Chain\n\n");

        let mut fr_ids: Vec<&str> = self.requirement_texts.keys().map(|s| s.as_str()).collect();
        fr_ids.sort();

        let intent_id = self
            .links
            .iter()
            .find(|l| l.link_type == TraceLinkType::IntentToRequirement)
            .map(|l| l.from_id.as_str());

        if let Some(iid) = intent_id {
            out.push_str(&format!("{iid}\n"));
        }

        for (i, fr_id) in fr_ids.iter().enumerate() {
            let is_last_fr = i == fr_ids.len() - 1;
            let fr_connector = if intent_id.is_some() {
                if is_last_fr {
                    "└── "
                } else {
                    "├── "
                }
            } else {
                ""
            };
            let fr_continuation = if intent_id.is_some() {
                if is_last_fr { "    " } else { "│   " }
            } else {
                ""
            };

            let fr_text = self
                .requirement_texts
                .get(*fr_id)
                .cloned()
                .unwrap_or_default();
            let orphaned = self.orphaned_requirements.contains(&fr_id.to_string());

            if orphaned {
                out.push_str(&format!("{fr_connector}{fr_id}  {fr_text}  ← no task\n"));
                continue;
            }

            out.push_str(&format!("{fr_connector}{fr_id}  {fr_text}\n"));

            let tasks = self.tasks_for_req(fr_id);
            for (j, task_id) in tasks.iter().enumerate() {
                let is_last_task = j == tasks.len() - 1;
                let task_connector = if is_last_task {
                    "└── "
                } else {
                    "├── "
                };
                let task_continuation = if is_last_task { "    " } else { "│   " };
                let task_text = self.task_texts.get(*task_id).cloned().unwrap_or_default();

                out.push_str(&format!(
                    "{fr_continuation}{task_connector}{task_id}  {task_text}\n"
                ));

                let tests = self.tests_for_task(task_id);
                for test_file in &tests {
                    out.push_str(&format!(
                        "{fr_continuation}{task_continuation}└── {test_file}\n"
                    ));
                }
            }
        }

        out
    }
}

// ── Regex patterns for trace parsing ────────────────────────────────────────

static INTENT_ID_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\*\*Intent ID\*\*:\s*(INT-\d+)").expect("invalid intent ID regex")
});
static FR_DEF_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*\*(FR-\d{3})\*\*:\s*(.+)").expect("invalid FR def regex"));
static TASK_LINE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*-\s+\[[ xX]\]\s+(T\d+)\s*(.*)").expect("invalid task line regex")
});
static FR_IN_TASK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bFR-\d{3}\b").expect("invalid FR-in-task regex"));
// Accept any T\d+ (not just 3+ digits) so T5/T25 in test files match T005/T025 in tasks.md
// after left-pad normalisation.
static TASK_IN_TEST_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bT(\d+)\b").expect("invalid task-in-test regex"));

/// Build a full Intent → Requirement → Task → Test traceability graph
/// for a feature directory. Returns `None` when `spec.md` is absent.
pub fn build_trace_graph(feature_dir: &Path) -> Option<TraceGraph> {
    let spec_path = feature_dir.join("spec.md");
    if !spec_path.exists() {
        return None;
    }
    let spec_content = std::fs::read_to_string(&spec_path).ok()?;

    // Intent ID
    let intent_id: Option<String> = {
        let intent_path = feature_dir.join("intent.md");
        if intent_path.exists() {
            std::fs::read_to_string(&intent_path)
                .ok()
                .and_then(|c| INTENT_ID_RE.captures(&c).map(|caps| caps[1].to_string()))
        } else {
            None
        }
    };

    // FR-XXX → description from spec
    let mut requirement_texts: HashMap<String, String> = HashMap::new();
    for caps in FR_DEF_RE.captures_iter(&spec_content) {
        requirement_texts.insert(caps[1].to_string(), caps[2].trim().to_string());
    }
    let fr_ids: Vec<String> = {
        let mut v: Vec<String> = requirement_texts.keys().cloned().collect();
        v.sort();
        v
    };

    // Tasks: task_id → Vec<FR-XXX referenced>, plus task descriptions
    let tasks_path = feature_dir.join("tasks.md");
    let mut task_fr_links: Vec<(String, Vec<String>)> = Vec::new();
    let mut task_texts: HashMap<String, String> = HashMap::new();
    if tasks_path.exists()
        && let Ok(content) = std::fs::read_to_string(&tasks_path)
    {
        for line in content.lines() {
            if let Some(caps) = TASK_LINE_RE.captures(line) {
                let task_id = caps[1].to_string();
                let task_desc = caps[2].trim().to_string();
                // Dedup so a task that mentions the same FR twice (e.g. in a comment)
                // does not produce duplicate RequirementToTask links.
                let frs: Vec<String> = {
                    let mut seen = HashSet::new();
                    FR_IN_TASK_RE
                        .find_iter(&task_desc)
                        .filter_map(|m| {
                            let fr = m.as_str().to_string();
                            seen.insert(fr.clone()).then_some(fr)
                        })
                        .collect()
                };
                task_texts.insert(task_id.clone(), task_desc);
                task_fr_links.push((task_id, frs));
            }
        }
    }

    // Test files: file_name → Vec<task IDs mentioned in content>
    let tests_dir = feature_dir.join("tests");
    let mut test_task_links: Vec<(String, Vec<String>)> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&tests_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let ext = path.extension().and_then(|x| x.to_str()).unwrap_or("");
            if !matches!(ext, "md" | "ts" | "py" | "rs" | "go") {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if let Ok(content) = std::fs::read_to_string(&path) {
                let task_refs: Vec<String> = {
                    let mut seen = HashSet::new();
                    TASK_IN_TEST_RE
                        .captures_iter(&content)
                        .filter_map(|c| {
                            let digits = &c[1];
                            // Normalise to T001 format (left-pad to 3 digits minimum)
                            let padded = if digits.len() < 3 {
                                format!("{:0>3}", digits)
                            } else {
                                digits.to_string()
                            };
                            let id = format!("T{padded}");
                            seen.insert(id.clone()).then_some(id)
                        })
                        .collect()
                };
                if !task_refs.is_empty() {
                    test_task_links.push((name, task_refs));
                }
            }
        }
    }

    // Build links
    let mut links: Vec<TraceLink> = Vec::new();

    // Intent → FR
    if let Some(ref iid) = intent_id {
        for fr in &fr_ids {
            links.push(TraceLink {
                from_id: iid.clone(),
                to_id: fr.clone(),
                link_type: TraceLinkType::IntentToRequirement,
            });
        }
    }

    // FR → Task
    for (task_id, task_frs) in &task_fr_links {
        for fr in task_frs {
            if requirement_texts.contains_key(fr) {
                links.push(TraceLink {
                    from_id: fr.clone(),
                    to_id: task_id.clone(),
                    link_type: TraceLinkType::RequirementToTask,
                });
            }
        }
    }

    // Task → Test
    for (test_name, task_ids) in &test_task_links {
        for task_id in task_ids {
            links.push(TraceLink {
                from_id: task_id.clone(),
                to_id: test_name.clone(),
                link_type: TraceLinkType::TaskToTest,
            });
        }
    }

    // Orphaned FRs: present in spec but not referenced by any task
    let fr_with_task: HashSet<&str> = links
        .iter()
        .filter(|l| l.link_type == TraceLinkType::RequirementToTask)
        .map(|l| l.from_id.as_str())
        .collect();

    let orphaned_requirements: Vec<String> = fr_ids
        .iter()
        .filter(|fr| !fr_with_task.contains(fr.as_str()))
        .cloned()
        .collect();

    Some(TraceGraph {
        links,
        orphaned_requirements,
        requirement_texts,
        task_texts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn spec_driven_graph() -> ArtifactGraph {
        ArtifactGraph::new(vec![
            ArtifactNode {
                id: "spec".into(),
                generates: vec!["spec.md".into()],
                requires: vec![],
                instruction: "".into(),
                template: None,
            },
            ArtifactNode {
                id: "clarify".into(),
                generates: vec!["spec.md".into()],
                requires: vec!["spec".into()],
                instruction: "".into(),
                template: None,
            },
            ArtifactNode {
                id: "plan".into(),
                generates: vec!["plan.md".into()],
                requires: vec!["spec".into()],
                instruction: "".into(),
                template: None,
            },
            ArtifactNode {
                id: "tasks".into(),
                generates: vec!["tasks.md".into()],
                requires: vec!["spec".into(), "plan".into()],
                instruction: "".into(),
                template: None,
            },
            ArtifactNode {
                id: "tests".into(),
                generates: vec!["tests/".into()],
                requires: vec!["spec".into()],
                instruction: "".into(),
                template: None,
            },
            ArtifactNode {
                id: "implement".into(),
                generates: vec!["tasks.md".into()],
                requires: vec!["tasks".into()],
                instruction: "".into(),
                template: None,
            },
            ArtifactNode {
                id: "analyze".into(),
                generates: vec!["analysis-report.md".into()],
                requires: vec!["spec".into()],
                instruction: "".into(),
                template: None,
            },
            ArtifactNode {
                id: "review".into(),
                generates: vec!["review-report.md".into()],
                requires: vec!["spec".into()],
                instruction: "".into(),
                template: None,
            },
        ])
        .expect("valid graph")
    }

    #[test]
    fn default_graph_has_eight_artifacts() {
        let graph = spec_driven_graph();
        assert_eq!(graph.nodes.len(), 8);
    }

    #[test]
    fn topological_order_starts_with_no_dependency_artifacts() {
        let graph = spec_driven_graph();
        let order = graph.topological_order().unwrap();
        assert_eq!(order[0].id, "spec");
    }

    #[test]
    fn tasks_requires_spec_and_plan() {
        let graph = spec_driven_graph();
        let tasks = graph.get("tasks").unwrap();
        assert!(tasks.requires.contains(&"spec".to_string()));
        assert!(tasks.requires.contains(&"plan".to_string()));
    }

    #[test]
    fn compute_states_shows_blocked_when_deps_missing() {
        let graph = spec_driven_graph();
        let completed = HashSet::from(["spec".to_string()]);
        let states = graph.compute_states(&completed);

        assert_eq!(states.get("plan").unwrap(), &ArtifactState::Ready);
        match states.get("tasks").unwrap() {
            ArtifactState::Blocked { missing_deps } => {
                assert!(missing_deps.contains(&"plan".to_string()));
            }
            _ => panic!("tasks should be blocked"),
        }
    }

    #[test]
    fn detect_completion_finds_existing_files() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("spec.md"), "# Spec").unwrap();
        std::fs::write(dir.path().join("plan.md"), "# Plan").unwrap();

        let graph = spec_driven_graph();
        let completed = graph.detect_completion(dir.path());
        assert!(completed.contains("spec"));
        assert!(completed.contains("plan"));
        assert!(!completed.contains("tasks"));
    }

    #[test]
    fn all_artifacts_in_default_graph_are_reachable() {
        let graph = spec_driven_graph();
        let order = graph.topological_order().unwrap();
        assert_eq!(order.len(), 8);
        let ids: HashSet<_> = order.iter().map(|n| n.id.as_str()).collect();
        assert!(ids.contains("spec"));
        assert!(ids.contains("clarify"));
        assert!(ids.contains("plan"));
        assert!(ids.contains("tasks"));
        assert!(ids.contains("tests"));
        assert!(ids.contains("implement"));
        assert!(ids.contains("analyze"));
        assert!(ids.contains("review"));
    }

    #[test]
    fn duplicate_artifact_id_errors() {
        let nodes = vec![
            ArtifactNode {
                id: "spec".into(),
                generates: vec!["a.md".into()],
                requires: vec![],
                instruction: "".into(),
                template: None,
            },
            ArtifactNode {
                id: "spec".into(),
                generates: vec!["b.md".into()],
                requires: vec![],
                instruction: "".into(),
                template: None,
            },
        ];
        assert!(ArtifactGraph::new(nodes).is_err());
    }

    #[test]
    fn unknown_dependency_errors() {
        let nodes = vec![ArtifactNode {
            id: "spec".into(),
            generates: vec!["spec.md".into()],
            requires: vec!["nonexistent".into()],
            instruction: "".into(),
            template: None,
        }];
        assert!(ArtifactGraph::new(nodes).is_err());
    }

    // ── build_trace_graph() tests ────────────────────────────────────────────

    fn write(dir: &std::path::Path, name: &str, content: &str) {
        if let Some(parent) = dir.join(name).parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(dir.join(name), content).unwrap();
    }

    const SPEC_WITH_TWO_FRS: &str = "\
## Requirements\n\
- **FR-001**: System MUST authenticate users via email\n\
- **FR-002**: System MUST allow password resets\n";

    const TASKS_WITH_FR_REF: &str = "\
- [ ] T001 Setup project [FR-001]\n\
- [ ] T002 Implement authentication module [FR-001]\n\
- [ ] T003 Add email service [FR-002]\n";

    const TASKS_NO_FR_REF: &str = "\
- [ ] T001 Setup project\n\
- [ ] T002 Implement things\n";

    #[test]
    fn no_spec_returns_none() {
        let dir = TempDir::new().unwrap();
        assert!(build_trace_graph(dir.path()).is_none());
    }

    #[test]
    fn extracts_fr_ids_from_spec() {
        let dir = TempDir::new().unwrap();
        write(dir.path(), "spec.md", SPEC_WITH_TWO_FRS);
        let tg = build_trace_graph(dir.path()).unwrap();
        assert!(tg.requirement_texts.contains_key("FR-001"));
        assert!(tg.requirement_texts.contains_key("FR-002"));
        assert_eq!(tg.requirement_texts.len(), 2);
    }

    #[test]
    fn all_frs_orphaned_when_no_tasks_md() {
        let dir = TempDir::new().unwrap();
        write(dir.path(), "spec.md", SPEC_WITH_TWO_FRS);
        let tg = build_trace_graph(dir.path()).unwrap();
        assert_eq!(tg.orphaned_requirements.len(), 2);
        assert!(tg.orphaned_requirements.contains(&"FR-001".to_string()));
        assert!(tg.orphaned_requirements.contains(&"FR-002".to_string()));
    }

    #[test]
    fn fr_with_task_not_orphaned() {
        let dir = TempDir::new().unwrap();
        write(dir.path(), "spec.md", SPEC_WITH_TWO_FRS);
        write(dir.path(), "tasks.md", TASKS_WITH_FR_REF);
        let tg = build_trace_graph(dir.path()).unwrap();
        assert!(!tg.orphaned_requirements.contains(&"FR-001".to_string()));
        assert!(!tg.orphaned_requirements.contains(&"FR-002".to_string()));
    }

    #[test]
    fn tasks_without_fr_refs_produce_orphaned_frs() {
        let dir = TempDir::new().unwrap();
        write(dir.path(), "spec.md", SPEC_WITH_TWO_FRS);
        write(dir.path(), "tasks.md", TASKS_NO_FR_REF);
        let tg = build_trace_graph(dir.path()).unwrap();
        assert_eq!(tg.orphaned_requirements.len(), 2);
    }

    #[test]
    fn req_to_task_links_created() {
        let dir = TempDir::new().unwrap();
        write(dir.path(), "spec.md", SPEC_WITH_TWO_FRS);
        write(dir.path(), "tasks.md", TASKS_WITH_FR_REF);
        let tg = build_trace_graph(dir.path()).unwrap();
        let tasks_for_fr001 = tg.tasks_for_req("FR-001");
        assert!(tasks_for_fr001.contains(&"T001"));
        assert!(tasks_for_fr001.contains(&"T002"));
        let tasks_for_fr002 = tg.tasks_for_req("FR-002");
        assert!(tasks_for_fr002.contains(&"T003"));
    }

    #[test]
    fn intent_to_req_links_created_when_intent_md_present() {
        let dir = TempDir::new().unwrap();
        write(dir.path(), "spec.md", SPEC_WITH_TWO_FRS);
        write(
            dir.path(),
            "intent.md",
            "# Intent: Auth\n\n**Intent ID**: INT-001\n**Feature**: 001-auth\n\
             **Created**: 2026-01-01\n**Status**: active\n\n## Goal\nAuth.\n",
        );
        let tg = build_trace_graph(dir.path()).unwrap();
        let intent_links: Vec<_> = tg
            .links
            .iter()
            .filter(|l| l.link_type == TraceLinkType::IntentToRequirement)
            .collect();
        assert_eq!(intent_links.len(), 2);
        assert!(intent_links.iter().all(|l| l.from_id == "INT-001"));
    }

    #[test]
    fn task_to_test_link_when_test_mentions_task_id() {
        let dir = TempDir::new().unwrap();
        write(dir.path(), "spec.md", SPEC_WITH_TWO_FRS);
        write(dir.path(), "tasks.md", TASKS_WITH_FR_REF);
        write(
            dir.path(),
            "tests/story1.md",
            "GIVEN: user\nWHEN: T001 setup\nSTATUS: IMPLEMENTED\n",
        );
        let tg = build_trace_graph(dir.path()).unwrap();
        let tests_for_t001 = tg.tests_for_task("T001");
        assert!(
            tests_for_t001.contains(&"story1.md"),
            "story1.md should link to T001"
        );
    }

    #[test]
    fn format_tree_contains_fr_ids() {
        let dir = TempDir::new().unwrap();
        write(dir.path(), "spec.md", SPEC_WITH_TWO_FRS);
        write(dir.path(), "tasks.md", TASKS_WITH_FR_REF);
        let tg = build_trace_graph(dir.path()).unwrap();
        let tree = tg.format_tree();
        assert!(tree.contains("FR-001"));
        assert!(tree.contains("FR-002"));
        assert!(tree.contains("T001"));
    }
}
