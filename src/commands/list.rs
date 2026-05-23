use crate::{db::entries::list_entries, errors::CliError, ui::tables::render_log_table};

pub fn handle() -> Result<(), CliError> {
    let entries = list_entries()?;

    if entries.is_empty() {
        println!("No log entries found.");
        return Ok(());
    }

    render_log_table(&entries);

    Ok(())
}
