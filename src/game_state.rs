use crate::character::Character;
use crate::quest::Quest;
use crate::ability::Ability;

pub struct GameState {
    pub character: Character,
    pub quests: Vec<Quest>,
    pub abilities: Vec<Ability>,
    pub resources: u32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            character: Character::new("Hero"),
            quests: vec![],
            abilities: Vec::new(),
            resources: 0,
        }
    }

    pub fn add_quest(&mut self, quest: Quest) {
        self.quests.push(quest);
    }

    pub fn execute_ability(&mut self, index: usize) {
        self.character.execute_ability(index);
    }
}