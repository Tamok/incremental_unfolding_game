/*
This file will contain the Quest struct and associated logic.
*/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quest {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub reward: u32,
}

impl Quest {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            is_active: false,
            is_complete: false,
            id: 1,
            cost: 10
        }
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn complete(&mut self) {
        self.is_active = false;
        self.is_complete = true;
    }

    pub fn example_data() -> Vec<Quest> {
        vec![
            Quest {
                id: 1,
                name: String::from("First Quest"),
                description: String::from("This is the first quest."),
                cost: 10,
            },
            Quest {
                id: 2,
                name: String::from("Second Quest"),
                description: String::from("This is the second quest."),
                cost: 20,
            },
        ]
    }
}