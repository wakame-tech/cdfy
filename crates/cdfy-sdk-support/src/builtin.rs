#[cfg(not(target_arch = "wasm32"))]
use rand::{thread_rng, Rng};

#[cfg(target_arch = "wasm32")]
use cdfy_sdk::debug as _debug;

#[cfg(not(target_arch = "wasm32"))]
pub fn debug(message: String) {
    println!("{}", message);
}

#[cfg(target_arch = "wasm32")]
pub fn debug(message: String) {
    _debug(message)
}

#[cfg(target_arch = "wasm32")]
use cdfy_sdk::rand as _rand;

#[cfg(not(target_arch = "wasm32"))]
pub fn rand() -> u32 {
    let mut rng = thread_rng();
    rng.gen()
}

#[cfg(target_arch = "wasm32")]
pub fn rand() -> u32 {
    _rand()
}

#[cfg(target_arch = "wasm32")]
use cdfy_sdk::reserve as _reserve;

#[cfg(not(target_arch = "wasm32"))]
pub fn reserve(_player_id: String, _room_id: String, _action: String, _timeout: u32) -> String {
    "dummy".to_string()
}

#[cfg(target_arch = "wasm32")]
pub fn reserve(player_id: String, room_id: String, action: String, timeout: u32) -> String {
    _reserve(player_id, room_id, action, timeout)
}
