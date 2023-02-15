use cdfy_sdk::State;
// use deck::Deck;
use crate::{
    card::Card,
    deck::Deck,
    effect::{effect_card, flush},
    game::{career_poker_card_ord, servable},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub river_size: Option<usize>,
    pub effect_3: bool,
    pub effect_10: bool,
    pub effect_11: bool,
    pub effect_12: bool,
    pub revoluted: bool,
}

impl Effect {
    pub fn new() -> Self {
        Self {
            river_size: None,
            effect_3: false,
            effect_10: false,
            effect_11: false,
            effect_12: false,
            revoluted: false,
        }
    }

    pub fn new_turn(effect: Effect) -> Self {
        Self {
            river_size: None,
            effect_3: false,
            effect_10: false,
            effect_11: false,
            effect_12: false,
            ..effect
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareerPokerState {
    pub current: Option<String>,
    pub players: Vec<String>,
    pub excluded: Deck,
    pub trushes: Deck,
    pub river: Vec<Deck>,
    pub last_served_player_id: Option<String>,
    pub fields: HashMap<String, Deck>,
    pub effect: Effect,
    pub prompt_4_player_id: Option<String>,
    pub prompt_7_player_id: Option<String>,
    pub prompt_13_player_id: Option<String>,
    pub prompt_1_player_ids: HashSet<String>,
}

impl CareerPokerState {
    pub fn new() -> Self {
        Self {
            excluded: Deck(vec![]),
            trushes: Deck(vec![]),
            river: vec![],
            players: vec![],
            last_served_player_id: None,
            current: None,
            fields: HashMap::new(),
            effect: Effect::new(),
            prompt_4_player_id: None,
            prompt_7_player_id: None,
            prompt_13_player_id: None,
            prompt_1_player_ids: HashSet::new(),
        }
    }

    pub fn reset(&mut self) {
        *self = Self {
            excluded: Deck(vec![]),
            trushes: Deck(vec![]),
            river: vec![],
            players: self.players.clone(),
            last_served_player_id: None,
            current: None,
            fields: HashMap::new(),
            effect: Effect::new(),
            prompt_4_player_id: None,
            prompt_7_player_id: None,
            prompt_13_player_id: None,
            prompt_1_player_ids: HashSet::new(),
        }
    }

    pub fn has_prompt(&self) -> bool {
        self.prompt_4_player_id.is_some()
            || self.prompt_7_player_id.is_some()
            || self.prompt_13_player_id.is_some()
            || !self.prompt_1_player_ids.is_empty()
    }

    pub fn into_state(&self) -> State {
        State {
            data: serde_json::to_string(&self).unwrap(),
        }
    }

    pub fn join(&mut self, player_id: String) {
        if !self.players.contains(&player_id) {
            self.players.push(player_id.clone());
        }
    }

    pub fn leave(&mut self, player_id: String) {
        if let Some(i) = self.players.iter().position(|id| id == &player_id) {
            self.players.remove(i);
        }
    }

    pub fn distribute(&mut self) {
        self.reset();
        let deck = Deck::with_jokers(2);
        let players = self.players.iter().collect::<Vec<_>>();
        for player_id in players.iter() {
            self.fields.insert(player_id.to_string(), Deck(vec![]));
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
        self.current = Some(self.players[0].clone())
    }

    pub fn get_relative_player(&self, player_id: &str, d: i32) -> Option<String> {
        let player_index = self.players.iter().position(|id| id == &player_id).unwrap();
        let mut delta: i32 = d;
        loop {
            let index =
                ((player_index as i32 + delta).rem_euclid(self.players.len() as i32)) as usize;
            if let Some(deck) = self.fields.get(&self.players[index]) {
                if !deck.0.is_empty() {
                    return Some(self.players[index].clone());
                }
            }
            if d as usize == self.players.len() {
                return None;
            }
            delta += d;
        }
    }

    pub fn next(&mut self, player_id: &str) {
        self.current = self.get_relative_player(&player_id, 1);
        if self.current == self.last_served_player_id {
            flush(self);
        }
        if self.current.is_none() {
            flush(self);
            self.current = self.last_served_player_id.clone();
        }
    }

    pub fn pass(&mut self, player_id: String) {
        self.next(&player_id);
    }

    pub fn select_trushes(&mut self, player_id: String, serves: Vec<Card>) {
        if self.river.last().unwrap().0.len() != serves.len() {
            return;
        }
        let Some(deck) = self.fields.get_mut(&player_id) else {
            return;
        };
        self.trushes -= Deck(serves.clone());
        deck.0.extend(serves);
        self.prompt_4_player_id = None;
        self.next(&player_id);
    }

    pub fn select_excluded(&mut self, player_id: String, serves: Vec<Card>) {
        if self.river.last().unwrap().0.len() != serves.len() {
            return;
        }
        let Some(deck) = self.fields.get_mut(&player_id) else {
            return;
        };
        self.excluded -= Deck(serves.clone());
        deck.0.extend(serves);
        self.prompt_13_player_id = None;
        self.next(&player_id);
    }

    pub fn select_passes(&mut self, player_id: String, serves: Vec<Card>) {
        if self.river.last().unwrap().0.len() != serves.len() {
            return;
        }
        let left_id = self.get_relative_player(&player_id, -1).unwrap();
        let Some(deck) = self.fields.get_mut(&player_id) else {
            return;
        };
        *deck -= Deck(serves.clone());
        let Some(left_deck) = self.fields.get_mut(&left_id) else {
            return;
        };
        left_deck.0.extend(serves);
        self.prompt_7_player_id = None;
    }

    pub fn one_chance(&mut self, player_id: String, serves: Vec<Card>) {
        // TODO: janken subgame
        // self.prompt_1_player_ids.extend(self.players.clone());
        let Some(deck) = self.fields.get_mut(&player_id) else {
            return;
        };
        *deck -= Deck(serves.clone());
        flush(self);
        self.trushes.0.extend(serves);
        self.current = Some(player_id);
    }

    pub fn serve(&mut self, player_id: String, serves: Deck) {
        if !servable(&self, &serves) {
            return;
        }
        let Some(deck) = self.fields.get_mut(&player_id) else {
            return;
        };
        *deck -= serves.clone();
        self.last_served_player_id = Some(player_id.clone());

        effect_card(self, &player_id, &serves);
        if !self.has_prompt() {
            self.next(&player_id);
        }
    }
}
