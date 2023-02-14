use card::Card;
use cdfy_sdk::{fp_export_impl, State};
use deck::Deck;
use game::Game;

pub mod card;
pub mod deck;
pub mod game;

#[fp_export_impl(cdfy_sdk)]
pub fn on_serve(state: State, player_id: String, cards: Vec<String>) -> State {
    let mut game: Game = serde_json::from_str(&state.state).unwrap();
    let cards: Vec<Card> = cards.iter().map(|c| Card::from(c)).collect();
    game.river.push(Deck(cards));
    State {
        state: serde_json::to_string(&game).unwrap(),
    }
}
