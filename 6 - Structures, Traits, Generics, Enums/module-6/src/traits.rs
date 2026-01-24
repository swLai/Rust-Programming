//! Traits for shared behavior.

use crate::task::{Task, TaskStatus};
use crate::project::Project;

/// Types that can produce a text summary.
pub trait Summarizable {
    fn summary(&self) -> String;

    fn one_line_summary(&self) -> String {
        let full = self.summary();
        if full.len() > 60 {
            format!("{}...", &full[..57])
        } else {
            full
        }
    }
}

impl Summarizable for Task {
    fn summary(&self) -> String {
        let status = match &self.status {
            TaskStatus::Todo => "TODO".to_string(),
            TaskStatus::InProgress { started_by } => format!("In Progress ({})", started_by),
            TaskStatus::Blocked { reason } => format!("BLOCKED: {}", reason),
            TaskStatus::Completed { completed_by, hours_spent } => {
                format!("Done by {} in {}h", completed_by, hours_spent)
            }
        };
        format!("[{}] {:?}: {} | {}", self.id, self.task_type, self.title, status)
    }
}

impl Summarizable for Project {
    fn summary(&self) -> String {
        format!(
            "Project: {} ({} tasks, {:.1}% complete)",
            self.name,
            self.tasks.len(),
            self.completion_percentage()
        )
    }
}

/// Types that can calculate statistics.
pub trait Statistics {
    fn total_estimate(&self) -> Option<f32>;
    fn average_estimate(&self) -> Option<f32>;
}

impl Statistics for Project {
    fn total_estimate(&self) -> Option<f32> {
        let estimates: Vec<f32> = self.tasks
            .iter()
            .filter_map(|t| t.estimated_hours)
            .collect();

        if estimates.is_empty() {
            None
        } else {
            Some(estimates.iter().sum())
        }
    }

    fn average_estimate(&self) -> Option<f32> {
        let total = self.total_estimate()?;
        if self.tasks.is_empty() {
            None
        } else {
            Some(total / self.tasks.len() as f32)
        }
    }
}
