use crate::{db::entities::get_or_create_named_entity, errors::CliError, models::LogEntry};

use chrono::{DateTime, Local};
use rusqlite::{Connection, params};
use uuid::Uuid;

use super::connection::get_connection;

const TAGS_TABLE: &str = "tags";
const PROJECTS_TABLE: &str = "projects";
const ACTIVITY_TYPES_TABLE: &str = "activity_types";

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
        activity_types,
        start_time,
        end_time,
    } = entry;

    let entry_id = id.to_string();

    let mut conn = get_connection()?;

    let tx = conn.transaction()?;

    // Insert task if missing
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

    // Tags
    for tag in tags {
        let tag_id = get_or_create_named_entity(&tx, TAGS_TABLE, &tag)?;

        tx.execute(
            "
            INSERT OR IGNORE INTO log_entry_tags (
                entry_id,
                tag_id
            )
            VALUES (?1, ?2)
            ",
            params![&entry_id, tag_id],
        )?;
    }

    // Projects
    for project in projects {
        let project_id = get_or_create_named_entity(&tx, PROJECTS_TABLE, &project)?;

        tx.execute(
            "
            INSERT OR IGNORE INTO log_entry_projects (
                entry_id,
                project_id
            )
            VALUES (?1, ?2)
            ",
            params![&entry_id, project_id],
        )?;
    }

    // Activity types
    for activity_type in activity_types {
        let activity_type_id =
            get_or_create_named_entity(&tx, ACTIVITY_TYPES_TABLE, &activity_type)?;

        tx.execute(
            "
            INSERT OR IGNORE INTO log_entry_activity_types (
                entry_id,
                activity_type_id
            )
            VALUES (?1, ?2)
            ",
            params![&entry_id, activity_type_id],
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

        let entry_id = Uuid::parse_str(&id)?;

        let entry = LogEntry {
            id: entry_id,

            created_at,
            updated_at,

            task_key,
            task_description,

            message,

            tags: load_tags(&conn, &id)?,
            projects: load_projects(&conn, &id)?,
            activity_types: load_activity_types(&conn, &id)?,

            start_time,
            end_time,
        };

        entries.push(entry);
    }

    Ok(entries)
}

fn load_tags(conn: &Connection, entry_id: &str) -> Result<Vec<String>, CliError> {
    let mut stmt = conn.prepare(
        "
        SELECT t.name
        FROM log_entry_tags let
        JOIN tags t
            ON let.tag_id = t.id
        WHERE let.entry_id = ?1
        ORDER BY t.name
        ",
    )?;

    let rows = stmt.query_map(params![entry_id], |row| row.get(0))?;

    Ok(rows.collect::<Result<Vec<String>, _>>()?)
}

fn load_projects(conn: &Connection, entry_id: &str) -> Result<Vec<String>, CliError> {
    let mut stmt = conn.prepare(
        "
        SELECT p.name
        FROM log_entry_projects lep
        JOIN projects p
            ON lep.project_id = p.id
        WHERE lep.entry_id = ?1
        ORDER BY p.name
        ",
    )?;

    let rows = stmt.query_map(params![entry_id], |row| row.get(0))?;

    Ok(rows.collect::<Result<Vec<String>, _>>()?)
}

fn load_activity_types(conn: &Connection, entry_id: &str) -> Result<Vec<String>, CliError> {
    let mut stmt = conn.prepare(
        "
        SELECT at.name
        FROM log_entry_activity_types leat
        JOIN activity_types at
            ON leat.activity_type_id = at.id
        WHERE leat.entry_id = ?1
        ORDER BY at.name
        ",
    )?;

    let rows = stmt.query_map(params![entry_id], |row| row.get(0))?;

    Ok(rows.collect::<Result<Vec<String>, _>>()?)
}
