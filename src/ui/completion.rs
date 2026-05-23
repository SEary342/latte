use dialoguer::Completion;

pub struct VecCompletion {
    pub items: Vec<String>,
}

impl Completion for VecCompletion {
    fn get(&self, input: &str) -> Option<String> {
        self.items
            .iter()
            .find(|item| item.starts_with(input))
            .cloned()
    }
}
