use crate::card::{Card, Suit};

pub fn deck(jokers: usize) -> Vec<Card> {
    let mut deck = vec![];
    for suit in Suit::suits().iter() {
        for number in 1u8..=13 {
            deck.push(Card::Number(suit.clone(), number))
        }
    }
    for _ in 0..jokers {
        deck.push(Card::Joker)
    }
    deck
}
