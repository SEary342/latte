use chrono::Local;

use crate::{
    cli::SearchArgs,
    db::entries::{EntryFilter, list_entries},
    errors::CliError,
    ui::tables::render_log_table,
};

pub fn handle(args: SearchArgs) -> Result<(), CliError> {
    let today = Local::now().format("%Y-%m-%d").to_string();

    // Date Logic: Handle --today OR --start-date/--end-date
    let (start_date, end_date) = if args.today {
        (Some(today.as_str()), Some(today.as_str()))
    } else {
        (args.start_date.as_deref(), args.end_date.as_deref())
    };

    // Time Logic: Parse string-based time inputs (e.g., "1300") to u32
    let start_time_gte = args.start_time.and_then(|s| s.parse::<u32>().ok());
    let end_time_lte = args.end_time.and_then(|s| s.parse::<u32>().ok());

    let filter = EntryFilter {
        task_key: args.key.as_deref(),
        tag: args.tag.as_deref(),
        project: args.project.as_deref(),
        activity_type: args.activity.as_deref(),
        start_date,
        end_date,
        start_time_gte,
        end_time_lte,
    };

    let entries = list_entries(&filter)?;

    if entries.is_empty() {
        println!("No log entries found.");
        return Ok(());
    }

    render_log_table(&entries);
    Ok(())
}
