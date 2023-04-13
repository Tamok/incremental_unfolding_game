use crate::ability::{Ability, create_ability_list};

#[derive(Default)]
pub struct Character {
    pub name: String,
    pub level: u32,
    pub experience: f64,
    pub resources: u32,
    pub abilities: Vec<Ability>,
}

impl Character {
    pub fn new(name: &str) -> Character {
        Character {
            name: name.to_string(),
            level: 1,
            experience: 0.0,
            resources: 0,
            abilities: create_ability_list(),
        }
    }

    pub fn execute_ability(&mut self, index: usize) {
        if index < self.abilities.len() {
            let cost = self.abilities[index].cost;
            let effect = self.abilities[index].effect;
    
            if self.resources >= cost {
                effect(self);
                self.resources -= cost;
            }
        }
    }    
}