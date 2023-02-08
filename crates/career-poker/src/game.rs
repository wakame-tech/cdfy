use super::effects::{cards::Revolution, DefaultCareerPokerRiverGuard, Effect};
use anyhow::Result;
use inquire::MultiSelect;
use playing_card::deck::Deck;
use std::{collections::HashMap, fmt::Display};

/// `Action` is a minimal unit of operationg `Game`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// プレイヤーに出したいカードを問い合わせる
    Prompt((usize, Deck)),
    /// カードを山札に出す
    ServeRiver((usize, Deck)),
    /// 手番を別の人に渡す
    Pass(usize),
    /// 山札から墓地に送る
    FlushRiver,
    /// 山札から除外に送る
    Exclude,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Prompt((u, _)) => write!(f, "Prompt {}", u),
            Action::ServeRiver((u, d)) => write!(f, "Serve River ({}, {})", u, d),
            Action::Pass(u) => write!(f, "Pass {}", u),
            Action::FlushRiver => write!(f, "FlushRiver"),
            Action::Exclude => write!(f, "Exclude"),
        }
    }
}

pub fn next_element<T: PartialEq + Clone>(v: &Vec<T>, e: &T) -> T {
    assert!(!v.is_empty());
    if let Some(i) = v.iter().position(|i| i == e) {
        v[(i + 1) % v.len()].clone()
    } else {
        v[0].clone()
    }
}

#[derive(Debug)]
pub struct Game {
    pub players: HashMap<usize, Deck>,
    pub trushes: Deck,
    pub excluded: Deck,
    pub river: Vec<Deck>,
    pub current_player: usize,
    pub turn_joiners: Vec<usize>,
    pub action_stack: Vec<Action>,
}

impl Game {
    pub fn new(player_count: usize) -> Self {
        let mut deck = Deck::with_jokers(2);
        deck.shuffle();
        let players = HashMap::from_iter(
            deck.0
                .chunks((deck.0.len() / player_count) + 1)
                .into_iter()
                .enumerate()
                .map(|(i, cards)| (i, Deck(cards.to_vec()))),
        );
        let player_ids = players.iter().map(|(i, _)| *i).collect();
        Self {
            players,
            trushes: Deck::new(),
            excluded: Deck::new(),
            river: vec![],
            current_player: 0,
            turn_joiners: player_ids,
            action_stack: vec![],
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

    pub fn push_action(&mut self, action: &Action, effects: &mut Vec<Box<dyn Effect>>) {
        println!("\tpush_action = {}", action);
        let mut action: Result<Action> = Ok(action.clone());
        for effect in effects.iter_mut() {
            effect.on_push(self, &mut action);
        }
        match action {
            Ok(action) => self.action_stack.push(action),
            Err(e) => println!("{}", e),
        };
    }

    pub fn resolve_action(&mut self, action: &Action, effects: &mut Vec<Box<dyn Effect>>) -> bool {
        println!("\tresolve_action {}", action);
        let mut action: Result<Action> = Ok(action.clone());
        for effect in effects.iter_mut() {
            effect.on_resolve(self, &mut action);
        }

        match action {
            Ok(action) => match action {
                Action::Prompt((player_id, deck)) => {
                    println!(
                        "river = {}",
                        self.river
                            .iter()
                            .map(|d| d.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    );
                    let options: Vec<_> = deck.0.to_vec();
                    let cards = MultiSelect::new(format!("player {}", player_id).as_str(), options)
                        .prompt()
                        .unwrap();
                    self.push_action(&Action::ServeRiver((player_id, Deck(cards))), effects);
                }
                Action::ServeRiver((_, served)) => {
                    self.river.push(served);
                    if self.turn_joiners.len() == 1 {
                        self.turn_joiners = self.players.keys().cloned().collect::<Vec<usize>>();
                    }
                    self.current_player = next_element(&self.turn_joiners, &self.current_player);
                    let deck = self.players.get(&self.current_player).unwrap();
                    self.push_action(
                        &Action::Prompt((self.current_player, deck.clone())),
                        effects,
                    )
                }
                Action::Pass(player_id) => {
                    self.turn_joiners.remove(player_id);
                }
                Action::FlushRiver => self.flush(),
                Action::Exclude => self.exclude(),
            },
            Err(e) => println!("{}", e),
        }
        true
    }

    pub fn run(&mut self) -> Result<()> {
        let mut effects: Vec<Box<dyn Effect>> = vec![
            Box::new(DefaultCareerPokerRiverGuard),
            Box::new(Revolution::new()),
        ];
        self.push_action(
            &Action::Prompt((
                self.current_player,
                self.players[&self.current_player].clone(),
            )),
            &mut effects,
        );

        while let Some(action) = self.action_stack.pop() {
            println!(
                "actions = [{}]",
                self.action_stack
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            self.resolve_action(&action, &mut effects);
        }
        Ok(())
    }
}
