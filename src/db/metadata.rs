use crate::errors::CliError;

use rusqlite::Connection;

use super::connection::get_connection;

pub fn list_named_entities(table: &str) -> Result<Vec<String>, CliError> {
    let conn = get_connection()?;

    list_named_entities_with_conn(&conn, table)
}

pub fn list_named_entities_with_conn(
    conn: &Connection,
    table: &str,
) -> Result<Vec<String>, CliError> {
    let sql = format!(
        "
        SELECT name
        FROM {}
        ORDER BY name
        ",
        table
    );

    let mut stmt = conn.prepare(&sql)?;

    let rows = stmt.query_map([], |row| row.get(0))?;

    Ok(rows.collect::<Result<Vec<String>, _>>()?)
}
