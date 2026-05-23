use clap::Parser;

mod cli;
mod commands;
mod config;
mod db;
mod errors;
mod models;
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
        Commands::Add(args) => commands::add::handle(args)?,

        Commands::List => {
            commands::list::handle()?;
        }

        Commands::Search(_args) => {
            // TODO
        }

        Commands::Edit(_args) => {
            // TODO
        }

        Commands::Summary(_args) => {
            // TODO
        }

        Commands::Path => {
            println!("{}", config::show_paths()?);
        }

        Commands::Cleanup(args) => commands::cleanup::handle(args)?,
    }

    Ok(())
}
