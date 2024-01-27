defmodule CdfyRoomServerWeb.RoomLive do
  use CdfyRoomServerWeb, :live_view
  require Logger

  alias CdfyRoomServer.Room
  alias Phoenix.PubSub

  @impl true
  def mount(%{"room_id" => room_id}, _session, socket) do
    if connected?(socket) do
      Room.start(room_id)
      PubSub.subscribe(CdfyRoomServer.PubSub, "room:#{room_id}")
      Room.monitor(room_id)
    end

    socket =
      socket
      |> assign(:room_id, room_id)
      |> assign(:version, 0)

    {:ok, socket}
  end

  @impl true
  def handle_event("load_game", _params, %{assigns: %{room_id: room_id}} = socket) do
    :ok = Room.load_game(room_id)
    version = socket.assigns.version + 1
    Room.broadcast_game_state(room_id, socket.assigns.version + 1)
    {:noreply, socket}
  end

  @impl true
  def handle_event(message, params, %{assigns: %{room_id: room_id}} = socket) do
    event =
      %{
        player_id: "player",
        event_name: message,
        value: params
      }

    Room.new_event(room_id, event)
    Room.broadcast_game_state(room_id, socket.assigns.version + 1)
    {:noreply, socket}
  end

  @impl true
  def handle_info({:version, version}, socket) do
    {:noreply, assign(socket, version: version)}
  end

  @impl true
  def render(%{room_id: room_id} = assigns) do
    v = Room.render(room_id)
    html = Phoenix.HTML.raw(v)

    ~H"""
    <button phx-click="load_game">Load Game</button>
    <%= html %>
    """
  end
end
