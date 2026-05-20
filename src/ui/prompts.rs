use crate::errors::CliError;
use dialoguer::Input;

pub fn prompt_for_message() -> Result<String, CliError> {
    let input = Input::<String>::new()
        .with_prompt("Work description")
        .allow_empty(true)
        .interact_text()?;

    Ok(input.trim().to_string())
}
