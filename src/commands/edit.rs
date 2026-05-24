use colored::Colorize;

use crate::{
    cli::EditArgs,
    db::{
        entities::rename_named_entity,
        entries::{EntryFilter, get_entry_by_prefix, list_entries, update_entry},
        metadata::{list_entities_by_column, list_named_entities},
        tasks::{get_task_description, update_task_description},
    },
    errors::CliError,
    ui::{
        header::print_header,
        prompts::{prompt_for_message, prompt_for_multi_value, prompt_for_time, prompt_for_value},
        util::clear_screen,
    },
};

// ─── Entry-point ────────────────────────────────────────────────────────────

pub fn handle(args: EditArgs) -> Result<(), CliError> {
    // Global entity rename flows take priority when no entry id is given.
    if let Some(ref old_name) = args.tag {
        return handle_rename_entity("tags", "Tag", old_name);
    }
    if let Some(ref old_name) = args.project {
        return handle_rename_entity("projects", "Project", old_name);
    }
    if let Some(ref old_name) = args.activity {
        return handle_rename_entity("activity_types", "Activity Type", old_name);
    }
    if let Some(ref task_key) = args.task {
        return handle_edit_task(task_key);
    }

    // Log entry editing flow
    handle_edit_entry(args)
}

// ─── Log Entry Edit ──────────────────────────────────────────────────────────

fn handle_edit_entry(args: EditArgs) -> Result<(), CliError> {
    // Resolve the entry: either from the CLI id or by interactive selection.
    let entry = match &args.id {
        Some(prefix) => match get_entry_by_prefix(prefix)? {
            Some(e) => e,
            None => {
                println!(
                    "{}",
                    format!("No entry found matching prefix '{}'.", prefix).red()
                );
                return Ok(());
            }
        },
        None => {
            // Interactive selection: list entries and let the user pick one.
            let all = list_entries(&EntryFilter::default())?;
            if all.is_empty() {
                println!("{}", "No log entries found.".yellow());
                return Ok(());
            }

            println!("Edit Log Entry\n");
            println!(
                "{}",
                "Select an entry to edit (type the short ID):".dimmed()
            );
            for e in &all {
                let short_id = &e.id.to_string()[..8];
                let task_info = if e.task_description.is_empty() {
                    e.task_key.clone()
                } else {
                    format!("{} ({})", e.task_key, e.task_description)
                };
                let msg = e.message.as_deref().unwrap_or("—");
                let date = e.log_date.format("%Y-%m-%d").to_string();
                println!(
                    "  {} │ {} │ {} │ {}",
                    short_id.cyan(),
                    date.dimmed(),
                    task_info.bold(),
                    msg
                );
            }
            println!();

            let ids: Vec<String> = all
                .iter()
                .map(|e| e.id.to_string()[..8].to_string())
                .collect();
            let prefix = prompt_for_value("Entry short ID", ids, false)?;

            match get_entry_by_prefix(&prefix)? {
                Some(e) => e,
                None => {
                    println!("{}", format!("No entry found matching '{}'.", prefix).red());
                    return Ok(());
                }
            }
        }
    };

    clear_screen()?;
    print_header(&entry.task_key, &entry.task_description);
    println!(
        "{}",
        "Editing log entry — press Enter to keep current value.\n".dimmed()
    );

    // ── Field-by-field editing with defaults ────────────────────────────────

    // Message
    let new_message = if args.message.is_some() {
        args.message.clone()
    } else {
        let current = entry.message.as_deref().unwrap_or("");
        let input = prompt_for_message(Some(current))?;
        if input.is_empty() { None } else { Some(input) }
    };

    // Tags
    let new_tags = if !args.entry_tags.is_empty() {
        args.entry_tags.clone()
    } else {
        prompt_for_multi_value(
            "Tags (comma separated)",
            list_named_entities("tags")?,
            &entry.tags,
        )?
    };

    // Projects
    let new_projects = if !args.entry_projects.is_empty() {
        args.entry_projects.clone()
    } else {
        prompt_for_multi_value(
            "Projects (comma separated)",
            list_named_entities("projects")?,
            &entry.projects,
        )?
    };

    // Activity types
    let new_activities = if !args.entry_activities.is_empty() {
        args.entry_activities.clone()
    } else {
        prompt_for_multi_value(
            "Activity Types (comma separated)",
            list_named_entities("activity_types")?,
            &entry.activity_types,
        )?
    };

    // Start time
    let new_start = if args.start.is_some() {
        args.start
    } else {
        prompt_for_time("Start Time (HHMM)", entry.start_time)?
    };

    // End time
    let new_end = if args.end.is_some() {
        args.end
    } else {
        prompt_for_time("End Time (HHMM)", entry.end_time)?
    };

    clear_screen()?;

    update_entry(
        entry.id,
        new_message,
        new_start,
        new_end,
        new_tags,
        new_projects,
        new_activities,
    )?;

    println!(
        "{}",
        format!("Updated log entry {}.", &entry.id.to_string()[..8]).green()
    );

    Ok(())
}

// ─── Global Entity Rename ────────────────────────────────────────────────────

fn handle_rename_entity(table: &str, label: &str, old_name: &str) -> Result<(), CliError> {
    // Resolve old name: if passed as a bare flag value, use it; otherwise prompt.
    let resolved_old = if old_name.is_empty() {
        let options = list_named_entities(table)?;
        if options.is_empty() {
            println!(
                "{}",
                format!("No {}s found.", label.to_lowercase()).yellow()
            );
            return Ok(());
        }
        prompt_for_value(&format!("{} to rename", label), options, false)?
    } else {
        old_name.to_string()
    };

    println!(
        "\n{} {} {}\n",
        "Renaming".dimmed(),
        resolved_old.bold().cyan(),
        format!("({})", label).dimmed()
    );

    let options = list_named_entities(table)?;
    let new_name = prompt_for_value(
        &format!("New {} name", label.to_lowercase()),
        options,
        false,
    )?;

    if new_name.trim().is_empty() {
        println!("{}", "Rename cancelled — new name was empty.".yellow());
        return Ok(());
    }

    let rows = rename_named_entity(table, &resolved_old, new_name.trim())?;

    if rows == 0 {
        println!(
            "{}",
            format!("{} '{}' not found.", label, resolved_old).red()
        );
    } else {
        println!(
            "{}",
            format!(
                "Renamed {} '{}' → '{}' (affects all log entries).",
                label.to_lowercase(),
                resolved_old,
                new_name.trim()
            )
            .green()
        );
    }

    Ok(())
}

// ─── Task Description Edit ───────────────────────────────────────────────────

fn handle_edit_task(task_key: &str) -> Result<(), CliError> {
    // If the caller passed an empty string, prompt for the task key.
    let resolved_key = if task_key.is_empty() {
        let options = list_entities_by_column("tasks", "task_key")?;
        if options.is_empty() {
            println!("{}", "No tasks found.".yellow());
            return Ok(());
        }
        prompt_for_value("Task key to edit", options, false)?
    } else {
        task_key.to_string()
    };

    let current_desc = match get_task_description(&resolved_key)? {
        Some(d) => d,
        None => {
            println!("{}", format!("Task '{}' not found.", resolved_key).red());
            return Ok(());
        }
    };

    print_header(&resolved_key, &current_desc);
    println!(
        "{}",
        "Edit task description — press Enter to keep current value.\n".dimmed()
    );

    use dialoguer::Input;
    let new_desc: String = Input::new()
        .with_prompt("Task description")
        .with_initial_text(&current_desc)
        .allow_empty(true)
        .interact_text()?;

    let new_desc = new_desc.trim();

    if new_desc.is_empty() || new_desc == current_desc {
        println!("{}", "No changes made.".dimmed());
        return Ok(());
    }

    update_task_description(&resolved_key, new_desc)?;
    println!(
        "{}",
        format!("Updated description for task '{}'.", resolved_key).green()
    );

    Ok(())
}
