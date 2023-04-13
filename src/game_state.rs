use crate::character::Character;
use crate::quest::Quest;
use crate::ability::Ability;

pub struct GameState {
    pub character: Character,
    pub quests: Vec<Quest>,
    pub abilities: Vec<Ability>,
    pub resources: u32,
    pub quest_button_states: Vec<button::State>,
}

impl GameState {
    pub fn new() -> Self {
        let quest_button_states = vec![button::State::new(); quests.len()];
        GameState {
            character: Character::new("Hero"),
            quests: vec![],
            abilities: Vec::new(),
            resources: 0,
            quest_button_states,
        }
    }

    pub fn add_quest(&mut self, quest: Quest) {
        self.quests.push(quest);
    }

    pub fn execute_ability(&mut self, index: usize) {
        self.character.execute_ability(index);
    }
}