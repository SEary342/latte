use crate::models::LogEntry;

use tabled::{Table, Tabled};

#[derive(Tabled)]
struct LogRow {
    id: String,
    log_date: String,
    task: String,
    message: String,
    tags: String,
    projects: String,
    activity_types: String,
    time: String,
    created: String,
}

#[derive(Tabled)]
struct MetadataRow {
    #[tabled(rename = "Item Name")]
    name: String,
}

#[derive(Tabled)]
struct TaskRow {
    #[tabled(rename = "Task Key")]
    key: String,
    #[tabled(rename = "Description")]
    description: String,
}

pub fn render_log_table(entries: &[LogEntry]) {
    let rows: Vec<LogRow> = entries
        .iter()
        .map(|entry| LogRow {
            id: entry.id.to_string()[..8].to_string(),

            log_date: entry.log_date.format("%Y-%m-%d %H:%M").to_string(),

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
        })
        .collect();

    let table = Table::new(rows);

    println!("{}", table);
}

pub fn render_metadata_table(title: &str, items: &[String]) {
    if items.is_empty() {
        println!("No {} found.", title.to_lowercase());
        return;
    }

    let rows: Vec<MetadataRow> = items
        .iter()
        .map(|item| MetadataRow { name: item.clone() })
        .collect();

    let table = Table::new(rows);

    // Optional aesthetic touch: giving the table a title header styling
    println!("\n=== All {} ===", title);
    println!("{}", table);
}

pub fn render_tasks_table(tasks: &[(String, String)]) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    let rows: Vec<TaskRow> = tasks
        .iter()
        .map(|(key, desc)| TaskRow {
            key: key.clone(),
            description: desc.clone(),
        })
        .collect();

    let table = Table::new(rows);
    println!("\n=== All Tasks ===");
    println!("{}", table);
}
