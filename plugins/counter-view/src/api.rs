use cdfy_sdk_support::Event;
use gloo::console::console;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    WillIncrement,
    Increment,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    room_id: String,
    users: HashSet<String>,
    states: HashMap<String, String>,
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

static ORIGIN: &'static str = "http://localhost:1234";

pub async fn fetch_room(room_id: &str) -> Result<Room, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("{}/rooms/{}", ORIGIN, room_id);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;
    let room: Room = serde_wasm_bindgen::from_value(json).map_err(|e| FetchError {
        err: JsValue::from_str(e.to_string().as_str()),
    })?;
    Ok(room)
}

pub async fn join_room(room_id: &str, user_id: &str) -> Result<Room, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let url = format!("{}/rooms/{}/join/{}", ORIGIN, room_id, user_id);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo::utils::window();
    let resp = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    let room: Room = serde_wasm_bindgen::from_value(json).map_err(|e| FetchError {
        err: JsValue::from_str(e.to_string().as_str()),
    })?;
    Ok(room)
}

pub async fn create_room(room_id: &str) -> Result<Room, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let url = format!("{}/rooms/{}", ORIGIN, room_id);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo::utils::window();
    let resp = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    let room: Room = serde_wasm_bindgen::from_value(json).map_err(|e| FetchError {
        err: JsValue::from_str(e.to_string().as_str()),
    })?;
    Ok(room)
}

pub async fn load_plugin(room_id: &str) -> Result<Room, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let url = format!("{}/rooms/{}/counter", ORIGIN, room_id);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo::utils::window();
    let resp = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    let room: Room = serde_wasm_bindgen::from_value(json).map_err(|e| FetchError {
        err: JsValue::from_str(e.to_string().as_str()),
    })?;
    Ok(room)
}

pub async fn send_message(
    room_id: &str,
    player_id: &str,
    message: Message,
) -> Result<Room, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    let event = Event::Message {
        player_id: player_id.to_string(),
        room_id: room_id.to_string(),
        message,
    };
    let event = serde_json::to_string(&event).map_err(|e| FetchError {
        err: JsValue::from_str(e.to_string().as_str()),
    })?;
    // console::log_1(&JsValue::from_str(&event));
    opts.body(Some(&JsValue::from_str(&event)));

    let url = format!("{}/rooms/{}/counter/message", ORIGIN, room_id);
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo::utils::window();
    let resp = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    let room: Room = serde_wasm_bindgen::from_value(json).map_err(|e| FetchError {
        err: JsValue::from_str(e.to_string().as_str()),
    })?;
    Ok(room)
}
