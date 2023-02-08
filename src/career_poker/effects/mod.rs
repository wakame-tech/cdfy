use super::game::{Action, Game};
use crate::{card::Card, deck::Deck};
use anyhow::{anyhow, Result};
use std::{cmp::Ordering, collections::HashSet, fmt::Debug};
pub mod cards;

/// `Effect` affects the game and player's action
pub trait Effect: Debug {
    fn on_push(&mut self, game: &mut Game, action: &mut Result<Action>);
    fn on_resolve(&mut self, game: &mut Game, action: &mut Result<Action>);
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

pub fn is_same_number(cards: &Vec<Card>) -> bool {
    let numbers: HashSet<_> = cards.iter().filter_map(|c| c.number()).collect();
    // if only jokers, len == 0
    numbers.len() <= 1
}

#[derive(Debug)]
pub struct DefaultCareerPokerRiverGuard;

impl Effect for DefaultCareerPokerRiverGuard {
    fn on_resolve(&mut self, game: &mut Game, action: &mut Result<Action>) {
        match action {
            Ok(Action::ServeRiver((_, deck))) => {
                if let Some(lasts) = game.river.last() {
                    if lasts.0.len() != deck.0.len()
                        || !is_same_number(&deck.0)
                        || ord(game, deck, lasts).is_lt()
                    {
                        *action = Err(anyhow!("deck {} < river {}", deck, lasts));
                    }
                }
            }
            _ => {}
        }
    }

    fn on_push(&mut self, _game: &mut Game, _action: &mut Result<Action>) {}
}
