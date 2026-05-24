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

    /// List stored information
    List(ListArgs),

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
    #[arg(index = 1)]
    pub task_key: Option<String>,

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

#[derive(Debug, ClapArgs, Default)]
pub struct SearchArgs {
    #[arg(short, long)]
    pub tag: Option<String>,

    #[arg(short, long)]
    pub project: Option<String>,

    #[arg(short, long)]
    pub key: Option<String>,

    #[arg(short, long)]
    pub activity: Option<String>,

    /// Start date (HHDD)
    #[arg(long)]
    pub start_time: Option<String>,

    /// End time (HHDD)
    #[arg(long)]
    pub end_time: Option<String>,

    /// Start date of logs
    #[arg(long)]
    pub start_date: Option<String>,

    /// End date of logs
    #[arg(long)]
    pub end_date: Option<String>,

    /// Limit logs to today
    #[arg(short('d'), long)]
    pub today: bool,
}

#[derive(Debug, ClapArgs)]
pub struct EditArgs {
    /// Short UUID prefix (first 8 chars) of the log entry to edit.
    /// Leave blank to use --tag / --project / --activity for global entity edits.
    #[arg(index = 1)]
    pub id: Option<String>,

    // ── Log-entry field overrides (used together with `id`) ────────────────
    /// New message / work description
    #[arg(short, long)]
    pub message: Option<String>,

    /// Replace tags on the entry (comma-separated)
    #[arg(long = "tags", value_delimiter = ',')]
    pub entry_tags: Vec<String>,

    /// Replace projects on the entry (comma-separated)
    #[arg(long = "projects", value_delimiter = ',')]
    pub entry_projects: Vec<String>,

    /// Replace activity types on the entry (comma-separated)
    #[arg(long = "activities", value_delimiter = ',')]
    pub entry_activities: Vec<String>,

    /// New start time (HHMM)
    #[arg(short, long)]
    pub start: Option<u32>,

    /// New end time (HHMM)
    #[arg(short, long)]
    pub end: Option<u32>,

    // ── Global entity rename flows (mutually exclusive with `id`) ──────────
    /// Rename a global tag (prompts for new name)
    #[arg(long, conflicts_with = "id")]
    pub tag: Option<String>,

    /// Rename a global project (prompts for new name)
    #[arg(long, conflicts_with = "id")]
    pub project: Option<String>,

    /// Rename a global activity type (prompts for new name)
    #[arg(long, conflicts_with = "id")]
    pub activity: Option<String>,

    /// Edit a task description (prompts for new description)
    #[arg(long, conflicts_with = "id")]
    pub task: Option<String>,
}

#[derive(Debug, ClapArgs)]
#[group(id = "modes", multiple = false)]
pub struct ListArgs {
    /// Display all tasks
    #[arg(long)]
    pub task: bool,

    /// Display all tags
    #[arg(short, long)]
    pub tag: bool,

    /// Display all projects
    #[arg(short, long)]
    pub project: bool,

    /// Display all activities
    #[arg(short, long)]
    pub activity: bool,

    /// Limit logs to today
    #[arg(short('d'), long)]
    pub today: bool,
}

#[derive(Debug, ClapArgs)]
pub struct SummaryArgs {
    /// Start date for range (YYYY-MM-DD)
    #[arg(long)]
    pub start_date: Option<String>,

    /// End date for range (YYYY-MM-DD)
    #[arg(long)]
    pub end_date: Option<String>,

    /// JSON Output
    #[arg(short, long)]
    pub json: bool,
}

#[derive(Debug, ClapArgs)]
pub struct CleanupArgs {
    #[arg(long)]
    pub dry_run: bool,
}
