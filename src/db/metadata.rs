use crate::errors::CliError;

use rusqlite::params;

use super::connection::get_connection;

pub fn list_tags() -> Result<Vec<String>, CliError> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare(
        "
        SELECT DISTINCT tag
        FROM log_entry_tags
        ORDER BY tag
        ",
    )?;

    let rows = stmt.query_map([], |row| row.get(0))?;

    Ok(rows.collect::<Result<Vec<String>, _>>()?)
}

pub fn list_projects() -> Result<Vec<String>, CliError> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare(
        "
        SELECT DISTINCT project
        FROM log_entry_projects
        ORDER BY project
        ",
    )?;

    let rows = stmt.query_map([], |row| row.get(0))?;

    Ok(rows.collect::<Result<Vec<String>, _>>()?)
}
