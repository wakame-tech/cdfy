use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Suit {
    Spade,
    Diamond,
    Heart,
    Clover,
}

impl Suit {
    pub fn suits() -> Vec<Suit> {
        vec![Suit::Spade, Suit::Diamond, Suit::Heart, Suit::Clover]
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Spade => write!(f, "♤"),
            Suit::Diamond => write!(f, "♢"),
            Suit::Heart => write!(f, "♡"),
            Suit::Clover => write!(f, "♧"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Card {
    Number(Suit, u8),
    Joker,
}

impl Card {
    pub fn suit(&self) -> Option<Suit> {
        match self {
            Card::Number(s, _) => Some(s.clone()),
            Card::Joker => None,
        }
    }

    pub fn number(&self) -> Option<u8> {
        match self {
            Card::Joker => None,
            Card::Number(_, n) => Some(*n),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn s(n: u8) -> String {
            match n {
                1 => "A".to_string(),
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                _ => n.to_string(),
            }
        }
        match self {
            Card::Number(suit, number) => write!(f, "{}  {}", suit, s(*number)),
            Card::Joker => write!(f, "joker"),
        }
    }
}
