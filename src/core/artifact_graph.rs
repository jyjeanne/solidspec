//! DAG-based artifact graph engine.
//!
//! Replaces the linear pipeline with a dependency graph where artifacts
//! can be created in any order as long as their prerequisites are met.
//! Uses Kahn's algorithm for topological sort and filesystem-based
//! completion detection.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn spec_driven_graph() -> ArtifactGraph {
        ArtifactGraph::new(vec![
            ArtifactNode {
                id: "spec".into(), generates: vec!["spec.md".into()],
                requires: vec![], instruction: "".into(), template: None,
            },
            ArtifactNode {
                id: "clarify".into(), generates: vec!["spec.md".into()],
                requires: vec!["spec".into()], instruction: "".into(), template: None,
            },
            ArtifactNode {
                id: "plan".into(), generates: vec!["plan.md".into()],
                requires: vec!["spec".into()], instruction: "".into(), template: None,
            },
            ArtifactNode {
                id: "tasks".into(), generates: vec!["tasks.md".into()],
                requires: vec!["spec".into(), "plan".into()],
                instruction: "".into(), template: None,
            },
            ArtifactNode {
                id: "tests".into(), generates: vec!["tests/".into()],
                requires: vec!["spec".into()], instruction: "".into(), template: None,
            },
            ArtifactNode {
                id: "implement".into(), generates: vec!["tasks.md".into()],
                requires: vec!["tasks".into()],
                instruction: "".into(), template: None,
            },
            ArtifactNode {
                id: "analyze".into(), generates: vec!["analysis-report.md".into()],
                requires: vec!["spec".into()], instruction: "".into(), template: None,
            },
            ArtifactNode {
                id: "review".into(), generates: vec!["review-report.md".into()],
                requires: vec!["spec".into()], instruction: "".into(), template: None,
            },
        ]).expect("valid graph")
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
                id: "spec".into(), generates: vec!["a.md".into()],
                requires: vec![], instruction: "".into(), template: None,
            },
            ArtifactNode {
                id: "spec".into(), generates: vec!["b.md".into()],
                requires: vec![], instruction: "".into(), template: None,
            },
        ];
        assert!(ArtifactGraph::new(nodes).is_err());
    }

    #[test]
    fn unknown_dependency_errors() {
        let nodes = vec![ArtifactNode {
            id: "spec".into(), generates: vec!["spec.md".into()],
            requires: vec!["nonexistent".into()],
            instruction: "".into(), template: None,
        }];
        assert!(ArtifactGraph::new(nodes).is_err());
    }
}
