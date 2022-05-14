pub mod game;
pub mod piece;

use crate::game::Game;

fn main() -> std::io::Result<()> {
    let src = std::fs::read_to_string("C:/repo/Rust/chess-engine/src/res/board.txt")?;
    let mut game = Game::new(&src);

    println!("{}", game);

    Ok(())
}