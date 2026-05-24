use crate::errors::CliError;

use rusqlite::Connection;

pub fn run_migrations(conn: &Connection) -> Result<(), CliError> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS tasks (
            task_key TEXT PRIMARY KEY,
            description TEXT NOT NULL DEFAULT ''
        );

        CREATE TABLE IF NOT EXISTS log_entries (
            id TEXT PRIMARY KEY,
            created_at TEXT NOT NULL,
            log_date TEXT NOT NULL,
            task_key TEXT NOT NULL,
            message TEXT,
            start_time INTEGER,
            end_time INTEGER,

            FOREIGN KEY(task_key)
                REFERENCES tasks(task_key)
        );

        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS activity_types (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS log_entry_tags (
            entry_id TEXT NOT NULL,
            tag_id INTEGER NOT NULL,

            PRIMARY KEY(entry_id, tag_id),

            FOREIGN KEY(entry_id)
                REFERENCES log_entries(id)
                ON DELETE CASCADE,

            FOREIGN KEY(tag_id)
                REFERENCES tags(id)
                ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS log_entry_projects (
            entry_id TEXT NOT NULL,
            project_id INTEGER NOT NULL,

            PRIMARY KEY(entry_id, project_id),

            FOREIGN KEY(entry_id)
                REFERENCES log_entries(id)
                ON DELETE CASCADE,

            FOREIGN KEY(project_id)
                REFERENCES projects(id)
                ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS log_entry_activity_types (
            entry_id TEXT NOT NULL,
            activity_type_id INTEGER NOT NULL,

            PRIMARY KEY(entry_id, activity_type_id),

            FOREIGN KEY(entry_id)
                REFERENCES log_entries(id)
                ON DELETE CASCADE,

            FOREIGN KEY(activity_type_id)
                REFERENCES activity_types(id)
                ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_log_entries_task_key
            ON log_entries(task_key);

        CREATE INDEX IF NOT EXISTS idx_log_entries_start_time
            ON log_entries(start_time);

        CREATE INDEX IF NOT EXISTS idx_tags_name
            ON tags(name);

        CREATE INDEX IF NOT EXISTS idx_projects_name
            ON projects(name);

        CREATE INDEX IF NOT EXISTS idx_activity_types_name
            ON activity_types(name);

        CREATE INDEX IF NOT EXISTS idx_log_entry_tags_tag_id
            ON log_entry_tags(tag_id);

        CREATE INDEX IF NOT EXISTS idx_log_entry_projects_project_id
            ON log_entry_projects(project_id);

        CREATE INDEX IF NOT EXISTS idx_log_entry_activity_types_activity_type_id
            ON log_entry_activity_types(activity_type_id);
        ",
    )?;

    Ok(())
}
