use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use crate::cli::AddArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: Uuid,

    pub created_at: DateTime<Local>,
    pub log_date: DateTime<Local>,

    pub task_key: String,
    pub task_description: String,

    pub message: Option<String>,

    pub tags: Vec<String>,
    pub projects: Vec<String>,
    pub activity_types: Vec<String>,

    pub start_time: Option<u32>,
    pub end_time: Option<u32>,
}

impl LogEntry {
    pub fn formatted_time(&self) -> String {
        match (self.start_time, self.end_time) {
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

    pub fn from_add_args(args: AddArgs, task_key: String, task_description: String) -> Self {
        let now = Local::now();

        Self {
            id: Uuid::new_v4(),

            created_at: now,
            log_date: now,

            task_key: task_key,
            task_description,

            message: args.message,

            tags: args.tags,
            projects: args.projects,
            activity_types: args.activity_types,

            start_time: args.start,
            end_time: args.end,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_path: PathBuf,
}
