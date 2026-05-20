use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("IO error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not parse integer: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Dialog error: {0}")]
    Dialog(#[from] dialoguer::Error),

    #[error("Could not determine home/config directory")]
    MissingHomeDir,

    #[error("Invalid path: {path}")]
    InvalidPath { path: std::path::PathBuf },

    #[error("{0}")]
    Storage(String),
}
