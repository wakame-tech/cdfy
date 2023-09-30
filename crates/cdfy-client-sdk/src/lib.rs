use anyhow::Result;
use cdfy_sdk_core::Event;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

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
    pub room_id: String,
    pub users: HashSet<String>,
    pub states: HashMap<String, String>,
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Api {
    pub origin: String,
}

impl Api {
    async fn get_json<T>(url: String) -> Result<T, FetchError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(&url, &opts)?;

        let window = gloo::utils::window();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();

        let json = JsFuture::from(resp.json()?).await?;
        serde_wasm_bindgen::from_value(json).map_err(|e| FetchError {
            err: JsValue::from_str(e.to_string().as_str()),
        })
    }

    async fn post_json<Res>(url: String) -> Result<Res, FetchError>
    where
        Res: for<'de> Deserialize<'de>,
    {
        Self::post_json_with_body(url, Option::<()>::None).await
    }

    async fn post_json_with_body<Req, Res>(
        url: String,
        body: Option<Req>,
    ) -> Result<Res, FetchError>
    where
        Req: Serialize,
        Res: for<'de> Deserialize<'de>,
    {
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);

        let body = body
            .map(|body| serde_json::to_string(&body).unwrap())
            .map(|body| JsValue::from_str(&body));
        opts.body(body.as_ref());

        let request = Request::new_with_str_and_init(&url, &opts)?;

        let window = gloo::utils::window();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();

        let json = JsFuture::from(resp.json()?).await?;
        serde_wasm_bindgen::from_value(json).map_err(|e| FetchError {
            err: JsValue::from_str(e.to_string().as_str()),
        })
    }

    pub async fn fetch_room(&self, room_id: &str) -> Result<Room, FetchError> {
        let url = format!("{}/rooms/{}", self.origin, room_id);
        Self::get_json(url).await
    }

    pub async fn join_room(&self, room_id: &str, user_id: &str) -> Result<Room, FetchError> {
        let url = format!("{}/rooms/{}/join/{}", self.origin, room_id, user_id);
        Self::post_json(url).await
    }

    pub async fn create_room(&self, room_id: &str) -> Result<Room, FetchError> {
        let url = format!("{}/rooms/{}", self.origin, room_id);
        Self::post_json(url).await
    }

    pub async fn load_plugin(&self, room_id: &str) -> Result<Room, FetchError> {
        let url = format!("{}/rooms/{}/plugins/counter", self.origin, room_id);
        Self::post_json(url).await
    }

    pub async fn send_message<M: Serialize>(
        &self,
        room_id: &str,
        player_id: &str,
        message: M,
    ) -> Result<Room, FetchError> {
        let event = Event::Message {
            player_id: player_id.to_string(),
            room_id: room_id.to_string(),
            message: serde_json::to_string(&message).unwrap(),
        };
        let url = format!("{}/rooms/{}/plugins/counter/message", self.origin, room_id);
        Self::post_json_with_body(url, Some(event)).await
    }
}
