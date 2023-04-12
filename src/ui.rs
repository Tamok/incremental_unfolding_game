use iced::{Column, Element, Sandbox, Settings, Text};
use crate::Character;

pub fn run(character: &Character) -> iced::Result {
    Ui::run(Settings::default(), character)
}

pub struct Ui<'a> {
    character: &'a Character,
}

impl Sandbox for Ui {
    type Message = ();

    fn new(character: &'a Character) -> Self {
        Self { character }
    }    

    fn title(&self) -> String {
        String::from("Incremental Game")
    }

    fn update(&mut self, _message: Self::Message) {
        // This will be updated later to handle user input
    }

    fn view(&mut self) -> Element<Self::Message> {
        let character_info = Column::new()
            .push(Text::new(format!("Name: {}", self.character.name)))
            .push(Text::new(format!("Level: {}", self.character.level)))
            .push(Text::new(format!("Experience: {:.1}", self.character.experience)))
            .spacing(10);
    
        let resources_info = self
            .character
            .resources
            .values()
            .map(|resource| Text::new(format!("{}: {:.1}", resource.name, resource.amount)))
            .fold(Column::new().spacing(10), |column, text| column.push(text));
    
        let content = Column::new()
            .push(Text::new("Incremental Game"))
            .push(character_info)
            .push(resources_info)
            .spacing(20);
    
        Column::new()
            .padding(20)
            .push(content)
            .into()
    }    
}