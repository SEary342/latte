use inquire::autocompletion::Autocomplete;

#[derive(Clone)]
pub struct ListCompleter {
    pub options: Vec<String>,
}

impl Autocomplete for ListCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        // Find the token the user is currently typing (the part after the last comma)
        let last_token = input.split(',').last().unwrap_or("").trim();

        let matches = self
            .options
            .iter()
            .filter(|o| o.to_lowercase().contains(&last_token.to_lowercase()))
            .cloned()
            .collect();
        Ok(matches)
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Option<String>, inquire::CustomUserError> {
        if let Some(suggestion) = highlighted_suggestion {
            let mut parts: Vec<&str> = input.split(',').collect();
            if let Some(last) = parts.last_mut() {
                *last = &suggestion;
            }
            return Ok(Some(parts.join(", ")));
        }
        Ok(None)
    }
}
