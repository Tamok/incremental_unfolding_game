#[derive(Debug, Clone)]
pub struct Quest {
    title: String,
    description: String,
    completed: bool,
}

impl Quest {
    pub fn new(title: &str, description: &str) -> Self {
        Quest {
            title: title.to_string(),
            description: description.to_string(),
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}