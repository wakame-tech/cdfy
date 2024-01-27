defmodule CdfyRoomServerWeb.RoomLive do
  use CdfyRoomServerWeb, :live_view
  require Logger

  alias CdfyRoomServer.Room
  alias Phoenix.PubSub

  @impl true
  def mount(%{"room_id" => room_id}, _session, socket) do
    Room.start(room_id)
    player_id = socket.id

    if connected?(socket) do
      PubSub.subscribe(CdfyRoomServer.PubSub, "room:#{room_id}")
      Room.monitor(room_id, player_id)
    end

    socket =
      socket
      |> assign(:room_id, room_id)
      |> assign(:player_id, player_id)
      |> assign(:version, 0)

    {:ok, socket}
  end

  @impl true
  def handle_event("load_game", _params, %{assigns: %{room_id: room_id}} = socket) do
    :ok = Room.load_game(room_id)
    Room.broadcast_game_state(room_id, socket.assigns.version + 1)
    {:noreply, socket}
  end

  @impl true
  def handle_event(message, params, %{assigns: %{room_id: room_id, player_id: player_id}} = socket) do
    event =
      %{
        player_id: player_id,
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
  def render(%{room_id: room_id, player_id: player_id} = assigns) do
    plugin_state = Room.get_plugin_state(room_id)
    v = Room.render(room_id)
    html = Phoenix.HTML.raw(v)

    ~H"""
    <p> player_id: <%= player_id %></p>
    <button phx-click="load_game">Load Game</button>
    <%= html %>

    <!-- <p>Plugin State: <%= inspect(plugin_state) %></p> -->
    """
  end
end
