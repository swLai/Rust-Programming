//! Project containing multiple tasks.

use crate::task::Task;

/// A project with a collection of tasks.
#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub tasks: Vec<Task>,
}

impl Project {
    pub fn new(name: &str) -> Self {
        Project {
            name: String::from(name),
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn completion_percentage(&self) -> f32 {
        if self.tasks.is_empty() {
            return 0.0;
        }
        let completed = self.tasks.iter().filter(|t| t.status.is_done()).count();
        (completed as f32 / self.tasks.len() as f32) * 100.0
    }

    pub fn find_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }
}
