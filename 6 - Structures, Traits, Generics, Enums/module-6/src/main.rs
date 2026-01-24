//! Task Management System

mod task;
mod project;
mod traits;
mod analytics;

use task::{Priority, Task, TaskType};
use project::Project;
use traits::{Summarizable, Statistics};

fn main() {
    // Create tasks
    let task1 = Task::new(1, "Fix login authentication bug", TaskType::Bug)
        .with_priority(Priority::Critical)
        .assigned_to("Alice")
        .with_estimate(4.0);

    let task2 = Task::new(2, "Implement dark mode", TaskType::Feature)
        .with_priority(Priority::Medium)
        .assigned_to("Bob")
        .with_estimate(16.0);

    let task3 = Task::new(3, "Optimize database queries", TaskType::Improvement)
        .with_priority(Priority::High)
        .with_estimate(8.0);

    let task4 = Task::new(4, "Update API documentation", TaskType::Documentation)
        .with_priority(Priority::Low)
        .assigned_to("Charlie")
        .with_estimate(3.0);

    // Create project and add tasks
    let mut project = Project::new("Website Redesign");
    project.add_task(task1);
    project.add_task(task2);
    project.add_task(task3);
    project.add_task(task4);

    // Display project overview
    println!("{}\n", project.summary());
    for task in &project.tasks {
        println!("  {}", task.one_line_summary());
    }

    // Show estimates
    if let Some(total) = project.total_estimate() {
        println!("\nTotal estimated: {:.1} hours", total);
    }
    if let Some(avg) = project.average_estimate() {
        println!("Average per task: {:.1} hours", avg);
    }

    // Complete a task
    if let Some(task) = project.find_task_mut(1) {
        let _ = task.start("Alice");
        let _ = task.complete("Alice", 3.5);
    }

    println!("\nAfter completing task 1:");
    println!("{}", project.summary());

    // Analytics
    println!("\nWorkload by developer:");
    for (dev, hours) in &analytics::workload_by_assignee(&project.tasks) {
        println!("  {}: {:.1}h", dev, hours);
    }

    println!("\nBy priority:");
    for (priority, tasks) in &analytics::tasks_by_priority(&project.tasks) {
        println!("  {:?}: {}", priority, tasks.len());
    }

    println!("\nBy status:");
    for (status, count) in &analytics::tasks_by_status(&project.tasks) {
        println!("  {}: {}", status, count);
    }

    // Find unassigned tasks
    let unassigned: Vec<_> = project.tasks.iter()
        .filter(|t| t.assignee.is_none())
        .collect();
    println!("\nUnassigned tasks: {}", unassigned.len());
}
