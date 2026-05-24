use super::connection::get_connection;
use crate::errors::CliError;
use rusqlite::Connection;

pub fn list_entities_by_column(table: &str, column: &str) -> Result<Vec<String>, CliError> {
    let conn = get_connection()?;
    list_entities_by_column_with_conn(&conn, table, column)
}

pub fn list_entities_by_column_with_conn(
    conn: &Connection,
    table: &str,
    column: &str,
) -> Result<Vec<String>, CliError> {
    let sql = format!(
        "
        SELECT {}
        FROM {}
        ORDER BY {}
        ",
        column, table, column
    );

    let mut stmt = conn.prepare(&sql)?;

    let rows = stmt.query_map([], |row| row.get(0))?;

    Ok(rows.collect::<Result<Vec<String>, _>>()?)
}

pub fn list_named_entities(table: &str) -> Result<Vec<String>, CliError> {
    list_entities_by_column(table, "name")
}

pub fn list_tasks_with_descriptions() -> Result<Vec<(String, String)>, CliError> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare("SELECT task_key, description FROM tasks ORDER BY task_key")?;

    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;

    Ok(rows.collect::<Result<Vec<(String, String)>, _>>()?)
}
