//! Colored step-progress UI (`Step`/`StepTracker`) for reporting multi-step
//! command progress. Fully built and tested but not yet wired into any CLI
//! command — `cli/pipeline.rs` currently prints phase progress with plain
//! `println!` calls instead. Candidate replacement for that ad-hoc reporting
//! if/when a richer progress display is wanted.
#![allow(dead_code)]

use console::Style;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    Running,
    Done,
    Error,
    Skipped,
}

#[derive(Debug)]
pub struct Step {
    pub label: String,
    pub status: StepStatus,
    pub detail: Option<String>,
}

impl Step {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            status: StepStatus::Pending,
            detail: None,
        }
    }

    pub fn display(&self) -> String {
        let (symbol, style) = match self.status {
            StepStatus::Pending => ("○", Style::new().dim()),
            StepStatus::Running => ("○", Style::new().yellow()),
            StepStatus::Done => ("●", Style::new().green()),
            StepStatus::Error => ("●", Style::new().red()),
            StepStatus::Skipped => ("○", Style::new().dim()),
        };

        let label = style.apply_to(&self.label);
        let detail = self
            .detail
            .as_ref()
            .map(|d| format!(" {}", Style::new().dim().apply_to(format!("({d})"))))
            .unwrap_or_default();

        format!("{symbol} {label}{detail}")
    }
}

#[derive(Debug)]
pub struct StepTracker {
    steps: Vec<Step>,
}

impl StepTracker {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn add(&mut self, label: impl Into<String>) -> usize {
        let idx = self.steps.len();
        self.steps.push(Step::new(label));
        idx
    }

    pub fn set_status(&mut self, idx: usize, status: StepStatus) {
        if let Some(step) = self.steps.get_mut(idx) {
            step.status = status;
        }
    }

    pub fn set_detail(&mut self, idx: usize, detail: impl Into<String>) {
        if let Some(step) = self.steps.get_mut(idx) {
            step.detail = Some(detail.into());
        }
    }

    pub fn print_all(&self) {
        for step in &self.steps {
            println!("  {}", step.display());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_transitions() {
        let mut step = Step::new("Test step");
        assert_eq!(step.status, StepStatus::Pending);

        step.status = StepStatus::Running;
        assert_eq!(step.status, StepStatus::Running);
        let display = step.display();
        assert!(display.contains("○"));

        step.status = StepStatus::Done;
        assert_eq!(step.status, StepStatus::Done);
        let display = step.display();
        assert!(display.contains("●"));

        step.status = StepStatus::Error;
        let display = step.display();
        assert!(display.contains("●"));

        step.status = StepStatus::Skipped;
        let display = step.display();
        assert!(display.contains("○"));
    }

    #[test]
    fn step_detail_text() {
        let mut step = Step::new("Loading");
        step.detail = Some("50%".into());
        let display = step.display();
        assert!(display.contains("50%"));
    }

    #[test]
    fn step_tracker_add_and_update() {
        let mut tracker = StepTracker::new();
        let idx = tracker.add("Step 1");
        assert_eq!(idx, 0);

        tracker.set_status(idx, StepStatus::Done);
        tracker.set_detail(idx, "completed");

        assert_eq!(tracker.steps[0].status, StepStatus::Done);
        assert_eq!(tracker.steps[0].detail.as_deref(), Some("completed"));
    }

    #[test]
    fn step_tracker_out_of_bounds_noop() {
        let mut tracker = StepTracker::new();
        tracker.set_status(99, StepStatus::Done); // should not panic
        tracker.set_detail(99, "test");
    }
}
