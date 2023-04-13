use crate::game_state::GameState;
use crate::character::Character;
use iced::{
    Application, Column, Command, Container, Element, Text, button, Button, Length,
};

pub struct IncrementalGame {
    game_state: GameState,
    resource_button_state: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ResourceButtonPressed,
}

impl IncrementalGame {
    pub fn new() -> Self {
        IncrementalGame {
            game_state: GameState::new(),
            resource_button_state: button::State::new(),
        }
    }
}

impl Application for IncrementalGame {
    type Executor = iced::executor::Default;
    type Flags = Character;
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (IncrementalGame::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Incremental Unfolding Game")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .push(Text::new("Resources:"))
            .push(
                Button::new(&mut self.resource_button_state, Text::new("Gain Resource"))
                    .padding(10)
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