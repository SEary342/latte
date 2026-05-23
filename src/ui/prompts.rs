use crate::errors::CliError;

use dialoguer::Input;

use super::completion::VecCompletion;

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
            "New task key '{}' detected. Enter description",
            task_key
        ))
        .allow_empty(true)
        .interact_text()?;

    Ok(input.trim().to_string())
}

pub fn prompt_for_multi_value(prompt: &str, options: Vec<String>) -> Result<Vec<String>, CliError> {
    let completion = VecCompletion { items: options };

    let input = Input::<String>::new()
        .with_prompt(prompt)
        .allow_empty(true)
        .completion_with(&completion)
        .interact_text()?;

    Ok(parse_csv_input(&input))
}

fn parse_csv_input(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToOwned::to_owned)
        .collect()
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
