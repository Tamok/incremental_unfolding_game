use crate::quest::Quest;
use iced::{
    Application, Column, Command, Container, Element, Settings, Text, button, Button, Length, Row,
};

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    CompleteQuest(usize),
}

#[derive(Debug, Clone)]
pub struct Character {
    name: String,
    level: u32,
    experience: f64,
}

impl Character {
    pub fn new(name: &str, level: u32, experience: f64) -> Self {
        Character {
            name: name.to_string(),
            level,
            experience,
        }
    }
}

pub struct IncrementalGame {
    character: Character,
    increment_button: button::State,
    quests: Vec<Quest>,
}

impl IncrementalGame {
    pub fn new(settings: Settings<Character>) -> iced::Result {
        IncrementalGame::run(settings)
    }
}

impl Application for IncrementalGame {
    type Executor = iced::executor::Default;
    type Flags = Character;
    type Message = Message;

    fn new(flags: Character) -> (Self, Command<Self::Message>) {
        let mut quests = Vec::new();
        quests.push(Quest::new(
            "First Quest",
            "Increment your experience to complete this quest.",
        ));

        (
            Self {
                character: flags,
                increment_button: button::State::new(),
                quests,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Incremental Game")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Increment => {
                self.character.experience += 10.0;
                if self.character.experience >= 10.0 {
                    self.quests[0].complete();
                }
            }
            Message::CompleteQuest(index) => {
                self.quests[index].complete();
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let increment_button = Button::new(
            &mut self.increment_button,
            Text::new("Increment Experience"),
        )
        .on_press(Message::Increment);

 
        let mut content = Column::new()
            .push(Text::new("Incremental Game"))
            .push(Text::new(format!("Name: {}", self.character.name)))
            .push(Text::new(format!("Level: {}", self.character.level)))
            .push(Text::new(format!(
                "Experience: {:.1}",
                self.character.experience
            )))
            .push(increment_button)
            .spacing(20);

        for (index, quest) in self.quests.iter().enumerate() {
            let quest_status = if quest.completed {
                "Completed"
            } else {
                "Not completed"
            };

            content = content
                .push(Text::new(format!("{} - {}", quest.title, quest.description)))
                .push(Text::new(format!("Status: {}", quest_status)))
                .spacing(20);
        }

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
        }
}