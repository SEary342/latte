use crate::{db::entities::get_or_create_named_entity, errors::CliError, models::LogEntry};

use chrono::{DateTime, Local};
use rusqlite::{ToSql, params};
use uuid::Uuid;

use super::connection::get_connection;

const TAGS_TABLE: &str = "tags";
const PROJECTS_TABLE: &str = "projects";
const ACTIVITY_TYPES_TABLE: &str = "activity_types";

pub fn add_entry(entry: LogEntry) -> Result<(), CliError> {
    let LogEntry {
        id,
        created_at,
        log_date,
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
            log_date,
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
            log_date.to_rfc3339(),
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

#[derive(Default, Debug)]
pub struct EntryFilter<'a> {
    pub task_key: Option<&'a str>,
    pub tag: Option<&'a str>,
    pub project: Option<&'a str>,
    pub activity_type: Option<&'a str>,
    pub start_time_gte: Option<u32>,
    pub end_time_lte: Option<u32>,
    pub start_date: Option<&'a str>,
    pub end_date: Option<&'a str>,
}

// --- DRY Helpers for SQL Generation ---

/// Generates the COALESCE/GROUP_CONCAT subquery for the SELECT clause
fn select_csv_subquery(join_table: &str, ref_table: &str, fk_col: &str) -> String {
    format!(
        "COALESCE((
            SELECT GROUP_CONCAT(ref.name, ',') 
            FROM {join_table} jt 
            JOIN {ref_table} ref ON jt.{fk_col} = ref.id 
            WHERE jt.entry_id = le.id
        ), '')"
    )
}

/// Dynamically pushes an EXISTS filter onto the WHERE clause if a value is provided
// Notice the change to &'a Option<&'a str>
fn push_relation_filter<'a>(
    query: &mut String,
    params: &mut Vec<&'a dyn ToSql>,
    filter_val: &'a Option<&'a str>,
    join_table: &str,
    ref_table: &str,
    fk_col: &str,
) {
    // ref val gives us &&str, which safely coerces to &dyn ToSql
    if let Some(val) = filter_val {
        query.push_str(&format!(
            " AND EXISTS (
                SELECT 1 FROM {join_table} jt
                JOIN {ref_table} ref ON jt.{fk_col} = ref.id
                WHERE jt.entry_id = le.id AND ref.name = ?
            )"
        ));
        params.push(val);
    }
}

// --- Main Function ---

pub fn list_entries(filter: &EntryFilter) -> Result<Vec<LogEntry>, CliError> {
    let conn = get_connection()?;

    let mut query = format!(
        "
        SELECT
            le.id, le.created_at, le.log_date, le.task_key,
            t.description, le.message, le.start_time, le.end_time,
            {} AS tags_csv,
            {} AS projects_csv,
            {} AS activities_csv
        FROM log_entries le
        JOIN tasks t ON le.task_key = t.task_key
        WHERE 1=1
        ",
        select_csv_subquery("log_entry_tags", "tags", "tag_id"),
        select_csv_subquery("log_entry_projects", "projects", "project_id"),
        select_csv_subquery(
            "log_entry_activity_types",
            "activity_types",
            "activity_type_id"
        )
    );

    let mut params: Vec<&dyn ToSql> = Vec::new();

    if let Some(ref tk) = filter.task_key {
        query.push_str(" AND le.task_key = ?");
        params.push(tk);
    }
    if let Some(ref sd) = filter.start_date {
        // strftime returns 'YYYY-MM-DD' from the RFC3339 string
        query.push_str(" AND strftime('%Y-%m-%d', le.log_date) >= ?");
        params.push(sd);
    }
    if let Some(ref ed) = filter.end_date {
        query.push_str(" AND strftime('%Y-%m-%d', le.log_date) <= ?");
        params.push(ed);
    }
    if let Some(ref st) = filter.start_time_gte {
        query.push_str(" AND le.start_time >= ?");
        params.push(st);
    }
    if let Some(ref et) = filter.end_time_lte {
        query.push_str(" AND le.end_time <= ?");
        params.push(et);
    }

    push_relation_filter(
        &mut query,
        &mut params,
        &filter.tag,
        "log_entry_tags",
        "tags",
        "tag_id",
    );
    push_relation_filter(
        &mut query,
        &mut params,
        &filter.project,
        "log_entry_projects",
        "projects",
        "project_id",
    );
    push_relation_filter(
        &mut query,
        &mut params,
        &filter.activity_type,
        "log_entry_activity_types",
        "activity_types",
        "activity_type_id",
    );

    query.push_str(" ORDER BY le.start_time DESC");

    let mut stmt = conn.prepare(&query)?;

    let rows = stmt.query_map(&*params, |row| {
        // Look up by column name. The compiler infers the rusqlite extraction
        // type based on the variable type declarations here.
        let id: String = row.get("id")?;
        let created_at: String = row.get("created_at")?;
        let log_date: String = row.get("log_date")?;
        let task_key: String = row.get("task_key")?;
        let task_description: String = row.get("description")?;

        // Optionals
        let message: Option<String> = row.get("message")?;
        let start_time: Option<u32> = row.get("start_time")?;
        let end_time: Option<u32> = row.get("end_time")?;

        // Our dynamically generated CSV strings
        let tags_csv: String = row.get("tags_csv")?;
        let projects_csv: String = row.get("projects_csv")?;
        let activities_csv: String = row.get("activities_csv")?;

        Ok((
            id,
            created_at,
            log_date,
            task_key,
            task_description,
            message,
            start_time,
            end_time,
            tags_csv,
            projects_csv,
            activities_csv,
        ))
    })?;

    // Helper closure to handle string splitting safely
    let split_csv = |csv: String| -> Vec<String> {
        if csv.is_empty() {
            Vec::new()
        } else {
            csv.split(',').map(|s| s.to_string()).collect()
        }
    };

    let mut entries = Vec::new();

    for row in rows {
        let (
            id,
            created_at,
            log_date,
            task_key,
            task_description,
            message,
            start_time,
            end_time,
            tags_csv,
            projects_csv,
            activities_csv,
        ) = row?;

        entries.push(LogEntry {
            id: Uuid::parse_str(&id)?,
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Local),
            log_date: DateTime::parse_from_rfc3339(&log_date)?.with_timezone(&Local),
            task_key,
            task_description,
            message,

            tags: split_csv(tags_csv),
            projects: split_csv(projects_csv),
            activity_types: split_csv(activities_csv),

            start_time,
            end_time,
        });
    }

    Ok(entries)
}

/// Look up a single log entry whose UUID starts with `prefix` (case-insensitive).
/// Returns `None` if no match is found, or an error if more than one entry matches.
pub fn get_entry_by_prefix(prefix: &str) -> Result<Option<LogEntry>, CliError> {
    let filter = EntryFilter::default();
    let all = list_entries(&filter)?;

    let prefix_lower = prefix.to_lowercase();
    let matches: Vec<LogEntry> = all
        .into_iter()
        .filter(|e| e.id.to_string().to_lowercase().starts_with(&prefix_lower))
        .collect();

    match matches.len() {
        0 => Ok(None),
        1 => Ok(Some(matches.into_iter().next().unwrap())),
        _ => Err(CliError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "Ambiguous prefix '{}': matches {} entries",
                prefix,
                matches.len()
            ),
        ))),
    }
}

/// Update the scalar fields of a log entry (message, start_time, end_time) and
/// replace its tag / project / activity-type relations.
pub fn update_entry(
    entry_id: Uuid,
    message: Option<String>,
    start_time: Option<u32>,
    end_time: Option<u32>,
    tags: Vec<String>,
    projects: Vec<String>,
    activity_types: Vec<String>,
) -> Result<(), CliError> {
    let entry_id_str = entry_id.to_string();
    let mut conn = get_connection()?;
    let tx = conn.transaction()?;

    // Update scalar fields
    tx.execute(
        "UPDATE log_entries SET message = ?1, start_time = ?2, end_time = ?3 WHERE id = ?4",
        params![message, start_time, end_time, &entry_id_str],
    )?;

    // Replace tag relations
    tx.execute(
        "DELETE FROM log_entry_tags WHERE entry_id = ?1",
        params![&entry_id_str],
    )?;
    for tag in &tags {
        let tag_id = get_or_create_named_entity(&tx, TAGS_TABLE, tag)?;
        tx.execute(
            "INSERT OR IGNORE INTO log_entry_tags (entry_id, tag_id) VALUES (?1, ?2)",
            params![&entry_id_str, tag_id],
        )?;
    }

    // Replace project relations
    tx.execute(
        "DELETE FROM log_entry_projects WHERE entry_id = ?1",
        params![&entry_id_str],
    )?;
    for project in &projects {
        let project_id = get_or_create_named_entity(&tx, PROJECTS_TABLE, project)?;
        tx.execute(
            "INSERT OR IGNORE INTO log_entry_projects (entry_id, project_id) VALUES (?1, ?2)",
            params![&entry_id_str, project_id],
        )?;
    }

    // Replace activity-type relations
    tx.execute(
        "DELETE FROM log_entry_activity_types WHERE entry_id = ?1",
        params![&entry_id_str],
    )?;
    for activity_type in &activity_types {
        let at_id = get_or_create_named_entity(&tx, ACTIVITY_TYPES_TABLE, activity_type)?;
        tx.execute(
            "INSERT OR IGNORE INTO log_entry_activity_types (entry_id, activity_type_id) VALUES (?1, ?2)",
            params![&entry_id_str, at_id],
        )?;
    }

    tx.commit()?;
    Ok(())
}
