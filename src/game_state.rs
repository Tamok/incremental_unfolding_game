/*
This file will contain the GameState struct, which will hold the game's state,
including the character, quests, abilities, and resources.
*/
use crate::{
    ability::Ability,
    character::Character,
    load_data::load_from_json,
    quest::Quest,
    resource::Resources,
};
use iced::button::State as ButtonState;

pub struct GameState {
    pub abilities: Vec<Ability>,
    pub characters: Vec<Character>,
    pub quests: Vec<Quest>,
    pub resources: Resources,
    pub quest_button_states: Vec<ButtonState>,
}

impl GameState {
    pub fn new() -> Self {
        let abilities = load_from_json::<Ability>("data/abilities.json").expect("Failed to load abilities");
        let quests = load_from_json::<Quest>("data/quests.json").expect("Failed to load quests");
        let resources = Resources::default();
        let characters = vec![
            Character::new("Alice", abilities.clone()),
            Character::new("Bob", abilities.clone()),
        ];

        let quest_button_states = vec![ButtonState::new(); quests.len()];

        GameState {
            abilities,
            characters,
            quests,
            resources,
            quest_button_states,
        }
    }
}