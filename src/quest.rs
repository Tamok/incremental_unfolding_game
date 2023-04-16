/*
This file will contain the Quest struct and associated logic.
*/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quest {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub cost: u32,
}

impl Quest {
    pub fn new(name: &str, description: &str, id: u32, reward: u32) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            id,
            reward,
        }
    }
}