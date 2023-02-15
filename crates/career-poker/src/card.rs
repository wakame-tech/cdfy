use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Suit {
    #[serde(rename = "?")]
    UnSuited,
    #[serde(rename = "s")]
    Spade,
    #[serde(rename = "d")]
    Diamond,
    #[serde(rename = "h")]
    Heart,
    #[serde(rename = "c")]
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
            Suit::Spade => write!(f, "s"),
            Suit::Diamond => write!(f, "d"),
            Suit::Heart => write!(f, "h"),
            Suit::Clover => write!(f, "c"),
            Suit::UnSuited => write!(f, "*"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

impl From<&str> for Card {
    // A-K,shdc
    fn from(e: &str) -> Self {
        if e == "joker" {
            return Card::Joker;
        }
        let chars = e.chars().collect::<Vec<char>>();
        let n: u8 = match chars[0] {
            'A' => 1,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            n => n.to_string().parse().unwrap(),
        };
        let s = match chars[1] {
            'h' => Suit::Heart,
            'd' => Suit::Diamond,
            'c' => Suit::Clover,
            's' => Suit::Spade,
            _ => panic!(),
        };
        Card::Number(s, n)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn s(n: u8) -> String {
            match n {
                1 => "A".to_string(),
                10 => "T".to_string(),
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                _ => n.to_string(),
            }
        }
        match self {
            Card::Number(suit, number) => write!(f, "{}{}", s(*number), suit),
            Card::Joker => write!(f, "joker"),
        }
    }
}
