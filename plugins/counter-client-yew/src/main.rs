use cdfy_client_sdk::{Api, FetchError, Room};
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};
use yew::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    WillIncrement,
    Increment,
}

enum ClientMessage {
    GetRoom,
    CreateRoom,
    LoadPlugin,
    JoinRoom,
    UpdateRoom(Result<Room, FetchError>),
    Action(Action),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CounterState {
    tasks: VecDeque<String>,
    player_ids: HashSet<String>,
    count: usize,
}

#[derive(Debug, PartialEq, Properties)]
struct CounterComponentProps {
    state: CounterState,
    on_action: Callback<Action>,
}

struct CounterComponent;

impl Component for CounterComponent {
    type Message = Action;
    type Properties = CounterComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <button onclick={ctx.props().on_action.reform(|_| Action::Increment)}>
            { "increment" }
        </button>

        <button onclick={ctx.props().on_action.reform(|_| Action::WillIncrement)}>
            { "will_increment" }
        </button>

            <p>{ "count:" } { &ctx.props().state.count }</p>
        </div>
        }
    }
}

struct App {
    room_id: String,
    user_id: String,
    api: Api,
    room: Option<Room>,
    error: Option<String>,
}

impl Component for App {
    type Message = ClientMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            room_id: "a".to_string(),
            user_id: "u".to_string(),
            api: Api {
                origin: "http://localhost:1234".to_string(),
            },
            room: None,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let api = self.api.clone();
        let room_id = self.room_id.clone();
        let user_id = self.user_id.clone();

        match msg {
            ClientMessage::UpdateRoom(result) => {
                match result {
                    Ok(room) => self.room = Some(room),
                    Err(e) => self.error = Some(e.to_string()),
                }
                true
            }
            ClientMessage::GetRoom => {
                ctx.link().send_future(async move {
                    ClientMessage::UpdateRoom(api.fetch_room(&room_id).await)
                });
                true
            }
            ClientMessage::CreateRoom => {
                ctx.link().send_future(async move {
                    ClientMessage::UpdateRoom(api.create_room(&room_id).await)
                });
                true
            }
            ClientMessage::LoadPlugin => {
                ctx.link().send_future(async move {
                    ClientMessage::UpdateRoom(api.load_plugin(&room_id).await)
                });
                true
            }
            ClientMessage::JoinRoom => {
                ctx.link().send_future(async move {
                    ClientMessage::UpdateRoom(api.join_room(&room_id, &user_id).await)
                });
                true
            }
            ClientMessage::Action(message) => {
                ctx.link().send_future(async move {
                    ClientMessage::UpdateRoom(api.send_message(&room_id, &user_id, message).await)
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = self
            .room
            .as_ref()
            .and_then(|r| r.states.get("counter"))
            .and_then(|state| serde_json::from_str::<CounterState>(state).ok());
        let on_action = ctx.link().callback(|action| ClientMessage::Action(action));

        html! {
            <>
                <button onclick={ctx.link().callback(|_| ClientMessage::GetRoom)}>
                    { "get_room" }
                </button>
                <button onclick={ctx.link().callback(|_| ClientMessage::CreateRoom)}>
                    { "create_room user:" } { &self.user_id }
                </button>
                <button onclick={ctx.link().callback(|_| ClientMessage::LoadPlugin)}>
                    { "load plugin counter" }
                </button>
                <button onclick={ctx.link().callback(|_| ClientMessage::JoinRoom)}>
                    { "join_room:" } { &self.room_id } { " user:" } { &self.user_id }
                </button>

                <hr/>

                if let Some(state) = state {
                    <CounterComponent {state} {on_action} />
                }

                if let Some(err) = &self.error {
                    <p>{ "err" } { err }</p>
                }
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
