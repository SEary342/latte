use crate::{
    errors::CliError,
    models::{Config, LogDatabase, LogEntry},
};
use std::{fs, path::PathBuf};

const APP_NAME: &str = "worklog";
const CONFIG_FILE: &str = "config.json";
const DB_FILE: &str = "logs.json";

fn config_dir() -> Result<PathBuf, CliError> {
    let mut path = dirs::config_dir().ok_or(CliError::MissingHomeDir)?;

    path.push(APP_NAME);

    fs::create_dir_all(&path)?;

    Ok(path)
}

fn config_path() -> Result<PathBuf, CliError> {
    let mut path = config_dir()?;
    path.push(CONFIG_FILE);
    Ok(path)
}

fn default_database_path() -> Result<PathBuf, CliError> {
    let mut path = dirs::data_local_dir().ok_or(CliError::MissingHomeDir)?;

    path.push(APP_NAME);

    fs::create_dir_all(&path)?;

    path.push(DB_FILE);

    Ok(path)
}

pub fn load_config() -> Result<Config, CliError> {
    let path = config_path()?;

    if !path.exists() {
        let config = Config {
            database_path: default_database_path()?,
        };

        save_config(&config)?;

        return Ok(config);
    }

    let contents = fs::read_to_string(path)?;

    let config: Config = serde_json::from_str(&contents)?;

    Ok(config)
}

pub fn save_config(config: &Config) -> Result<(), CliError> {
    let path = config_path()?;

    let json = serde_json::to_string_pretty(config)?;

    fs::write(path, json)?;

    Ok(())
}

pub fn database_path() -> Result<PathBuf, CliError> {
    Ok(load_config()?.database_path)
}

pub fn load_database() -> Result<LogDatabase, CliError> {
    let path = database_path()?;

    if !path.exists() {
        return Ok(LogDatabase::default());
    }

    let contents = fs::read_to_string(path)?;

    let db: LogDatabase = serde_json::from_str(&contents)?;

    Ok(db)
}

pub fn save_database(db: &LogDatabase) -> Result<(), CliError> {
    let path = database_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(db)?;

    fs::write(path, json)?;

    Ok(())
}

pub fn add_entry(entry: LogEntry) -> Result<(), CliError> {
    let mut db = load_database()?;

    db.entries.push(entry);

    save_database(&db)?;

    Ok(())
}
