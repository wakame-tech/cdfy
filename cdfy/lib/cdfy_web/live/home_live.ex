defmodule CdfyWeb.HomeLive do
  use CdfyWeb, :live_view
  require Logger
  alias Cdfy.Room

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
    title = "cdfy_plugin_janken"

    case Room.start(room_id: room_id, plugin_title: title) do
      {:ok, :initiated} -> {:noreply, socket}
      {:error, :already_exists} -> {:noreply, socket}
    end
  end

  @impl true
  def render(assigns) do
    ~H"""
    <button class="p-2 bg-blue-700 text-white font-bold rounded" phx-click="create_room">
      Create Room
    </button>

    <h1 class="text-2xl font-bold">Rooms</h1>

    <%= for {room_id, player_ids, phase, _} <- @rooms do %>
      <div class="py-2">
        <h2 class="font-bold text-xl">
          <a class="underline" href={~p"/rooms/#{room_id}"}>
            <%= room_id %>
          </a>
        </h2>
        <span class="text-gray-500"><%= Enum.count(player_ids) %> players</span>
        <span class="text-orange-500"><%= phase %></span>
      </div>
    <% end %>
    """
  end
end
