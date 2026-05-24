use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("IO error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not parse integer: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Could not parse timestamp: {0}")]
    ChronoParse(#[from] chrono::ParseError),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Dialog error: {0}")]
    Dialog(#[from] dialoguer::Error),

    #[error("Database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("UUID parsing error: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Could not determine home/config directory")]
    MissingHomeDir,

    #[error("Prompt error: {0}")]
    Inquire(#[from] inquire::InquireError),
}
