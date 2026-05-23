use crate::{
    cli::AddArgs,
    db::{entries::add_entry, metadata::list_named_entities, tasks::get_task_description},
    errors::CliError,
    models::LogEntry,
    ui::prompts::{
        prompt_for_message, prompt_for_multi_value, prompt_for_task_description, prompt_for_time,
    },
};

pub fn handle(mut args: AddArgs) -> Result<(), CliError> {
    let task_description = match get_task_description(&args.task_key)? {
        Some(desc) => desc,
        None => prompt_for_task_description(&args.task_key)?,
    };

    if args.message.is_none() {
        let input = prompt_for_message()?;
        args.message = if input.is_empty() { None } else { Some(input) };
    }

    if !args.quick {
        if args.tags.is_empty() {
            args.tags =
                prompt_for_multi_value("Tags (comma separated)", list_named_entities("tags")?)?;
        }

        if args.projects.is_empty() {
            args.projects = prompt_for_multi_value(
                "Projects (comma separated)",
                list_named_entities("projects")?,
            )?;
        }

        if args.activity_types.is_empty() {
            args.activity_types = prompt_for_multi_value(
                "Activity Types (comma separated)",
                list_named_entities("activity_types")?,
            )?;
        }

        if args.start.is_none() {
            args.start = prompt_for_time("Start Time (HHMM)")?;
        }

        if args.end.is_none() {
            args.end = prompt_for_time("End Time (HHMM)")?;
        }
    }

    let entry = LogEntry::from_add_args(args, task_description);

    let entry_id = entry.id;

    add_entry(entry)?;

    println!("Created log entry: {}", &entry_id.to_string()[..8]);

    Ok(())
}
