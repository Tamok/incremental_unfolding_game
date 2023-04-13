use crate::ability::{Ability, create_ability_list};

pub struct Character {
    pub name: String,
    pub level: u32,
    pub resources: u32,
    pub abilities: Vec<Ability>,
}

impl Character {
    pub fn execute_ability(&mut self, index: usize) {
        if index < self.abilities.len() {
            let ability = &self.abilities[index];
            if self.resources >= ability.cost {
                (ability.effect)(self);
                self.resources -= ability.cost;
            }
        }
    }
}

pub fn create_character(name: &str) -> Character {
    Character {
        name: name.to_string(),
        level: 1,
        resources: 0,
        abilities: create_ability_list(),
    }
}