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
    ui::{guard::TerminalGuard, header::print_logo, util::clear_screen},
};

fn main() -> Result<(), CliError> {
    let _guard = TerminalGuard::new();
    clear_screen()?;
    print_logo();
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        return Err(e);
    }
    Ok(())
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
