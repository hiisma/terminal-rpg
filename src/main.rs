mod maps;
mod riengine;

use riengine::game::game::{game_loop, Game};

fn main() {
    let mut game: Game = Game::new();
    game.scene = maps::map02::map02();

    game_loop(&mut game).unwrap();
}
