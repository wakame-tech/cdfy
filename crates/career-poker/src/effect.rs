use crate::{
    deck::Deck,
    game::number,
    state::{CareerPokerState, Effect},
};

pub fn flush(state: &mut CareerPokerState) {
    state.trushes.0.extend(
        state
            .river
            .iter()
            .flat_map(|d| d.0.clone())
            .collect::<Vec<_>>(),
    );
    state.effect = Effect::new_turn(state.effect.clone());
    state.river.clear();
    state.current = state.last_served_player_id.clone();
}

fn exclude(state: &mut CareerPokerState) {
    state.excluded.0.extend(
        state
            .river
            .iter()
            .flat_map(|d| d.0.clone())
            .collect::<Vec<_>>(),
    );
    state.river.clear();
    state.current = state.last_served_player_id.clone();
}

pub fn effect_card(state: &mut CareerPokerState, player_id: &str, serves: &Deck) {
    let n = number(&serves.0);
    state.effect.river_size = Some(serves.0.len());
    if serves.0.len() == 4 {
        state.effect.revoluted = !state.effect.revoluted;
    }
    state.river.push(serves.clone());
    if !state.effect.effect_3 && !(state.effect.effect_10 && 1 <= n && n <= 10) {
        match n {
            3 => {
                state.effect.effect_3 = true;
            }
            4 => {
                let hands = state.fields.get(player_id).unwrap();
                if hands.0.is_empty() || state.trushes.0.is_empty() {
                    return;
                }
                state.prompt_4_player_id = Some(player_id.to_string());
            }
            5 => {
                state.current = state.get_relative_player(player_id, 1 + serves.0.len() as i32);
            }
            7 => {
                state.prompt_7_player_id = Some(player_id.to_string());
            }
            8 => {
                flush(state);
            }
            9 => {
                state.effect.river_size = match state.effect.river_size {
                    Some(1) => Some(3),
                    Some(3) => Some(1),
                    _ => panic!(),
                }
            }
            10 => {
                state.effect.effect_10 = true;
            }
            11 => {
                state.effect.effect_11 = true;
            }
            12 => {
                state.effect.effect_12 = true;
            }
            13 => {
                let hands = state.fields.get(player_id).unwrap();
                if hands.0.is_empty() || state.excluded.0.is_empty() {
                    return;
                }
                state.prompt_13_player_id = Some(player_id.to_string());
            }
            2 => {
                exclude(state);
            }
            _ => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{deck::Deck, state::CareerPokerState};

    #[test]
    fn test_get_relative_player() {
        let mut state = CareerPokerState::new();
        state.fields = HashMap::from_iter(vec![
            ("a".to_string(), Deck(vec!["Ah".into()])),
            ("b".to_string(), Deck(vec!["Ah".into()])),
            ("c".to_string(), Deck(vec!["Ah".into()])),
        ]);
        state.players = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(state.get_relative_player("a", 1), Some("b".to_string()));
        assert_eq!(state.get_relative_player("a", -1), Some("c".to_string()));
        assert_eq!(state.get_relative_player("a", 2), Some("c".to_string()));
        assert_eq!(state.get_relative_player("a", 3), Some("a".to_string()));
    }
}
