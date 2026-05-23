use crate::{
    db::{
        entries::add_entry,
        metadata::{list_projects, list_tags},
        tasks::get_task_description,
    },
    errors::CliError,
    models::LogEntry,
    ui::prompts::{
        prompt_for_message, prompt_for_multi_value, prompt_for_task_description, prompt_for_time,
    },
};

pub fn handle(
    task_key: String,
    message: Option<String>,
    tags: Vec<String>,
    projects: Vec<String>,
    start_time: Option<u32>,
    end_time: Option<u32>,
) -> Result<(), CliError> {
    let task_description = match get_task_description(&task_key)? {
        Some(desc) => desc,

        None => prompt_for_task_description(&task_key)?,
    };

    let message = match message {
        Some(msg) => Some(msg),

        None => {
            let input = prompt_for_message()?;

            if input.is_empty() { None } else { Some(input) }
        }
    };

    let tags = if tags.is_empty() {
        prompt_for_multi_value("Tags (comma separated)", list_tags()?)?
    } else {
        tags
    };

    let projects = if projects.is_empty() {
        prompt_for_multi_value("Projects (comma separated)", list_projects()?)?
    } else {
        projects
    };

    let start_time = match start_time {
        Some(time) => Some(time),

        None => prompt_for_time("Start Time (HHMM)")?,
    };

    let end_time = match end_time {
        Some(time) => Some(time),

        None => prompt_for_time("End Time (HHMM)")?,
    };

    let entry = LogEntry::new(
        task_key,
        task_description,
        message,
        tags,
        projects,
        start_time,
        end_time,
    );

    let entry_id = entry.id;

    add_entry(entry)?;

    println!("Created log entry: {}", &entry_id.to_string()[..8]);

    Ok(())
}
