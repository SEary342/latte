use crossterm::{
    ExecutableCommand,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

use crate::errors::CliError;

pub fn clear_screen() -> Result<(), CliError> {
    stdout().execute(Clear(ClearType::All))?;
    //stdout().execute(crossterm::cursor::MoveTo(0, 0))?;
    Ok(())
}
