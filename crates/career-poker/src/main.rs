use anyhow::Result;
use game::Game;
pub mod effects;
pub mod game;

fn main() -> Result<()> {
    let mut game = Game::new(3);
    game.run()?;
    Ok(())
}
