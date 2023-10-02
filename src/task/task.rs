use std::{fmt::Display, str::FromStr};
use anyhow::{Ok, Result};

#[derive(Copy, Clone, Debug)]
pub enum TaskStatus {
    Completed,
    InProgress,
    Cancelled,
    Default,
}

impl FromStr for TaskStatus {
    type Err =  anyhow::Error;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err>  {
        match input.to_lowercase().as_str() {
            "completed" | "done" => Ok(TaskStatus::Completed),
            "inprogress" | "in progress" => Ok(TaskStatus::InProgress),
            "cancelled" => Ok(TaskStatus::Cancelled),
            "default" => Ok(TaskStatus::Default),
            _ => return Err(anyhow::anyhow!("unknown task status")),
        }
    }
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Cancelled => write!(f, "Cancelled"),
            TaskStatus::Default => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Task ID: {}, 
            \nTitle: {},
            \nDescription: {},
            \nStatus: {}
            ",
            self.id, self.title, self.description, self.status
        );
    }
}
