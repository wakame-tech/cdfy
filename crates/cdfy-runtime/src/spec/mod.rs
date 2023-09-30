use rand::{thread_rng, Rng};

pub mod bindings;
pub mod types;

#[allow(unused_imports)]
use types::*;

fn debug(message: String) {
    println!("{}", message);
}

fn rand() -> u32 {
    thread_rng().gen()
}

fn reserve(player_id: String, room_id: String, action: String, timeout: u32) {
    todo!()
}
