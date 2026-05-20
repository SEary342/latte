use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub task_key: String,
    pub message: Option<String>,
    pub tags: Vec<String>,
    pub projects: Vec<String>,
    pub start: Option<u32>,
    pub end: Option<u32>,
}

impl LogEntry {
    pub fn new(
        task_key: String,
        message: Option<String>,
        tags: Vec<String>,
        projects: Vec<String>,
        start: Option<u32>,
        end: Option<u32>,
    ) -> Self {
        let now = Local::now();

        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            task_key,
            message,
            tags,
            projects,
            start,
            end,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = Local::now();
    }

    pub fn formatted_time(&self) -> String {
        match (self.start, self.end) {
            (Some(start), Some(end)) => {
                format!("{:04}-{:04}", start, end)
            }
            (Some(start), None) => {
                format!("{:04}-", start)
            }
            (None, Some(end)) => {
                format!("-{:04}", end)
            }
            (None, None) => String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LogDatabase {
    pub entries: Vec<LogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_path: PathBuf,
}
