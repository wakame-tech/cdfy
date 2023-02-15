use cdfy_sdk::State;
// use deck::Deck;
use crate::{
    deck::Deck,
    game::{career_poker_card_ord, servable},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CareerPokerState {
    pub current: Option<String>,
    pub actions: Vec<String>,
    pub players: Vec<String>,
    pub excluded: Deck,
    pub trushes: Deck,
    pub river: Vec<Deck>,
    pub last_served_player_id: Option<String>,
    pub fields: HashMap<String, Deck>,
    pub river_size: Option<usize>,
}

impl CareerPokerState {
    pub fn new() -> Self {
        Self {
            actions: vec![
                "serve".to_string(),
                "pass".to_string(),
                "distribute".to_string(),
            ],
            excluded: Deck(vec![]),
            trushes: Deck(vec![]),
            river: vec![],
            players: vec![],
            last_served_player_id: None,
            current: None,
            fields: HashMap::new(),
            river_size: None,
        }
    }

    pub fn into_state(&self) -> State {
        State {
            data: serde_json::to_string(&self).unwrap(),
        }
    }

    fn flush(&mut self) {
        self.trushes.0.extend(
            self.river
                .iter()
                .flat_map(|d| d.0.clone())
                .collect::<Vec<_>>(),
        );
        self.river.clear();
    }

    fn exclude(&mut self) {
        self.excluded.0.extend(
            self.river
                .iter()
                .flat_map(|d| d.0.clone())
                .collect::<Vec<_>>(),
        );
        self.river.clear();
    }

    pub fn join(&mut self, player_id: String) {
        if !self.players.contains(&player_id) {
            self.players.push(player_id.clone());
            self.fields.insert(player_id, Deck(vec![]));
        }
    }

    pub fn leave(&mut self, player_id: String) {
        if let Some(i) = self.players.iter().position(|id| id == &player_id) {
            self.players.remove(i);
        }
    }

    pub fn distribute(&mut self) {
        let deck = Deck::with_jokers(2);
        let players = self.players.iter().collect::<Vec<_>>();
        for player_id in players.iter() {
            if let Some(deck) = self.fields.get_mut(*player_id) {
                deck.0.clear();
            }
        }
        for (i, card) in deck.0.into_iter().enumerate() {
            let player_id = players[i % players.len()];
            if let Some(deck) = self.fields.get_mut(player_id) {
                deck.0.push(card);
            }
        }
        for player_id in players.iter() {
            if let Some(deck) = self.fields.get_mut(*player_id) {
                deck.0.sort_by(|a, b| career_poker_card_ord(a, b))
            }
        }
    }

    pub fn serve(&mut self, player_id: String, serves: Deck) {
        if !servable(&self, &serves) {
            return;
        }
        let Some(deck) = self.fields.get_mut(&player_id) else {
            return;
        };
        self.river.push(serves.clone());
        self.river_size = Some(serves.0.len());
        *deck -= serves;
    }
}
