use crate::card::{Card, Suit};
use anyhow::anyhow;
use anyhow::Result;

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

pub fn remove_cards(deck: &mut Vec<Card>, cards: &Vec<Card>) -> Result<()> {
    let indices = cards
        .iter()
        .map(|c| deck.iter().position(|h| h == c))
        .collect::<Option<Vec<_>>>();
    if let Some(indices) = indices {
        *deck = deck
            .iter()
            .enumerate()
            .filter(|(i, _)| !indices.contains(i))
            .map(|(_, c)| c.clone())
            .collect();
        Ok(())
    } else {
        Err(anyhow!("cannot transfer"))
    }
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
        numbers.extend(jokers.iter().map(|c| Card::Number(Suit::UnSuited, n)));
        numbers
    }
}
