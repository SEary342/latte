use crate::errors::CliError;

use super::connection::get_connection;

pub fn cleanup_unused_tasks(dry_run: bool) -> Result<u64, CliError> {
    let conn = get_connection()?;

    let count: i64 = conn.query_row(
        r#"
        SELECT COUNT(*)
        FROM tasks
        WHERE task_key NOT IN (
            SELECT DISTINCT task_key
            FROM log_entries
        )
        "#,
        [],
        |row| row.get(0),
    )?;

    if dry_run {
        return Ok(count as u64);
    }

    conn.execute(
        r#"
        DELETE FROM tasks
        WHERE task_key NOT IN (
            SELECT DISTINCT task_key
            FROM log_entries
        )
        "#,
        [],
    )?;

    Ok(count as u64)
}

pub fn cleanup_unused_tags(dry_run: bool) -> Result<u64, CliError> {
    let conn = get_connection()?;

    let count: i64 = conn.query_row(
        r#"
        SELECT COUNT(*)
        FROM log_entry_tags
        WHERE entry_id NOT IN (
            SELECT id
            FROM log_entries
        )
        "#,
        [],
        |row| row.get(0),
    )?;

    if dry_run {
        return Ok(count as u64);
    }

    conn.execute(
        r#"
        DELETE FROM log_entry_tags
        WHERE entry_id NOT IN (
            SELECT id
            FROM log_entries
        )
        "#,
        [],
    )?;

    Ok(count as u64)
}

pub fn cleanup_unused_projects(dry_run: bool) -> Result<u64, CliError> {
    let conn = get_connection()?;

    let count: i64 = conn.query_row(
        r#"
        SELECT COUNT(*)
        FROM log_entry_projects
        WHERE entry_id NOT IN (
            SELECT id
            FROM log_entries
        )
        "#,
        [],
        |row| row.get(0),
    )?;

    if dry_run {
        return Ok(count as u64);
    }

    conn.execute(
        r#"
        DELETE FROM log_entry_projects
        WHERE entry_id NOT IN (
            SELECT id
            FROM log_entries
        )
        "#,
        [],
    )?;

    Ok(count as u64)
}
