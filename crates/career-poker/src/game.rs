use crate::{card::Card, deck::Deck};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashSet};

/// `Action` is a minimal unit of operationg `Game`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// カードを山札に出す
    ServeRiver((usize, Deck)),
    /// 手番を別の人に渡す
    Pass(usize),
}

pub fn career_poker_card_ord(_game: &Game, a: &Card, b: &Card) -> Ordering {
    match (a, b) {
        (Card::Joker, Card::Joker) => Ordering::Equal,
        (Card::Joker, _) => Ordering::Greater,
        (_, Card::Joker) => Ordering::Less,
        (Card::Number(_, i), Card::Number(_, j)) => ((i + 10) % 13).cmp(&((j + 10) % 13)),
    }
}

fn ord(game: &Game, lhs: &Deck, rhs: &Deck) -> Ordering {
    let (mut l, mut r) = (lhs.clone(), rhs.clone());
    l.0.sort_by(|a, b| career_poker_card_ord(game, a, b));
    r.0.sort_by(|a, b| career_poker_card_ord(game, a, b));
    let orderings: Vec<_> =
        l.0.iter()
            .zip(r.0.iter())
            .map(|(a, b)| career_poker_card_ord(game, a, b))
            .collect::<HashSet<_>>()
            .iter()
            .cloned()
            .collect();
    if orderings.len() == 1 {
        *orderings.first().unwrap()
    } else {
        Ordering::Equal
    }
}

fn _servable(game: &Game, deck: &Deck) -> bool {
    if let Some(lasts) = game.river.last() {
        !(lasts.0.len() != deck.0.len()
            || !is_same_number(&deck.0)
            || ord(game, deck, lasts).is_lt())
    } else {
        true
    }
}

pub fn is_same_number(cards: &Vec<Card>) -> bool {
    let numbers: HashSet<_> = cards.iter().filter_map(|c| c.number()).collect();
    // if only jokers, len == 0
    numbers.len() <= 1
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub river: Vec<Deck>,
}

impl Game {
    pub fn new() -> Self {
        Self { river: vec![] }
    }

    // fn flush(&mut self) {
    //     self.trushes.0.extend(
    //         self.river
    //             .iter()
    //             .flat_map(|d| d.0.clone())
    //             .collect::<Vec<_>>(),
    //     );
    //     self.river.clear();
    // }

    // fn exclude(&mut self) {
    //     self.excluded.0.extend(
    //         self.river
    //             .iter()
    //             .flat_map(|d| d.0.clone())
    //             .collect::<Vec<_>>(),
    //     );
    //     self.river.clear();
    // }
}
