use crate::{errors::CliError, models::Config};

use directories::ProjectDirs;

use std::{
    fs,
    path::{Path, PathBuf},
};

const CONFIG_FILE: &str = "config.json";
const DB_FILE: &str = "latte.db";

fn app_name() -> &'static str {
    if cfg!(debug_assertions) {
        "latte-dev"
    } else {
        "latte"
    }
}

fn project_dirs() -> Result<ProjectDirs, CliError> {
    ProjectDirs::from("", "", app_name()).ok_or(CliError::MissingHomeDir)
}

fn ensure_dir(path: &Path) -> Result<(), CliError> {
    fs::create_dir_all(path)?;
    Ok(())
}

fn config_path() -> Result<PathBuf, CliError> {
    let dirs = project_dirs()?;

    ensure_dir(dirs.config_dir())?;

    Ok(dirs.config_dir().join(CONFIG_FILE))
}

pub fn default_database_path() -> Result<PathBuf, CliError> {
    let dirs = project_dirs()?;

    ensure_dir(dirs.data_dir())?;

    Ok(dirs.data_dir().join(DB_FILE))
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

    Ok(serde_json::from_str(&contents)?)
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

pub fn show_paths() -> Result<String, CliError> {
    let config_path = config_path()?;
    let db_path = database_path()?;

    Ok(format!(
        "Configuration File: {}\nDatabase File: {}",
        config_path.display(),
        db_path.display()
    ))
}
