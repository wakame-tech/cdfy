defmodule CdfyRoomServerWeb.RoomLive do
  use CdfyRoomServerWeb, :live_view
  require Logger

  alias CdfyRoomServer.Room
  alias Phoenix.PubSub

  @impl true
  def mount(%{"room_id" => room_id}, _session, socket) do
    player_id = socket.id

    if connected?(socket) do
      PubSub.subscribe(CdfyRoomServer.PubSub, "room:#{room_id}")
      Room.monitor(room_id, player_id)
    end

    socket =
      socket
      |> assign(:room_id, room_id)
      |> assign(:player_id, player_id)
      |> assign(:error, %{})
      |> assign(:version, 0)
      |> assign(:debug, false)

    {:ok, socket}
  end

  @impl true
  def handle_event("load_game", _params, %{assigns: %{room_id: room_id}} = socket) do
    {:ok, _} = Room.load_game(room_id)
    Room.broadcast_game_state(room_id, socket.assigns.version + 1)
    {:noreply, socket}
  end

  @impl true
  def handle_event("toggle_debug", _params, socket) do
    {:noreply, assign(socket, debug: not socket.assigns.debug)}
  end

  @impl true
  def handle_event(
        message,
        params,
        %{assigns: %{room_id: room_id, player_id: player_id, error: error}} = socket
      ) do
    event =
      %{
        player_id: player_id,
        event_name: message,
        value: params
      }

    socket =
      case Room.new_event(room_id, event) do
        {:error, e} ->
          Logger.error("error handling event: #{inspect(e)}")
          assign(socket, error: Map.put(error, player_id, e))

        _ ->
          socket
      end

    Room.broadcast_game_state(room_id, socket.assigns.version + 1)
    {:noreply, socket}
  end

  @impl true
  def handle_info({:version, version}, socket) do
    {:noreply, assign(socket, version: version)}
  end

  @impl true
  def render(%{room_id: room_id, player_id: player_id, error: e, debug: debug} = assigns) do
    {:ok, state} =
      case debug do
        true -> Room.get_plugin_state(room_id)
        false -> {:ok, nil}
      end

    v = Room.render(room_id)
    error = Map.get(e, player_id)
    html = Phoenix.HTML.raw(v)

    ~H"""
    <button class="p-2 bg-red-500 text-white" phx-click="load_game">load</button>
    <p>player_id: <%= player_id %></p>
    <%= if error != nil do %>
      <p class="text-red-500">error: <%= error %></p>
    <% end %>

    <%= html %>

    <input id="debug" type="checkbox" phx-click="toggle_debug" checked={debug} />
    <label for="debug">debug</label>
    <%= if debug do %>
      <p><%= inspect(state) %></p>
    <% end %>
    """
  end
end
