#[derive(Debug, Clone)]
pub struct Quest {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub is_active: bool,
    pub is_complete: bool,
}

impl Quest {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            is_active: false,
            is_complete: false,
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn complete(&mut self) {
        self.is_active = false;
        self.is_complete = true;
    }
}