/*
This file will contain the Character struct and associated logic.
*/
use crate::ability::{Ability, create_ability_list};

use crate::ability::Ability;

#[derive(Default)]
pub struct Character {
    pub name: String,
    pub level: u32,
    pub experience: f64,
    pub resources: u32,
    pub abilities: Vec<Ability>,
}

impl Character {
    pub fn new(name: &str, abilities: Vec<Ability>) -> Character {
        Character {
            name: name.to_string(),
            level: 1,
            experience: 0.0,
            resources: 0,
            abilities,
        }
    }

    pub fn execute_ability(&mut self, ability: &Ability) {
        let cost = ability.cost;

        if self.resources >= cost {
            ability.gather_effect(self);
            self.resources -= cost;
        }
    }
}