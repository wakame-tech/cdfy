use crate::{card::Card, deck::Deck, state::CareerPokerState};
use std::{cmp::Ordering, collections::HashSet};

/// `Action` is a minimal unit of operationg `Game`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// カードを山札に出す
    ServeRiver((usize, Deck)),
    /// 手番を別の人に渡す
    Pass(usize),
}

pub fn career_poker_card_ord(a: &Card, b: &Card) -> Ordering {
    match (a, b) {
        (Card::Joker, Card::Joker) => Ordering::Equal,
        (Card::Joker, _) => Ordering::Greater,
        (_, Card::Joker) => Ordering::Less,
        (Card::Number(_, i), Card::Number(_, j)) => ((i + 10) % 13).cmp(&((j + 10) % 13)),
    }
}

fn ord(state: &CareerPokerState, lhs: &Deck, rhs: &Deck) -> Ordering {
    let (mut l, mut r) = (lhs.clone(), rhs.clone());
    l.0.sort_by(|a, b| career_poker_card_ord(a, b));
    r.0.sort_by(|a, b| career_poker_card_ord(a, b));
    let orderings: Vec<_> =
        l.0.iter()
            .zip(r.0.iter())
            .map(|(a, b)| career_poker_card_ord(a, b))
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

pub fn servable(state: &CareerPokerState, serves: &Deck) -> bool {
    let Some(lasts) = state.river.last() else {
        return true;
    };
    let Some(river_size) = state.river_size else {
        return true;
    };
    serves.0.len() == river_size && ord(state, lasts, serves).is_lt()
}

pub fn is_same_number(cards: &Vec<Card>) -> bool {
    let numbers: HashSet<_> = cards.iter().filter_map(|c| c.number()).collect();
    // if only jokers, len == 0
    numbers.len() <= 1
}
