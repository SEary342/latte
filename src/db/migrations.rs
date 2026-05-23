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
            updated_at TEXT NOT NULL,
            task_key TEXT NOT NULL,
            message TEXT,
            start_time INTEGER,
            end_time INTEGER,
            FOREIGN KEY(task_key) REFERENCES tasks(task_key)
        );

        CREATE TABLE IF NOT EXISTS log_entry_tags (
            entry_id TEXT NOT NULL,
            tag TEXT NOT NULL,
            PRIMARY KEY(entry_id, tag),
            FOREIGN KEY(entry_id) REFERENCES log_entries(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS log_entry_projects (
            entry_id TEXT NOT NULL,
            project TEXT NOT NULL,
            PRIMARY KEY(entry_id, project),
            FOREIGN KEY(entry_id) REFERENCES log_entries(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_log_entries_task_key
        ON log_entries(task_key);

        CREATE INDEX IF NOT EXISTS idx_log_entries_start_time
        ON log_entries(start_time);

        CREATE INDEX IF NOT EXISTS idx_log_entry_tags_tag
        ON log_entry_tags(tag);

        CREATE INDEX IF NOT EXISTS idx_log_entry_projects_project
        ON log_entry_projects(project);
        ",
    )?;

    Ok(())
}
