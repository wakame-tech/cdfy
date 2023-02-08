use super::Effect;
use crate::game::{Action, Game};
use anyhow::Result;

#[derive(Debug)]
pub struct Revolution {
    pub revoluted: bool,
}

impl Revolution {
    pub fn new() -> Self {
        Self { revoluted: false }
    }
}

impl Effect for Revolution {
    fn on_resolve(&mut self, game: &mut Game, _action: &mut Result<Action>) {
        match game.action_stack.last() {
            Some(Action::ServeRiver((_, deck))) => {
                if deck.0.len() == 1 {
                    self.revoluted = !self.revoluted;
                }
            }
            _ => {}
        };
    }

    fn on_push(&mut self, _game: &mut Game, _action: &mut Result<Action>) {}
}

// fn effect_card(&mut self, player: &Player, cards: &Vec<Card>) -> Result<()> {
//     let number = cards.first().and_then(|c| c.number());
//     if number.is_none() {
//         return Ok(());
//     }
//     let number = number.unwrap();
//     match self.ctx.state {
//         State { effect_3: true, .. } => {
//             self.ctx.river.push(cards.clone());
//             Ok(())
//         }
//         State {
//             effect_10: true, ..
//         } if (1..=10).contains(&number) => {
//             self.ctx.river.push(cards.clone());
//             Ok(())
//         }
//         _ => {
//             match number {
//                 3 => {
//                     println!("三途");
//                     self.ctx.state.effect_3 = true;
//                 }
//                 4 => {
//                     println!("死者蘇生");
//                 }
//                 5 => {
//                     println!("スキップ");
//                 }
//                 7 => {
//                     println!("7渡し");
//                 }
//                 8 => {
//                     println!("8切り");
//                     self.reset_turn();
//                 }
//                 9 => {
//                     println!("阿修羅");
//                 }
//                 10 => {
//                     println!("十戒");
//                     self.ctx.state.effect_10 = true;
//                 }
//                 11 => {
//                     println!("イレブンバック");
//                     self.ctx.state.revoluted = true;
//                 }
//                 12 => {
//                     println!("摩訶鉢特摩");
//                 }
//                 13 => {
//                     println!("ロイヤルレリーフ");
//                 }
//                 2 => {
//                     println!("除外");
//                     self.turn_joiners = Vec::new();
//                     self.ctx.exclude();
//                     return Ok(());
//                 }
//                 _ => {}
//             };
//             self.ctx.river.push(cards.clone());
//             Ok(())
//         }
//     }
// }
