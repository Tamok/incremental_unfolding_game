/*
This file will act as an entry point for the application.
It will handle creating the window and running the game loop.
*/
mod ui;
mod quest;
mod character;
mod ability;
mod game_state;
mod load_data;
mod resource;

use crate::ui::IncrementalGame;
use iced::{Application, Settings};

pub fn main() -> iced::Result {
    IncrementalGame::run(Settings::default())
}