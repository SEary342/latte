use crate::{
    errors::CliError, models::LogEntry, storage::add_entry, ui::prompts::prompt_for_message,
};

pub fn handle(
    task_key: String,
    message: Option<String>,
    tags: Vec<String>,
    projects: Vec<String>,
    start: Option<u32>,
    end: Option<u32>,
) -> Result<(), CliError> {
    let message = match message {
        Some(msg) => Some(msg),
        None => {
            let input = prompt_for_message()?;

            if input.is_empty() { None } else { Some(input) }
        }
    };
    let entry = LogEntry::new(task_key, message, tags, projects, start, end);

    add_entry(entry.clone())?;

    println!("Created log entry:");
    println!("{:#?}", entry);

    Ok(())
}
