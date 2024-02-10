defmodule CdfyRoomServerWeb.HomeLive do
  alias CdfyRoomServer.Plugin
  use CdfyRoomServerWeb, :live_view
  require Logger
  alias CdfyRoomServer.Room

  @impl true
  def mount(_params, _session, socket) do
    rooms =
      Room.room_states()
      |> Enum.map(fn state ->
        {state[:room_id], state[:player_ids], state[:phase], Enum.count(state[:pids])}
      end)

    socket = assign(socket, :rooms, rooms)
    {:ok, socket}
  end

  @impl true
  def handle_event("create_room", _params, socket) do
    room_id = Ecto.UUID.generate()

    plugin = %Plugin{
      version: "0.0.1",
      title: "cdfy_career_poker_plugin",
      url:
        "https://github.com/wakame-tech/cdfy_career_poker_plugin/releases/download/v0.0.1/cdfy_career_poker_plugin.wasm"
    }

    case Room.start(room_id: room_id, plugin: plugin) do
      {:ok, :initiated} -> {:noreply, socket}
      {:error, :already_exists} -> {:noreply, socket}
    end
  end

  @impl true
  def render(assigns) do
    ~H"""
    <button class="p-2 bg-blue-700 text-white rounded" phx-click="create_room">Create Room</button>

    <div class="room-list">
      <h2>Rooms</h2>
      <%= for {room_id, player_ids, phase, pids} <- @rooms do %>
        <div class="room">
          <.link href={~p"/rooms/#{room_id}"}>
            <%= room_id %>
          </.link>
          <span><%= phase %></span>
          <span><%= Enum.count(player_ids) %> players</span>
          <span><%= pids %> pids</span>
        </div>
      <% end %>
    </div>
    """
  end
end
