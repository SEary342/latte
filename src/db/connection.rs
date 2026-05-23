use crate::{config::database_path, errors::CliError};

use rusqlite::Connection;
use std::fs;

use super::migrations::run_migrations;

pub fn get_connection() -> Result<Connection, CliError> {
    let path = database_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(path)?;

    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    conn.pragma_update(None, "journal_mode", "WAL")?;

    conn.pragma_update(None, "synchronous", "NORMAL")?;

    run_migrations(&conn)?;

    Ok(conn)
}
