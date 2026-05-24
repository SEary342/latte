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
    let args = Args::parse();

    let is_interactive = matches!(
        args.command,
        Commands::Add(_) | Commands::Edit(_) | Commands::Cleanup(_)
    );
    let _guard = TerminalGuard::new(is_interactive);
    clear_screen()?;
    print_logo(is_interactive);

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        return Err(e);
    }
    Ok(())
}

fn run(args: Args) -> Result<(), CliError> {
    match args.command {
        Commands::Add(args) => commands::add::handle(args)?,
        Commands::List(args) => commands::list::handle(args)?,
        Commands::Search(args) => commands::search::handle(args)?,
        Commands::Edit(args) => commands::edit::handle(args)?,
        Commands::Summary(args) => commands::summary::handle(args)?,
        Commands::Path => {
            println!("{}", config::show_paths()?);
        }
        Commands::Cleanup(args) => commands::cleanup::handle(args)?,
    }

    Ok(())
}
