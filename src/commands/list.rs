use crate::{errors::CliError, storage::load_database, ui::tables::render_log_table};

pub fn handle() -> Result<(), CliError> {
    let db = load_database()?;

    if db.entries.is_empty() {
        println!("No log entries found.");
        return Ok(());
    }

    render_log_table(&db.entries);

    Ok(())
}
