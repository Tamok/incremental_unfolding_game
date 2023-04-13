use crate::game_state::GameState;
use iced::{
    Application, Column, Command, Container, Element, Length, Row, Settings, Text, button, Button,
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
    
        let character = character_widget(&state.character);
    
        let quests = state
            .quests
            .iter()
            .zip(&mut state.quest_button_states)
            .map(|(quest, button_state)| quest_widget(quest, button_state))
            .collect::<Column<_>>();
    
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

fn quest_widget(quest: &Quest, button_state: &mut button::State) -> Button<Message> {
    let label = format!("{} - Cost: {}", quest.name, quest.cost);
    
    Button::new(button_state, Text::new(label).size(16))
        .on_press(Message::QuestSelected(quest.id))
}