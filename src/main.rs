use anyhow::Result;
use career_poker::game::Game;

pub mod card;
pub mod career_poker;
pub mod deck;

fn main() -> Result<()> {
    let mut game = Game::new(3);
    game.run()?;
    Ok(())
}
