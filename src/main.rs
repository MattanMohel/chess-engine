pub mod game;
pub mod piece;

use crate::game::Game;

fn main() -> std::io::Result<()> {
    let src = std::fs::read_to_string("res/board.txt")?;
    let mut game = Game::new(&src);

    game.play()
}