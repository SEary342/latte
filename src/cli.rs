use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    after_help = concat!("Repository: ", env!("CARGO_PKG_REPOSITORY"))
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a new log entry
    Add {
        /// Task Key (e.g., JIRA-123)
        task_key: String,

        /// Description of the work done
        #[arg(short, long)]
        message: Option<String>,

        /// Tags associated with the entry
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,

        /// Projects associated with the entry
        #[arg(short, long, value_delimiter = ',')]
        projects: Vec<String>,

        /// Start time (e.g., 1300)
        #[arg(short, long)]
        start: Option<u32>,

        /// End time (e.g., 1600)
        #[arg(short, long)]
        end: Option<u32>,
    },

    /// List all logs for the current day
    List,

    /// Search logs by tag, project, task key, or time range
    Search {
        #[arg(short, long)]
        tag: Option<String>,

        #[arg(short, long)]
        project: Option<String>,

        #[arg(short, long)]
        key: Option<String>,
    },

    /// Edit an existing log entry by ID
    Edit {
        /// The unique ID of the log entry
        id: usize,
    },

    /// Generate a weekly summary report
    Summary {
        /// Start date for range (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,

        /// End date for range (YYYY-MM-DD)
        #[arg(long)]
        end_date: Option<String>,
    },
}
