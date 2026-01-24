//! Task and related types.

/// Priority levels for tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Types of tasks in the system.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskType {
    Bug,
    Feature,
    Improvement,
    Documentation,
}

/// Represents the current state of a task.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress { started_by: String },
    Blocked { reason: String },
    Completed { completed_by: String, hours_spent: f32 },
}

impl TaskStatus {
    pub fn is_done(&self) -> bool {
        matches!(self, TaskStatus::Completed { .. })
    }
}

/// A task in the system.
#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub priority: Priority,
    pub status: TaskStatus,
    pub task_type: TaskType,
    pub assignee: Option<String>,
    pub estimated_hours: Option<f32>,
}

impl Task {
    pub fn new(id: u32, title: &str, task_type: TaskType) -> Self {
        Task {
            id,
            title: String::from(title),
            priority: Priority::Medium,
            status: TaskStatus::Todo,
            task_type,
            assignee: None,
            estimated_hours: None,
        }
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn assigned_to(mut self, person: &str) -> Self {
        self.assignee = Some(String::from(person));
        self
    }

    pub fn with_estimate(mut self, hours: f32) -> Self {
        self.estimated_hours = Some(hours);
        self
    }

    pub fn start(&mut self, developer: &str) -> Result<(), String> {
        match &self.status {
            TaskStatus::Todo => {
                self.status = TaskStatus::InProgress {
                    started_by: String::from(developer),
                };
                Ok(())
            }
            TaskStatus::Blocked { reason } => {
                Err(format!("Cannot start: blocked - {}", reason))
            }
            TaskStatus::InProgress { started_by } => {
                Err(format!("Already in progress by {}", started_by))
            }
            TaskStatus::Completed { .. } => {
                Err(String::from("Task already completed"))
            }
        }
    }

    pub fn complete(&mut self, developer: &str, hours: f32) -> Result<(), String> {
        match &self.status {
            TaskStatus::InProgress { .. } => {
                self.status = TaskStatus::Completed {
                    completed_by: String::from(developer),
                    hours_spent: hours,
                };
                Ok(())
            }
            _ => Err(String::from("Can only complete tasks in progress")),
        }
    }
}
