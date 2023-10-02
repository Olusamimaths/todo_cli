use std::str::FromStr;

use crate::task::{Task, TaskStatus};

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        return TaskManager { tasks: vec![] };
    }

    pub fn add_task(&mut self, title: String, description: String, task_status: Option<String>) {
        let status = TaskStatus::from_str(&task_status.unwrap_or(String::from("default")));

        let new_task = Task {
            id: "adf".to_string(),
            title,
            description,
            status: status.unwrap(),
        };
        self.tasks.push(new_task);
    }

    fn change_task_status(&mut self, task_id: String, status: TaskStatus) -> bool {
        for task in &mut self.tasks {
            if task.id == task_id {
                task.status = status;
                return true;
            }
        }
        return false;
    }

    fn delete_task(&mut self, task_id: String) {
        self.tasks.retain(|task| task.id != task_id);
    }

    fn get_task(&mut self, task_id: String) -> Option<Task> {
        if let Some(index) = self.tasks.iter_mut().position(|task| task.id == task_id) {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!("-------------------------------------------");
            println!("{}", task);
            println!("-------------------------------------------");
        }
    }
}
