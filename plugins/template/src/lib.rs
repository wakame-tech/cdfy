use extism_pdk::*;
use serde::{Deserialize, Serialize};
use tera::Tera;

use crate::plugin::{GameConfig, LiveEvent};

mod plugin;

static APP_HTML: &[u8] = include_bytes!("templates/app.html");

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub count: usize,
}

impl ToBytes<'_> for Game {
    type Bytes = Vec<u8>;

    fn to_bytes(&self) -> Result<Self::Bytes, Error> {
        Ok(serde_json::to_vec(self)?)
    }
}

impl FromBytesOwned for Game {
    fn from_bytes_owned(bytes: &[u8]) -> Result<Self, Error> {
        Ok(serde_json::from_slice(&bytes)?)
    }
}

#[plugin_fn]
pub fn init_game(Json(_): Json<GameConfig>) -> FnResult<()> {
    let game = Game { count: 0 };
    var::set("game", &game)?;
    Ok(())
}

// debug
#[plugin_fn]
pub fn get_state(_: ()) -> FnResult<String> {
    Ok(var::get("game")?
        .map(|s: Game| serde_json::to_string(&s).unwrap())
        .unwrap_or("nil".to_string()))
}

#[plugin_fn]
pub fn handle_event(Json(event): Json<LiveEvent>) -> FnResult<()> {
    let mut game: Game = var::get("game")?.unwrap();
    game = match event.event_name.as_str() {
        "reset" => Game { count: 0 },
        "increment" => Game {
            count: game.count + 1,
        },
        _ => game,
    };
    var::set("game", &game)?;
    Ok(())
}

#[plugin_fn]
pub fn render(_: ()) -> FnResult<String> {
    let game: Game = var::get("game")?.unwrap();
    let mut context = tera::Context::new();
    context.insert("game", &game);
    Ok(Tera::one_off(
        std::str::from_utf8(APP_HTML).unwrap(),
        &context,
        false,
    )?)
}
