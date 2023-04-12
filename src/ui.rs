use iced::{Column, Element, Sandbox, Settings, Text};

pub fn run() -> iced::Result {
    Ui::run(Settings::default())
}

pub struct Ui;

impl Sandbox for Ui {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Incremental Game")
    }

    fn update(&mut self, _message: Self::Message) {
        // This will be updated later to handle user input
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content = Column::new()
            .push(Text::new("Incremental Game"))
            .spacing(20);

        Column::new()
            .padding(20)
            .push(content)
            .into()
    }
}