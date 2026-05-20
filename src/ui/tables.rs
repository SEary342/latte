use crate::models::LogEntry;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct LogRow {
    id: String,
    task: String,
    message: String,
    tags: String,
    projects: String,
    time: String,
    created: String,
    updated: String,
}

pub fn render_log_table(entries: &[LogEntry]) {
    let rows: Vec<LogRow> = entries
        .iter()
        .map(|entry| LogRow {
            id: entry.id.to_string()[..8].to_string(),

            task: entry.task_key.clone(),

            message: entry.message.clone().unwrap_or_default(),

            tags: entry.tags.join(","),

            projects: entry.projects.join(","),

            time: entry.formatted_time(),

            created: entry.created_at.format("%Y-%m-%d %H:%M").to_string(),

            updated: entry.updated_at.format("%Y-%m-%d %H:%M").to_string(),
        })
        .collect();

    let table = Table::new(rows);

    println!("{}", table);
}
