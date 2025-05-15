use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::riengine::{
    gfx::gfx::{clear_screen, draw_screen},
    player::Player,
    scene::Scene,
};

use super::input::handle_input;

#[derive(PartialEq)]
pub enum GameState {
    Running,
    Paused,
    Stopped,
    Exiting,
}

pub struct Game {
    pub id: u32,
    pub scene: Scene,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let game = Game {
            id: 0,
            scene: Scene::new(),
            state: GameState::Running,
        };
        game
    }
}

pub fn game_loop(game: &mut Game) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    while game.state != GameState::Exiting {
        clear_screen();
        let _ = handle_input(game);
        draw_screen(game);
    }
    disable_raw_mode()?;
    Ok(())
}
