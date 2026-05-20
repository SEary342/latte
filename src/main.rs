use clap::Parser;

mod cli;
mod commands;
mod errors;
mod models;
mod storage;
mod ui;

use crate::{
    cli::{Args, Commands},
    errors::CliError,
};

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<(), CliError> {
    match args.command {
        Commands::Add {
            task_key,
            message,
            tags,
            projects,
            start,
            end,
        } => commands::add::handle(task_key, message, tags, projects, start, end)?,

        Commands::List => {
            commands::list::handle()?;
        }

        Commands::Search { tag, project, key } => {
            // TODO
        }

        Commands::Edit { id } => {
            // TODO
        }

        Commands::Summary {
            start_date,
            end_date,
        } => {
            // TODO
        }
    }

    Ok(())
}
