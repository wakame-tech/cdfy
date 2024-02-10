use crate::plugin::{GameConfig, LiveEvent};
use anyhow::{anyhow, Result};
use extism_pdk::*;
use game::Game;
use plugin::RenderConfig;
use tera::Tera;

mod game;
mod plugin;

static APP_HTML: &[u8] = include_bytes!("templates/app.html");

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
pub fn init_game(Json(config): Json<GameConfig>) -> FnResult<()> {
    let game = Game::new(config.player_ids);
    var::set("game", &game)?;
    Ok(())
}

// debug
#[plugin_fn]
pub fn get_state(_: ()) -> FnResult<Game> {
    let game = var::get("game")?.ok_or(anyhow!("Game not found"))?;
    Ok(game)
}

#[plugin_fn]
pub fn handle_event(Json(event): Json<LiveEvent>) -> FnResult<()> {
    let mut game: Game = var::get("game")?.ok_or(anyhow!("Game not found"))?;

    match event.event_name.as_str() {
        "increment" => {
            game.increment();
        }
        _ => {}
    };
    var::set("game", &game)?;
    Ok(())
}

#[plugin_fn]
pub fn render(Json(_config): Json<RenderConfig>) -> FnResult<String> {
    let game: Game = var::get("game")?.ok_or(anyhow!("Game not found"))?;
    let mut context = tera::Context::new();
    context.insert("count", &game.count);
    let html = Tera::one_off(std::str::from_utf8(APP_HTML)?, &context, false)?;
    Ok(html)
}
