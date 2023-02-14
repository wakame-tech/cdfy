use crate::card::{Card, Suit};
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Deck(pub Vec<Card>);

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            write!(f, "(empty)")
        } else {
            write!(
                f,
                "[{}]",
                self.0
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    }
}

impl Deck {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn with_jokers(jokers: usize) -> Self {
        let mut rng = thread_rng();
        let mut deck = vec![];
        for suit in Suit::suits().iter() {
            for number in 1u8..=13 {
                deck.push(Card::Number(suit.clone(), number))
            }
        }
        for _ in 0..jokers {
            deck.push(Card::Joker)
        }
        deck.shuffle(&mut rng);
        Self(deck)
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.0.shuffle(&mut rng);
    }

    pub fn dejoker(cards: &Vec<Card>) -> Vec<Card> {
        let mut numbers: Vec<_> = cards
            .iter()
            .filter(|c| c.number().is_some())
            .cloned()
            .collect();
        let jokers: Vec<_> = cards
            .iter()
            .filter(|c| c.number().is_none())
            .cloned()
            .collect();
        if numbers.is_empty() {
            cards.clone()
        } else {
            let n = numbers.first().unwrap().number().unwrap();
            numbers.extend(jokers.iter().map(|_c| Card::Number(Suit::UnSuited, n)));
            numbers
        }
    }
}

impl std::ops::SubAssign for Deck {
    fn sub_assign(&mut self, rhs: Self) {
        let indices = self
            .0
            .iter()
            .map(|c| rhs.0.iter().position(|h| h == c))
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        *self = Deck(
            self.0
                .iter()
                .enumerate()
                .filter(|(i, _)| !indices.contains(i))
                .map(|(_, c)| c.clone())
                .collect(),
        );
    }
}
