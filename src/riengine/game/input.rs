use super::game::{Game, GameState};
use crossterm::event::{read, Event, KeyCode, KeyEventKind};

pub fn handle_input(game: &mut Game) -> Result<(), Box<dyn std::error::Error>> {
    let event = read()?;

    match event {
        Event::Key(key) => {
            if key.kind == KeyEventKind::Press {
                if handle_game_input(game, key.code)? {
                    return Ok(());
                }
                if handle_menu_input(game, key.code)? {
                    return Ok(());
                }
                if handle_exit_input(game, key.code)? {
                    return Ok(());
                }
                return Ok(());
            } else {
                return Ok(());
            }
        }
        _ => {
            return Ok(());
        }
    }
}

fn handle_game_input(game: &mut Game, key: KeyCode) -> Result<bool, Box<dyn std::error::Error>> {
    let player = &mut game.scene.player;

    match key {
        KeyCode::Char('w') => {
            if player.y > 0 {
                player.move_up(&game.scene.tile_map)?;
                return Ok(true);
            }
            return Ok(false);
        }
        KeyCode::Char('s') => {
            if player.y < game.scene.tile_map.get_height() as i32 - 1 {
                player.move_down(&game.scene.tile_map)?;
                return Ok(true);
            }
            return Ok(false);
        }
        KeyCode::Char('a') => {
            if player.x > 0 {
                player.move_left(&game.scene.tile_map)?;
                return Ok(true);
            }
            return Ok(false);
        }
        KeyCode::Char('d') => {
            if player.x < game.scene.tile_map.get_width() as i32 - 1 {
                player.move_right(&game.scene.tile_map)?;
                return Ok(true);
            }
            return Ok(false);
        }
        _ => {
            return Ok(false);
        }
    }
}

fn handle_menu_input(_game: &mut Game, _key: KeyCode) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(false)
}

fn handle_exit_input(_game: &mut Game, key: KeyCode) -> Result<bool, Box<dyn std::error::Error>> {
    if key == KeyCode::Esc {
        _game.state = GameState::Exiting;
        return Ok(true);
    }
    Ok(false)
}
