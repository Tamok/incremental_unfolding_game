/*
This file will contain the Ability struct and associated logic.
*/
use crate::character::Character;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ability {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub cost: u32,
}

pub fn create_ability_list() -> Vec<Ability> {
    vec![
        Ability {
            name: "Gather".to_string(),
            description: "Gather resources from the environment.".to_string(),
            cost: 1,
            effect: gather_effect,
        },
        // Add more abilities here as needed.
    ]
}

fn gather_effect(character: &mut Character) {
    character.resources += 1;
}
