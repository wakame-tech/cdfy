use crate::{
    card::Card,
    deck::{deck, remove_cards},
};
use anyhow::{anyhow, Result};
use inquire::MultiSelect;
use rand::{seq::SliceRandom, thread_rng};
use std::{cmp::Ordering, collections::HashSet, fmt::Display};

fn fmt<T: Display>(vec: &Vec<T>) -> String {
    if vec.is_empty() {
        "(empty)".to_string()
    } else {
        vec.iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn ord(a: &Card, b: &Card) -> Ordering {
    match (a, b) {
        (Card::Joker, Card::Joker) => Ordering::Equal,
        (Card::Joker, _) => Ordering::Greater,
        (_, Card::Joker) => Ordering::Less,
        (Card::Number(_, i), Card::Number(_, j)) => ((i + 10) % 13).cmp(&((j + 10) % 13)),
    }
}

fn is_same_number(cards: &Vec<Card>) -> bool {
    let numbers: HashSet<_> = cards.iter().filter_map(|c| c.number()).collect();
    // if only jokers, len == 0
    numbers.len() <= 1
}

fn vec_ord<F, T: Clone>(a: &Vec<T>, b: &Vec<T>, cmp: F) -> Ordering
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut a = a.clone();
    a.sort_by(&cmp);
    let mut b = b.clone();
    b.sort_by(&cmp);
    let orderings: Vec<_> = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| cmp(a, b))
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

#[derive(Debug)]
pub struct State {
    revoluted: bool,
    effect_3: bool,
    effect_10: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            revoluted: false,
            effect_3: false,
            effect_10: false,
        }
    }
}

#[derive(Debug)]
pub struct Context {
    pub trushes: Vec<Card>,
    pub excluded: Vec<Card>,
    pub river: Vec<Vec<Card>>,
    pub state: State,
}

impl Context {
    pub fn new() -> Self {
        Self {
            trushes: vec![],
            excluded: vec![],
            river: vec![],
            state: Default::default(),
        }
    }

    pub fn servable(&self, cards: &Vec<Card>) -> Result<()> {
        if let Some(lasts) = self.river.last() {
            if lasts.len() == cards.len()
                && is_same_number(cards)
                && vec_ord(cards, lasts, ord)
                    == if self.state.revoluted {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
            {
                Ok(())
            } else {
                Err(anyhow!("cannot served {} <= {}", fmt(&cards), fmt(&lasts)))
            }
        } else {
            Ok(())
        }
    }

    pub fn flush(&mut self) {
        self.trushes
            .extend(self.river.iter().flatten().cloned().collect::<Vec<_>>());
        self.river.clear();
    }

    pub fn exclude(&mut self) {
        self.excluded
            .extend(self.river.iter().flatten().cloned().collect::<Vec<_>>());
        self.river.clear();
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: usize,
    pub hands: Vec<Card>,
}

impl Player {
    pub fn prompt(&self, ctx: &Context) -> Result<Option<Vec<Card>>> {
        let lasts = ctx.river.last().cloned().unwrap_or(vec![]);
        println!("river {}", fmt(&lasts));
        let q = format!("player {} ({} cards) action?", self.id, self.hands.len());
        let mut options: Vec<_> = self
            .hands
            .iter()
            .filter(|c| {
                if lasts.is_empty() {
                    true
                } else if ctx.state.revoluted {
                    ord(c, lasts.first().unwrap()).is_lt()
                } else {
                    ord(c, lasts.first().unwrap()).is_gt()
                }
            })
            .cloned()
            .collect();
        // force pass
        if options.is_empty() {
            return Ok(None);
        }
        options.sort_by(|a, b| ord(a, b));
        let ans = MultiSelect::new(q.as_str(), options).prompt()?;
        if ans.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ans))
        }
    }

    pub fn distribute(players: usize, jokers: usize) -> Vec<Player> {
        let mut cards = deck(jokers);
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        cards
            .chunks(cards.len() / players)
            .enumerate()
            .map(|(i, hands)| Player {
                id: i,
                hands: hands.to_vec(),
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Game {
    ctx: Context,
    turn_joiners: Vec<usize>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            ctx: Context::new(),
            turn_joiners: Vec::new(),
        }
    }

    fn action(&mut self, player: &mut Player) -> Result<Option<Vec<Card>>> {
        loop {
            let cards = player.prompt(&self.ctx)?;
            if let Some(cards) = cards {
                match self.ctx.servable(&cards) {
                    Ok(_) => {
                        self.effect_card(player, &cards)?;
                        remove_cards(&mut player.hands, &cards)?;
                        return Ok(Some(cards));
                    }
                    Err(e) => println!("{}", e),
                }
            } else {
                // cannot pass if not_passed_players is only you
                if self.turn_joiners.len() == 1 && self.turn_joiners.contains(&player.id) {
                    println!("cannot passed because all except you players has already passed");
                    continue;
                }
                println!("passed");
                return Ok(None);
            }
        }
    }

    fn effect_card(&mut self, player: &Player, cards: &Vec<Card>) -> Result<()> {
        let number = cards.first().and_then(|c| c.number());
        if number.is_none() {
            return Ok(());
        }
        let number = number.unwrap();
        match self.ctx.state {
            State { effect_3: true, .. } => {
                self.ctx.river.push(cards.clone());
                Ok(())
            }
            State {
                effect_10: true, ..
            } if (1..=10).contains(&number) => {
                self.ctx.river.push(cards.clone());
                Ok(())
            }
            _ => {
                match number {
                    3 => {
                        println!("三途");
                        self.ctx.state.effect_3 = true;
                    }
                    4 => {
                        println!("死者蘇生");
                    }
                    5 => {
                        println!("スキップ");
                    }
                    7 => {
                        println!("7渡し");
                    }
                    8 => {
                        println!("8切り");
                        self.reset_turn();
                    }
                    9 => {
                        println!("阿修羅");
                    }
                    10 => {
                        println!("十戒");
                        self.ctx.state.effect_10 = true;
                    }
                    11 => {
                        println!("イレブンバック");
                        self.ctx.state.revoluted = true;
                    }
                    12 => {
                        println!("摩訶鉢特摩");
                    }
                    13 => {
                        println!("ロイヤルレリーフ");
                    }
                    2 => {
                        println!("除外");
                        self.turn_joiners = Vec::new();
                        self.ctx.exclude();
                        return Ok(());
                    }
                    _ => {}
                };
                self.ctx.river.push(cards.clone());
                Ok(())
            }
        }
    }

    fn reset_turn(&mut self) {
        println!("turn end");
        self.ctx.flush();
        self.ctx.state = Default::default();
    }

    fn next(
        &mut self,
        players: &Vec<Player>,
        current: Option<usize>,
        passed: bool,
    ) -> Option<usize> {
        let pos = self.turn_joiners.iter().position(|i| Some(*i) == current);
        println!("current: {:?} @ {:?}", current, pos);
        println!("turn_joiners: {:?}", self.turn_joiners);
        if let Some(i) = pos {
            let next = self
                .turn_joiners
                .get((i + 1) % self.turn_joiners.len())
                .cloned();
            if passed {
                self.turn_joiners.remove(i);
            }
            if self.turn_joiners.len() == 1 {
                self.reset_turn();
                self.turn_joiners = players
                    .iter()
                    .filter(|p| !p.hands.is_empty())
                    .map(|p| p.id)
                    .collect();
            }
            next
        } else {
            self.turn_joiners.first().cloned()
        }
    }

    pub fn run(&mut self, players: &mut Vec<Player>) -> Result<()> {
        self.turn_joiners = players.iter().map(|p| p.id).collect();
        let mut current = self.next(players, None, false);
        loop {
            if let Some(id) = current {
                let player = players.iter_mut().find(|p| p.id == id).unwrap();
                let res = self.action(player)?;
                current = self.next(players, current, res.is_none());
            } else {
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{
        all_roles_career_poker::{ord, vec_ord},
        card::{Card, Suit},
    };
    #[test]
    fn test_ord() {
        assert_eq!(
            vec_ord(
                &vec![Card::Joker],
                &vec![Card::Number(Suit::Spade, 13)],
                ord
            ),
            Ordering::Greater
        );
        assert_eq!(
            vec_ord(
                &vec![Card::Joker, Card::Number(Suit::Spade, 9)],
                &vec![Card::Number(Suit::Spade, 13), Card::Joker],
                ord
            ),
            Ordering::Less
        );
    }
}
