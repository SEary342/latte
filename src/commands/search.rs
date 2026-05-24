use crate::{
    cli::SearchArgs,
    db::entries::{EntryFilter, list_entries},
    errors::CliError,
    ui::tables::render_log_table,
};

pub fn handle(args: SearchArgs) -> Result<(), CliError> {
    let filter = EntryFilter {
        task_key: args.key.as_deref(),
        tag: args.tag.as_deref(),
        project: args.project.as_deref(),
        activity_type: args.activity.as_deref(),
        ..Default::default()
    };

    let entries = list_entries(&filter)?;

    if entries.is_empty() {
        println!("No log entries found.");
        return Ok(());
    }

    render_log_table(&entries);
    Ok(())
}
