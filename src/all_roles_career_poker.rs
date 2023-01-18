use crate::{card::Card, deck::deck};
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

fn migher<F, T>(a: impl Iterator<Item = T>, b: impl Iterator<Item = T>, cmp: F) -> bool
where
    F: Fn(T, T) -> Ordering,
{
    a.zip(b).map(|(a, b)| cmp(a, b)).all(|o| o.is_gt())
}

#[derive(Debug)]
struct Context {
    pub trushes: Vec<Card>,
    pub excluded: Vec<Card>,
    pub river: Vec<Vec<Card>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            trushes: vec![],
            excluded: vec![],
            river: vec![],
        }
    }

    pub fn serve(&mut self, cards: &Vec<Card>) -> Result<()> {
        if let Some(lasts) = self.river.last() {
            if lasts.len() == cards.len()
                && is_same_number(cards)
                && migher(cards.iter(), lasts.iter(), ord)
            {
                self.river.push(cards.clone());
                Ok(())
            } else {
                Err(anyhow!("cannot served {} <= {}", fmt(&cards), fmt(&lasts)))
            }
        } else {
            self.river.push(cards.clone());
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
    pub fn prompt(&self, lasts: Option<&Vec<Card>>) -> Result<Option<Vec<Card>>> {
        println!("river {}", fmt(lasts.unwrap_or(&vec![])));
        let q = format!("player {} ({} cards) action?", self.id, self.hands.len());
        let mut options: Vec<_> = self
            .hands
            .iter()
            .filter(|c| {
                if let Some(lasts) = lasts {
                    ord(c, lasts.first().unwrap()).is_gt()
                } else {
                    true
                }
            })
            .cloned()
            .collect();
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

    pub fn remove(&mut self, cards: &Vec<Card>) -> bool {
        let indices = cards
            .iter()
            .map(|c| self.hands.iter().position(|h| h == c))
            .collect::<Option<Vec<_>>>();
        if let Some(indices) = indices {
            self.hands = self
                .hands
                .iter()
                .enumerate()
                .filter(|(i, _)| !indices.contains(i))
                .map(|(_, c)| c)
                .cloned()
                .collect();
            true
        } else {
            false
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
    not_passed_players: HashSet<usize>,
}

impl Game {
    pub fn new(players: &Vec<Player>) -> Self {
        let ids: Vec<_> = players.iter().map(|p| p.id).collect();
        Self {
            ctx: Context::new(),
            not_passed_players: HashSet::from_iter(ids),
        }
    }

    fn action(&mut self, player: &mut Player) -> Result<Option<Vec<Card>>> {
        if player.hands.is_empty() || !self.not_passed_players.contains(&player.id) {
            return Ok(None);
        }

        loop {
            let cards = player.prompt(self.ctx.river.last())?;
            if let Some(cards) = cards {
                match self.ctx.serve(&cards) {
                    Ok(_) => {
                        player.remove(&cards);
                        return Ok(Some(cards));
                    }
                    Err(e) => println!("{}", e),
                }
            } else {
                // cannot pass if not_passed_players is only you
                if self.not_passed_players.len() == 1
                    && self.not_passed_players.contains(&player.id)
                {
                    println!("cannot passed because all except you players has already passed");
                    continue;
                }
                self.not_passed_players.remove(&player.id);
                println!("passed");
                return Ok(None);
            }
        }
    }

    fn effect_card(&mut self, player: &Player, cards: &Vec<Card>) -> Result<()> {
        let number = cards.first().and_then(|c| c.number());
        match number {
            Some(3) => {
                println!("三途");
            }
            Some(4) => {
                println!("死者蘇生");
            }
            Some(5) => {
                println!("スキップ");
            }
            Some(7) => {
                println!("7渡し");
            }
            Some(8) => {
                println!("8切り");
                self.not_passed_players = HashSet::from_iter(vec![player.id]);
                self.ctx.flush();
            }
            Some(9) => {
                println!("阿修羅");
            }
            Some(10) => {
                println!("十戒");
            }
            Some(11) => {
                println!("イレブンバック");
            }
            Some(12) => {
                println!("摩訶鉢特摩");
            }
            Some(13) => {
                println!("ロイヤルレリーフ");
            }
            Some(2) => {
                println!("除外")
            }
            _ => {}
        }
        Ok(())
    }

    pub fn run(&mut self, players: &mut Vec<Player>) -> Result<()> {
        loop {
            if players.iter().filter(|p| !p.hands.is_empty()).count() == 1 {
                return Ok(());
            }
            for player in players.iter_mut() {
                if self.not_passed_players.len() == 1
                    && self.not_passed_players.contains(&player.id)
                {
                    println!("all passed, river flushed");
                    self.ctx.flush();
                    self.not_passed_players = players
                        .iter()
                        .filter(|p| !p.hands.is_empty())
                        .map(|p| p.id)
                        .collect::<HashSet<_>>();

                    break;
                } else {
                    let served = self.action(player)?;
                    if let Some(served) = served {
                        self.effect_card(player, &served)?;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        all_roles_career_poker::ord,
        card::{Card, Suit},
    };

    use super::migher;

    #[test]
    fn test_ord() {
        assert_eq!(
            migher(
                vec![Card::Joker].iter(),
                vec![Card::Number(Suit::Spade, 13)].iter(),
                ord
            ),
            true
        );
    }
}
