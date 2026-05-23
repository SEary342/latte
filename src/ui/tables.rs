use crate::models::LogEntry;

use tabled::{Table, Tabled};

#[derive(Tabled)]
struct LogRow {
    id: String,
    task: String,
    message: String,
    tags: String,
    projects: String,
    activity_types: String,
    time: String,
    created: String,
    updated: String,
}

pub fn render_log_table(entries: &[LogEntry]) {
    let rows: Vec<LogRow> = entries
        .iter()
        .map(|entry| LogRow {
            id: entry.id.to_string()[..8].to_string(),

            task: if entry.task_description.is_empty() {
                entry.task_key.clone()
            } else {
                format!("{} ({})", entry.task_key, entry.task_description)
            },

            message: entry.message.clone().unwrap_or_default(),

            tags: entry.tags.join(","),

            projects: entry.projects.join(","),

            activity_types: entry.activity_types.join(","),

            time: entry.formatted_time(),

            created: entry.created_at.format("%Y-%m-%d %H:%M").to_string(),

            updated: entry.updated_at.format("%Y-%m-%d %H:%M").to_string(),
        })
        .collect();

    let table = Table::new(rows);

    println!("{}", table);
}
