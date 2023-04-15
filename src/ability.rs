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

impl Ability {
    pub fn gather_effect(&self, character: &mut Character) {
        character.resources += 1;
    }
}