use all_roles_career_poker::{Game, Player};
use anyhow::Result;

pub mod all_roles_career_poker;
pub mod card;
pub mod deck;

fn main() -> Result<()> {
    let mut players: Vec<Player> = Player::distribute(3, 2);
    let mut game = Game::new();
    game.run(&mut players)?;
    Ok(())
}
