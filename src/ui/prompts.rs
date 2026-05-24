use crate::{errors::CliError, ui::fuzzy::ListCompleter};

use dialoguer::Input;
use inquire::Text;

pub fn prompt_for_message() -> Result<String, CliError> {
    let input = Input::<String>::new()
        .with_prompt("Work description")
        .allow_empty(true)
        .interact_text()?;

    Ok(input.trim().to_string())
}

pub fn prompt_for_task_description(task_key: &str) -> Result<String, CliError> {
    let input = Input::<String>::new()
        .with_prompt(format!(
            "New task key '{}' detected.\nEnter task description",
            task_key
        ))
        .allow_empty(true)
        .interact_text()?;

    Ok(input.trim().to_string())
}

pub fn prompt_for_multi_value(
    prompt: &str,
    options: Vec<String>,
    _allow_empty: bool,
) -> Result<Vec<String>, CliError> {
    let completer = ListCompleter { options };

    let input = Text::new(prompt).with_autocomplete(completer).prompt()?;

    // Split the final string into a Vec
    let result = input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(result)
}

pub fn prompt_for_value(
    prompt: &str,
    options: Vec<String>,
    allow_empty: bool,
) -> Result<String, CliError> {
    let mut text_prompt = Text::new(prompt);

    // Apply autocomplete ONLY if we have options
    if !options.is_empty() {
        text_prompt = text_prompt.with_autocomplete(ListCompleter { options });
    }

    let input = text_prompt.prompt()?;

    // Unified Validation
    if !allow_empty && input.trim().is_empty() {
        return Err(CliError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Input cannot be empty",
        )));
    }

    Ok(input)
}

pub fn prompt_for_time(prompt: &str) -> Result<Option<u32>, CliError> {
    let input = Input::<String>::new()
        .with_prompt(prompt)
        .allow_empty(true)
        .validate_with(validate_time_input)
        .interact_text()?;

    let input = input.trim();

    if input.is_empty() {
        return Ok(None);
    }

    let value: u32 = input.parse().unwrap();

    Ok(Some(value))
}

fn validate_time_input(input: &String) -> Result<(), String> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Ok(());
    }

    let value = match trimmed.parse::<u32>() {
        Ok(v) => v,
        Err(_) => return Err(format!("Invalid input '{}'. Must be a number.", trimmed)),
    };

    let hours = value / 100;
    let minutes = value % 100;

    if hours > 23 || minutes > 59 {
        return Err(format!("Invalid time '{}'. Expected HHMM format.", value));
    }

    Ok(())
}
