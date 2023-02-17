use crate::{
    card::{Card, Suit},
    deck::{
        is_same_number, match_suits, number, remove_items, suits, with_jokers, Deck, DeckStyle,
    },
    will_flush,
};
use cdfy_sdk::cancel;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub river_size: Option<usize>,
    pub suit_limits: HashSet<Suit>,
    /// a number includes `effect_limits` ignore effect
    pub effect_limits: HashSet<u8>,
    /// card strength is reversed until the river is reset
    pub turn_revoluted: bool,
    /// when `is_step` is true, delta of previous cards number must be 1
    pub is_step: bool,
    /// when `revoluted` is true, card strength is reversed
    pub revoluted: bool,
}

impl Effect {
    pub fn new() -> Self {
        Self {
            river_size: None,
            suit_limits: HashSet::new(),
            effect_limits: HashSet::new(),
            turn_revoluted: false,
            is_step: false,
            revoluted: false,
        }
    }

    pub fn new_turn(effect: Effect) -> Self {
        Self {
            river_size: None,
            suit_limits: HashSet::new(),
            effect_limits: HashSet::new(),
            turn_revoluted: false,
            is_step: false,
            ..effect
        }
    }
}

fn cardinal(n: u8) -> u8 {
    (n + 10) % 13
}

fn card_ord(l: &Card, r: &Card) -> Ordering {
    let (ln, rn) = (l.number(), r.number());
    match (ln, rn) {
        (None, None) => Ordering::Less,
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (Some(i), Some(j)) => cardinal(i).cmp(&cardinal(j)),
    }
}

fn vec_ord<T, F>(l: impl Iterator<Item = T>, r: impl Iterator<Item = T>, ord: F) -> Ordering
where
    F: Fn(T, T) -> Ordering,
{
    let orderings = l.zip(r).map(|(a, b)| ord(a, b)).collect::<HashSet<_>>();
    orderings.into_iter().next().unwrap_or(Ordering::Equal)
}

fn deck_ord(lhs: &Vec<Card>, rhs: &Vec<Card>) -> Ordering {
    let (mut lhs, mut rhs) = (lhs.clone(), rhs.clone());
    lhs.sort_by(|a, b| card_ord(a, b));
    rhs.sort_by(|a, b| card_ord(a, b));
    vec_ord(lhs.iter(), rhs.iter(), card_ord)
}

pub fn servable(state: &CareerPokerState, serves: &Vec<Card>) -> bool {
    let mut ok = is_same_number(serves);
    let Some(lasts) = state.river.last() else {
        // river is empty
        return ok;
    };
    let river_size = state.effect.river_size.unwrap();
    // check ordering
    let ordering = if state.effect.revoluted ^ state.effect.turn_revoluted {
        deck_ord(&lasts.cards, serves).reverse()
    } else {
        deck_ord(&lasts.cards, serves)
    };
    ok = ok && ordering.is_lt();

    // check river size
    ok = ok
        && match number(serves) {
            9 if !state.effect.effect_limits.contains(&9) => servable_9(state, serves),
            _ => serves.len() == river_size,
        };
    // check steps
    if state.effect.is_step {
        ok = ok && cardinal(number(&lasts.cards)) - cardinal(number(serves)) == 1;
    }
    // check suits
    if !state.effect.suit_limits.is_empty() {
        ok = ok && match_suits(&lasts.cards, serves);
    }
    ok
}

pub fn effect_revolution(state: &mut CareerPokerState, _player_id: &str, serves: &Vec<Card>) {
    if serves.len() == 4 {
        state.effect.revoluted = !state.effect.revoluted;
    }
}

pub fn effect_3(state: &mut CareerPokerState, _player_id: &str, _serves: &Vec<Card>) {
    state.effect.effect_limits.extend(1..13)
}

pub fn effect_4(state: &mut CareerPokerState, player_id: &str, _serves: &Vec<Card>) {
    let hands = state.fields.get(player_id).unwrap();
    if hands.cards.is_empty() || state.trushes.cards.is_empty() {
        return;
    }
    state
        .prompts
        .insert(player_id.to_string(), "trushes".to_string());
}

pub fn effect_5(state: &mut CareerPokerState, player_id: &str, serves: &Vec<Card>) {
    state.current = state.get_relative_player(player_id, 1 + serves.len() as i32);
}

pub fn effect_7(state: &mut CareerPokerState, player_id: &str, _serves: &Vec<Card>) {
    state
        .prompts
        .insert(player_id.to_string(), player_id.to_string());
}

pub fn effect_8(state: &mut CareerPokerState, player_id: &str, _serves: &Vec<Card>) {
    state.will_flush(player_id, "trushes");
}

pub fn effect_9(state: &mut CareerPokerState, _player_id: &str, _serves: &Vec<Card>) {
    state.effect.river_size = match state.effect.river_size {
        Some(1) => Some(3),
        Some(3) => Some(1),
        n => n,
    }
}

pub fn servable_9(state: &CareerPokerState, serves: &Vec<Card>) -> bool {
    let river_size = state.effect.river_size.unwrap();
    river_size
        == match river_size {
            3 => 1,
            1 => 3,
            n => n,
        }
}

pub fn effect_10(state: &mut CareerPokerState, _player_id: &str, _serves: &Vec<Card>) {
    state.effect.effect_limits.extend(1..10)
}

pub fn effect_11(state: &mut CareerPokerState, _player_id: &str, _serves: &Vec<Card>) {
    state.effect.turn_revoluted = true;
}

pub fn effect_12(state: &mut CareerPokerState, _player_id: &str, serves: &Vec<Card>) {
    state.effect.is_step = true;
    state.effect.suit_limits.extend(suits(serves));
}

pub fn effect_13(state: &mut CareerPokerState, player_id: &str, _serves: &Vec<Card>) {
    let hands = state.fields.get(player_id).unwrap();
    if hands.cards.is_empty() || state.excluded.cards.is_empty() {
        return;
    }
    state
        .prompts
        .insert(player_id.to_string(), "excluded".to_string());
}

pub fn effect_1(state: &mut CareerPokerState, _player_id: &str, _serves: &Vec<Card>) {
    for player_id in state.players.iter() {
        state
            .prompts
            .insert(player_id.to_string(), "janken".to_string());
    }
}

pub fn effect_2(state: &mut CareerPokerState, player_id: &str, _serves: &Vec<Card>) {
    state.will_flush(player_id, "excluded");
}

pub fn effect_card(state: &mut CareerPokerState, player_id: &str, serves: &Vec<Card>) {
    state.effect.river_size = Some(serves.len());

    effect_revolution(state, player_id, serves);
    state.river.push(Deck {
        style: DeckStyle::Arrange,
        cards: serves.clone(),
    });
    state.effect.river_size = Some(serves.len());

    let n = number(serves);
    match n {
        3 if !state.effect.effect_limits.contains(&n) => effect_3(state, player_id, serves),
        4 if !state.effect.effect_limits.contains(&n) => effect_4(state, player_id, serves),
        5 if !state.effect.effect_limits.contains(&n) => effect_5(state, player_id, serves),
        7 if !state.effect.effect_limits.contains(&n) => effect_7(state, player_id, serves),
        8 if !state.effect.effect_limits.contains(&n) => effect_8(state, player_id, serves),
        9 if !state.effect.effect_limits.contains(&n) => effect_9(state, player_id, serves),
        10 if !state.effect.effect_limits.contains(&n) => effect_10(state, player_id, serves),
        11 if !state.effect.effect_limits.contains(&n) => effect_11(state, player_id, serves),
        12 if !state.effect.effect_limits.contains(&n) => effect_12(state, player_id, serves),
        13 if !state.effect.effect_limits.contains(&n) => effect_13(state, player_id, serves),
        1 if !state.effect.effect_limits.contains(&n) => effect_1(state, player_id, serves),
        2 if !state.effect.effect_limits.contains(&n) => effect_2(state, player_id, serves),
        _ => {}
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CareerPokerState {
    pub room_id: String,
    pub current: Option<String>,
    pub players: Vec<String>,
    pub excluded: Deck,
    pub trushes: Deck,
    pub river: Vec<Deck>,
    pub will_flush_task_id: Option<String>,
    pub last_served_player_id: Option<String>,
    pub fields: HashMap<String, Deck>,
    pub effect: Effect,
    /// pair of user id to deck id for prompt cards
    pub prompts: HashMap<String, String>,
}

impl CareerPokerState {
    pub fn new(room_id: String) -> Self {
        Self {
            room_id,
            excluded: Deck::new(vec![]),
            trushes: Deck::new(vec![]),
            river: vec![],
            will_flush_task_id: None,
            players: vec![],
            last_served_player_id: None,
            current: None,
            fields: HashMap::new(),
            effect: Effect::new(),
            prompts: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        *self = Self {
            room_id: self.room_id.to_string(),
            excluded: Deck::new(vec![]),
            trushes: Deck::new(vec![]),
            river: vec![],
            will_flush_task_id: None,
            players: self.players.clone(),
            last_served_player_id: None,
            current: None,
            fields: HashMap::new(),
            effect: Effect::new(),
            prompts: HashMap::new(),
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
            self.fields.remove(&player_id);
        }
    }

    pub fn distribute(&mut self) {
        self.reset();
        let cards = with_jokers(2);
        let players = self.players.iter().collect::<Vec<_>>();
        for player_id in players.iter() {
            self.fields.insert(
                player_id.to_string(),
                Deck {
                    cards: vec![],
                    style: DeckStyle::Arrange,
                },
            );
        }
        for (i, card) in cards.into_iter().enumerate() {
            let player_id = players[i % players.len()];
            if let Some(deck) = self.fields.get_mut(player_id) {
                deck.cards.push(card);
            }
        }
        for player_id in players.iter() {
            if let Some(deck) = self.fields.get_mut(*player_id) {
                deck.cards.sort_by(|a, b| card_ord(a, b))
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
                if !deck.cards.is_empty() {
                    return Some(self.players[index].clone());
                }
            }
            if delta as usize == self.players.len() {
                return None;
            }
            delta += 1;
        }
    }

    pub fn will_flush(&mut self, player_id: &str, to: &str) {
        self.will_flush_task_id = Some(will_flush(
            player_id.to_string(),
            self.room_id.to_string(),
            to.to_string(),
        ));
    }

    pub fn flush(&mut self, to: String) {
        let cards = self
            .river
            .iter()
            .map(|d| d.cards.clone())
            .flatten()
            .collect::<Vec<_>>();
        match to.as_str() {
            "trushes" => self.trushes.cards.extend(cards),
            "excluded" => self.excluded.cards.extend(cards),
            _ => panic!(),
        };
        self.effect = Effect::new_turn(self.effect.clone());
        self.river.clear();
        self.current = self.last_served_player_id.clone();
    }

    pub fn next(&mut self, player_id: &str) {
        self.current = self.get_relative_player(&player_id, 1);
        if self.current == self.last_served_player_id {
            self.will_flush(player_id, "trushes");
        }
        if self.current.is_none() {
            self.will_flush(player_id, "trushes");
        }
    }

    pub fn pass(&mut self, player_id: String) {
        self.next(&player_id);
    }

    pub fn select_trushes(&mut self, player_id: String, serves: Vec<Card>) {
        if self.river.last().unwrap().cards.len() != serves.len() {
            return;
        }
        let Some(deck) = self.fields.get_mut(&player_id) else {
            return;
        };
        remove_items(&mut self.trushes.cards, &serves);
        deck.cards.extend(serves);
        self.prompts.remove(&player_id);
        self.next(&player_id);
    }

    pub fn select_excluded(&mut self, player_id: String, serves: Vec<Card>) {
        if self.river.last().unwrap().cards.len() != serves.len() {
            return;
        }
        let Some(deck) = self.fields.get_mut(&player_id) else {
            return;
        };
        remove_items(&mut self.excluded.cards, &serves);
        deck.cards.extend(serves);
        self.prompts.remove(&player_id);
        self.next(&player_id);
    }

    pub fn select_passes(&mut self, player_id: String, serves: Vec<Card>) {
        if self.river.last().unwrap().cards.len() != serves.len() {
            return;
        }
        let left_id = self.get_relative_player(&player_id, -1).unwrap();
        let Some(deck) = self.fields.get_mut(&player_id) else {
            return;
        };
        remove_items(&mut deck.cards, &serves);
        let Some(left_deck) = self.fields.get_mut(&left_id) else {
            return;
        };
        left_deck.cards.extend(serves);
        self.prompts.remove(&player_id);
    }

    pub fn one_chance(&mut self, player_id: String, serves: Vec<Card>) {
        let Some(deck) = self.fields.get_mut(&player_id) else {
            return;
        };

        remove_items(&mut deck.cards, &serves);
        if let Some(task_id) = self.will_flush_task_id.as_ref() {
            cancel(self.room_id.clone(), task_id.to_string());
        }

        // effect_card(self, &player_id, &serves);
        // self.flush();
        // self.trushes.cards.extend(serves);
        // self.current = Some(player_id);
    }

    pub fn serve(&mut self, player_id: String, serves: Vec<Card>) {
        if !servable(&self, &serves) {
            return;
        }
        let Some(deck) = self.fields.get_mut(&player_id) else {
            return;
        };
        remove_items(&mut deck.cards, &serves);
        self.last_served_player_id = Some(player_id.clone());

        effect_card(self, &player_id, &serves);
        if self.prompts.is_empty() && self.will_flush_task_id.is_none() {
            self.next(&player_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        deck::Deck,
        state::{servable, CareerPokerState},
    };
    use std::collections::HashMap;

    #[test]
    fn test_servable() {
        let mut state = CareerPokerState::new("".to_string());
        let serves = vec!["3h".into(), "3d".into()];
        assert_eq!(servable(&state, &serves), true);

        state.effect.river_size = Some(1);
        state.river.push(Deck::new(vec!["Kh".into()]));

        let serves = vec!["Ah".into()];
        assert_eq!(servable(&state, &serves), true);
    }

    #[test]
    fn test_get_relative_player() {
        let mut state = CareerPokerState::new("".to_string());
        state.fields = HashMap::from_iter(vec![
            ("a".to_string(), Deck::new(vec!["Ah".into()])),
            ("b".to_string(), Deck::new(vec!["Ah".into()])),
            ("c".to_string(), Deck::new(vec!["Ah".into()])),
        ]);
        state.players = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(state.get_relative_player("a", 1), Some("b".to_string()));
        assert_eq!(state.get_relative_player("a", -1), Some("c".to_string()));
        assert_eq!(state.get_relative_player("a", 2), Some("c".to_string()));
        assert_eq!(state.get_relative_player("a", 3), Some("a".to_string()));

        let mut state = CareerPokerState::new("".to_string());
        state.fields = HashMap::from_iter(vec![
            ("a".to_string(), Deck::new(vec!["Ah".into()])),
            ("b".to_string(), Deck::new(vec!["Ah".into()])),
        ]);
        state.players = vec!["a".to_string(), "b".to_string()];
        assert_eq!(state.get_relative_player("a", 1), Some("b".to_string()));
        assert_eq!(state.get_relative_player("a", -1), Some("b".to_string()));
        assert_eq!(state.get_relative_player("a", 2), Some("a".to_string()));
    }
}
