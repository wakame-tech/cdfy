use crate::{
    card::{Card, Suit},
    deck::Deck,
    state::CareerPokerState,
};
use std::{cmp::Ordering, collections::HashSet};

pub fn career_poker_card_ord(a: &Card, b: &Card) -> Ordering {
    let (a_num, b_num) = (a.number(), b.number());
    match (a_num, b_num) {
        (None, None) => Ordering::Less,
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (Some(i), Some(j)) => ((i + 10) % 13).cmp(&((j + 10) % 13)),
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
    let Some(river_size) = state.effect.river_size else {
        return true;
    };

    let mut ok = true;

    // check ordering
    let ordering = if state.effect.revoluted ^ state.effect.effect_11 {
        ord(state, lasts, serves).reverse()
    } else {
        ord(state, lasts, serves)
    };
    ok = ok && ordering.is_lt();
    // check river size
    if (!state.effect.effect_3 && !state.effect.effect_10) && number(&serves.0) == 9 {
        let sizes = match state.effect.river_size {
            Some(3) => vec![Some(1), Some(3)],
            Some(1) => vec![Some(1), Some(3)],
            n => vec![n],
        };
        ok = ok && sizes.contains(&state.effect.river_size);
    } else {
        ok = ok && serves.0.len() == river_size;
    }
    ok = ok && is_same_number(&serves.0);

    if !state.effect.effect_3 && state.effect.effect_12 {
        ok = ok && number(&lasts.0) + 1 == number(&serves.0) && match_suits(&lasts.0, &serves.0)
    }
    ok
}

fn is_same_number(cards: &Vec<Card>) -> bool {
    let numbers: HashSet<_> = cards.iter().filter_map(|c| c.number()).collect();
    // if only jokers, len == 0
    numbers.len() <= 1
}

/// returns cards number, if only jokers, returns 0
pub fn number(cards: &Vec<Card>) -> u8 {
    let numbers = cards
        .iter()
        .filter_map(|c| c.number())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    *numbers.first().unwrap_or(&0)
}

pub fn match_suits(lhs: &Vec<Card>, rhs: &Vec<Card>) -> bool {
    assert_eq!(lhs.len(), rhs.len());
    let l_suits = lhs
        .iter()
        .map(|c| c.suit())
        .filter(|s| s != &Suit::UnSuited)
        .collect::<HashSet<_>>();
    let r_suits = rhs
        .iter()
        .map(|c| c.suit())
        .filter(|s| s != &Suit::UnSuited)
        .collect::<HashSet<_>>();
    r_suits.is_superset(&l_suits)
}
