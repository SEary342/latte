use crate::{cli::SearchArgs, commands::search, errors::CliError};

pub fn handle() -> Result<(), CliError> {
    search::handle(SearchArgs::default())
}
