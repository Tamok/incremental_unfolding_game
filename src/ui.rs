/*
This file will contain the User Interface (UI) logic of the game.
It will define the layout and structure of the UI elements and handle their interactions.
*/
use crate::game_state::GameState;
use crate::character::Character;
use crate::quest::Quest;

use iced::{
    Application, Column, Command, Container, Element, Length, Settings, Text, button, Button,
};

pub fn run() -> Result<(), iced::Error> {
    IncrementalGame::run(Settings::default())
}

pub struct IncrementalGame {
    state: GameState,
    resource_button_state: button::State,
}

impl IncrementalGame {
    fn gain_resource(&mut self) {
        self.state.resources += 1;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ResourceButtonPressed,
    QuestSelected(u32),
}

impl Application for IncrementalGame {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            IncrementalGame {
                state: GameState::new(),
                resource_button_state: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Incremental Unfolding Game")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ResourceButtonPressed => {
                self.gain_resource();
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let IncrementalGame {
            state,
            resource_button_state,
        } = self;
    
        let character = character_widget(&state.characters[0]);
    
        let quests = state
            .quests
            .iter()
            .zip(&mut state.quest_button_states)
            .map(|(quest, button_state)| quest_widget(quest, button_state))
            .fold(Column::new(), |column, button| column.push(button));            
    
        let content = Column::new()
            .spacing(20)
            .push(character)
            .push(quests)
            .push(
                Button::new(
                    resource_button_state,
                    Text::new("Gain resources").size(20),
                )
                .on_press(Message::ResourceButtonPressed),
            );
    
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}  

fn character_widget(character: &Character) -> Column<Message> {
    let name = Text::new(&character.name)
        .size(30)
        .color([1.0, 1.0, 1.0])
        .width(Length::Fill);
    
    let resources = Text::new(format!("Resources: {}", character.resources))
        .size(20)
        .color([1.0, 1.0, 1.0])
        .width(Length::Fill);
    
    Column::new()
        .spacing(10)
        .push(name)
        .push(resources)
}

fn quest_widget<'a>(quest: &'a Quest, button_state: &'a mut button::State) -> Button<'a, Message> {
    let label = format!("{} - Cost: {}", quest.name, quest.cost);
    
    Button::new(button_state, Text::new(label).size(16))
        .on_press(Message::QuestSelected(quest.id))
}