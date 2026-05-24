use super::connection::get_connection;
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

/// Rename an entity in a named-entity table (tags, projects, activity_types).
/// Returns the number of rows updated (0 if not found).
pub fn rename_named_entity(table: &str, old_name: &str, new_name: &str) -> Result<usize, CliError> {
    let conn = get_connection()?;
    let sql = format!("UPDATE {} SET name = ?1 WHERE name = ?2", table);
    let rows = conn.execute(&sql, params![new_name, old_name])?;
    Ok(rows)
}
