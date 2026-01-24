//! Analytics functions for task analysis.

use std::collections::HashMap;
use crate::task::{Priority, Task, TaskStatus};

/// Group tasks by priority.
pub fn tasks_by_priority(tasks: &[Task]) -> HashMap<Priority, Vec<&Task>> {
    let mut grouped: HashMap<Priority, Vec<&Task>> = HashMap::new();
    for task in tasks {
        grouped.entry(task.priority).or_default().push(task);
    }
    grouped
}

/// Count tasks by status.
pub fn tasks_by_status(tasks: &[Task]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for task in tasks {
        let key = match task.status {
            TaskStatus::Todo => "Todo",
            TaskStatus::InProgress { .. } => "In Progress",
            TaskStatus::Blocked { .. } => "Blocked",
            TaskStatus::Completed { .. } => "Completed",
        };
        *counts.entry(key.to_string()).or_default() += 1;
    }
    counts
}

/// Calculate total estimated hours per assignee.
pub fn workload_by_assignee(tasks: &[Task]) -> HashMap<String, f32> {
    let mut workload: HashMap<String, f32> = HashMap::new();
    for task in tasks {
        if let Some(assignee) = &task.assignee {
            let hours = task.estimated_hours.unwrap_or(0.0);
            *workload.entry(assignee.clone()).or_default() += hours;
        }
    }
    workload
}
