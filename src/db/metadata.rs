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
