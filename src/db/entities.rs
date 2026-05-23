use crate::errors::CliError;

use rusqlite::{Connection, params};

pub fn get_or_create_named_entity(
    conn: &Connection,
    table: &str,
    name: &str,
) -> Result<i64, CliError> {
    let insert_sql = format!(
        "
        INSERT OR IGNORE INTO {} (name)
        VALUES (?1)
        ",
        table
    );

    conn.execute(&insert_sql, params![name])?;

    let select_sql = format!(
        "
        SELECT id
        FROM {}
        WHERE name = ?1
        ",
        table
    );

    let id = conn.query_row(&select_sql, params![name], |row| row.get(0))?;

    Ok(id)
}
