use crate::{
    cli::AddArgs,
    db::{
        entries::add_entry,
        metadata::{list_entities_by_column, list_named_entities},
        tasks::get_task_description,
    },
    errors::CliError,
    models::LogEntry,
    ui::{
        header::print_header,
        prompts::{
            prompt_for_message, prompt_for_multi_value, prompt_for_task_description,
            prompt_for_time, prompt_for_value,
        },
        util::clear_screen,
    },
};

pub fn handle(mut args: AddArgs) -> Result<(), CliError> {
    println!("Add New Log\n");
    let mut task_key_str = args.task_key.clone().unwrap_or_default();
    if args.task_key.is_none() {
        let input = prompt_for_value(
            "Task Key",
            list_entities_by_column("tasks", "task_key")?,
            false,
        )?;
        task_key_str = input;
    }

    let task_description = match get_task_description(&task_key_str)? {
        Some(desc) => desc,
        None => {
            print_header(&task_key_str, "New Task"); // Placeholder description
            prompt_for_task_description(&task_key_str)?
        }
    };

    // Helper closure to show header consistently
    print_header(&task_key_str, &task_description);

    if args.message.is_none() {
        let input = prompt_for_message(None)?;
        args.message = if input.is_empty() { None } else { Some(input) };
    }

    if !args.quick {
        if args.tags.is_empty() {
            args.tags = prompt_for_multi_value(
                "Tags (comma separated)",
                list_named_entities("tags")?,
                &[],
            )?;
        }

        if args.projects.is_empty() {
            args.projects = prompt_for_multi_value(
                "Projects (comma separated)",
                list_named_entities("projects")?,
                &[],
            )?;
        }

        if args.activity_types.is_empty() {
            args.activity_types = prompt_for_multi_value(
                "Activity Types (comma separated)",
                list_named_entities("activity_types")?,
                &[],
            )?;
        }

        if args.start.is_none() {
            args.start = prompt_for_time("Start Time (HHMM)", None)?;
        }

        if args.end.is_none() {
            args.end = prompt_for_time("End Time (HHMM)", None)?;
        }
    }

    clear_screen()?;

    let entry = LogEntry::from_add_args(args, task_key_str, task_description);
    let entry_id = entry.id;

    add_entry(entry)?;

    println!("Created log entry: {}", &entry_id.to_string()[..8]);

    Ok(())
}
