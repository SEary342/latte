use crate::{errors::CliError, models::LogEntry};

use chrono::{DateTime, Local};
use rusqlite::{Connection, params};
use uuid::Uuid;

use super::connection::get_connection;

pub fn add_entry(entry: LogEntry) -> Result<(), CliError> {
    let LogEntry {
        id,
        created_at,
        updated_at,
        task_key,
        task_description,
        message,
        tags,
        projects,
        start_time,
        end_time,
    } = entry;

    let entry_id = id.to_string();

    let mut conn = get_connection()?;

    let tx = conn.transaction()?;

    // Upsert task
    tx.execute(
        "
        INSERT OR IGNORE INTO tasks (
            task_key,
            description
        )
        VALUES (?1, ?2)
        ",
        params![task_key, task_description],
    )?;

    // Insert log entry
    tx.execute(
        "
        INSERT INTO log_entries (
            id,
            created_at,
            updated_at,
            task_key,
            message,
            start_time,
            end_time
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ",
        params![
            &entry_id,
            created_at.to_rfc3339(),
            updated_at.to_rfc3339(),
            task_key,
            message,
            start_time,
            end_time,
        ],
    )?;

    // Insert tags
    for tag in tags {
        tx.execute(
            "
            INSERT OR IGNORE INTO log_entry_tags (
                entry_id,
                tag
            )
            VALUES (?1, ?2)
            ",
            params![&entry_id, tag],
        )?;
    }

    // Insert projects
    for project in projects {
        tx.execute(
            "
            INSERT OR IGNORE INTO log_entry_projects (
                entry_id,
                project
            )
            VALUES (?1, ?2)
            ",
            params![&entry_id, project],
        )?;
    }

    tx.commit()?;

    Ok(())
}

pub fn list_entries() -> Result<Vec<LogEntry>, CliError> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare(
        "
        SELECT
            le.id,
            le.created_at,
            le.updated_at,
            le.task_key,
            t.description,
            le.message,
            le.start_time,
            le.end_time
        FROM log_entries le
        JOIN tasks t
            ON le.task_key = t.task_key
        ORDER BY le.start_time DESC
        ",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, Option<String>>(5)?,
            row.get::<_, Option<u32>>(6)?,
            row.get::<_, Option<u32>>(7)?,
        ))
    })?;

    let mut entries = Vec::new();

    for row in rows {
        let (id, created_at, updated_at, task_key, task_description, message, start_time, end_time) =
            row?;

        let created_at = DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Local);

        let updated_at = DateTime::parse_from_rfc3339(&updated_at)?.with_timezone(&Local);

        let mut entry = LogEntry {
            id: Uuid::parse_str(&id)?,

            created_at,
            updated_at,

            task_key,
            task_description,

            message,

            tags: Vec::new(),
            projects: Vec::new(),

            start_time,
            end_time,
        };

        entry.tags = load_tags(&conn, &entry.id.to_string())?;

        entry.projects = load_projects(&conn, &entry.id.to_string())?;

        entries.push(entry);
    }

    Ok(entries)
}

fn load_tags(conn: &Connection, entry_id: &str) -> Result<Vec<String>, CliError> {
    let mut stmt = conn.prepare(
        "
        SELECT tag
        FROM log_entry_tags
        WHERE entry_id = ?1
        ORDER BY tag
        ",
    )?;

    let rows = stmt.query_map(params![entry_id], |row| row.get(0))?;

    Ok(rows.collect::<Result<Vec<String>, _>>()?)
}

fn load_projects(conn: &Connection, entry_id: &str) -> Result<Vec<String>, CliError> {
    let mut stmt = conn.prepare(
        "
        SELECT project
        FROM log_entry_projects
        WHERE entry_id = ?1
        ORDER BY project
        ",
    )?;

    let rows = stmt.query_map(params![entry_id], |row| row.get(0))?;

    Ok(rows.collect::<Result<Vec<String>, _>>()?)
}
