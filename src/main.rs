pub mod game;
pub mod piece;
pub mod moving;

use crate::game::Game;

fn main() -> std::io::Result<()> {
    println!("\u{2654}");
    
    let src = std::fs::read_to_string("src/res/board.txt")?;
    let mut game = Game::new(&src);

    game.play()
}