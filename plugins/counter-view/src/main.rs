use api::create_room;
use api::fetch_room;
use api::join_room;
use api::load_plugin;
use api::send_message;
use api::FetchState;
use api::Message;
use api::Room;
use yew::prelude::*;

pub mod api;
enum Msg {
    GetRoom,
    CreateRoom,
    LoadPlugin,
    JoinRoom,
    SetRoomState(FetchState<Room>),
    Message(Message),
}

struct App {
    room: FetchState<Room>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            room: FetchState::NotFetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetRoomState(state) => {
                self.room = state;
                true
            }
            Msg::GetRoom => {
                ctx.link().send_future(async {
                    match fetch_room("a").await {
                        Ok(room) => Msg::SetRoomState(FetchState::Success(room)),
                        Err(err) => Msg::SetRoomState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetRoomState(FetchState::Fetching));
                true
            }
            Msg::CreateRoom => {
                ctx.link().send_future(async {
                    match create_room("a").await {
                        Ok(room) => Msg::SetRoomState(FetchState::Success(room)),
                        Err(err) => Msg::SetRoomState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetRoomState(FetchState::Fetching));
                true
            }
            Msg::LoadPlugin => {
                ctx.link().send_future(async {
                    match load_plugin("a").await {
                        Ok(room) => Msg::SetRoomState(FetchState::Success(room)),
                        Err(err) => Msg::SetRoomState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetRoomState(FetchState::Fetching));
                true
            }
            Msg::JoinRoom => {
                ctx.link().send_future(async {
                    match join_room("a", "u").await {
                        Ok(room) => Msg::SetRoomState(FetchState::Success(room)),
                        Err(err) => Msg::SetRoomState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetRoomState(FetchState::Fetching));
                true
            }
            Msg::Message(message) => {
                ctx.link().send_future(async {
                    match send_message("a", "u", message).await {
                        Ok(room) => Msg::SetRoomState(FetchState::Success(room)),
                        Err(err) => Msg::SetRoomState(FetchState::Failed(err)),
                    }
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <button onclick={ctx.link().callback(|_| Msg::GetRoom)}>
                    { "get_room" }
                </button>
                <button onclick={ctx.link().callback(|_| Msg::CreateRoom)}>
                    { "create_room a" }
                </button>
                <button onclick={ctx.link().callback(|_| Msg::LoadPlugin)}>
                    { "load plugin counter in a" }
                </button>
                <button onclick={ctx.link().callback(|_| Msg::JoinRoom)}>
                    { "join_room a as u" }
                </button>

                <button onclick={ctx.link().callback(|_| Msg::Message(Message::Increment))}>
                    { "increment" }
                </button>

                <button onclick={ctx.link().callback(|_| Msg::Message(Message::WillIncrement))}>
                    { "will_increment" }
                </button>

                if let FetchState::Success(room) = &self.room {
                    <p>{ room }</p>
                }
                if let FetchState::Failed(err) = &self.room {
                    <p>{ err }</p>
                }
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
