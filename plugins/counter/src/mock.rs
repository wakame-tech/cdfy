// #[cfg(not(target = "wasm32-unknown-unknown"))]
// use rand::{thread_rng, Rng};

pub fn rand() -> u32 {
    0
    // let mut rng = thread_rng();
    // rng.gen()
}

/// reserve task and execute returns `task_id`
pub fn reserve(_player_id: String, _room_id: String, _action: String, _timeout: u32) -> String {
    "dummy".to_string()
}

pub struct State {
    pub data: String,
}

pub enum ResultState {
    Ok(State),
    Err(String),
}
