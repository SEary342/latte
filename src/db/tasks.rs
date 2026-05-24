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

/// Update the description of an existing task.
/// Returns `true` if a row was updated, `false` if the task key wasn't found.
pub fn update_task_description(task_key: &str, new_description: &str) -> Result<bool, CliError> {
    let conn = get_connection()?;
    let rows = conn.execute(
        "UPDATE tasks SET description = ?1 WHERE task_key = ?2",
        params![new_description, task_key],
    )?;
    Ok(rows > 0)
}
