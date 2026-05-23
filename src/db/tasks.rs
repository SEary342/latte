use crate::errors::CliError;

use rusqlite::params;

use super::connection::get_connection;

pub fn get_task_description(task_key: &str) -> Result<Option<String>, CliError> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare("SELECT description FROM tasks WHERE task_key = ?1")?;

    let mut rows = stmt.query(params![task_key])?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}
