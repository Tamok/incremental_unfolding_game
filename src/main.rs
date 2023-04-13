mod ui;
mod quest;

use iced::Settings;
use ui::IncrementalGame;

fn main() -> iced::Result {
    let character = ui::Character::new("Player", 1, 0.0);
    IncrementalGame::new(Settings::with_flags(character))
}
