use clap::{Args as ClapArgs, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    after_help = concat!(
        "Repository: ",
        env!("CARGO_PKG_REPOSITORY")
    )
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a new log entry
    Add(AddArgs),

    /// List all logs for the current day
    List,

    /// Search logs by tag, project, task key, or time range
    Search(SearchArgs),

    /// Edit an existing log entry by ID
    Edit(EditArgs),

    /// Generate a weekly summary report
    Summary(SummaryArgs),

    /// Show the path to the database file
    Path,

    /// Auto-clean the database
    Cleanup(CleanupArgs),
}

#[derive(Debug, ClapArgs)]
pub struct AddArgs {
    /// Task Key (e.g., JIRA-123)
    pub task_key: String,

    /// Description of the work done
    #[arg(short, long)]
    pub message: Option<String>,

    /// Tags associated with the entry
    #[arg(short, long, value_delimiter = ',')]
    pub tags: Vec<String>,

    /// Projects associated with the entry
    #[arg(short, long, value_delimiter = ',')]
    pub projects: Vec<String>,

    /// Type of work activity associated with the entry
    #[arg(short, long, value_delimiter = ',')]
    pub activity_types: Vec<String>,

    /// Start time (e.g., 1300)
    #[arg(short, long)]
    pub start: Option<u32>,

    /// End time (e.g., 1600)
    #[arg(short, long)]
    pub end: Option<u32>,

    /// Skip optional prompts
    #[arg(short, long)]
    pub quick: bool,
}

#[derive(Debug, ClapArgs)]
pub struct SearchArgs {
    #[arg(short, long)]
    pub tag: Option<String>,

    #[arg(short, long)]
    pub project: Option<String>,

    #[arg(short, long)]
    pub key: Option<String>,
}

#[derive(Debug, ClapArgs)]
pub struct EditArgs {
    /// The unique ID of the log entry
    pub id: usize,
}

#[derive(Debug, ClapArgs)]
pub struct SummaryArgs {
    /// Start date for range (YYYY-MM-DD)
    #[arg(long)]
    pub start_date: Option<String>,

    /// End date for range (YYYY-MM-DD)
    #[arg(long)]
    pub end_date: Option<String>,
}

#[derive(Debug, ClapArgs)]
pub struct CleanupArgs {
    #[arg(long)]
    pub dry_run: bool,
}
