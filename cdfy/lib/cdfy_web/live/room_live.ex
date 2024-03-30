defmodule CdfyWeb.RoomLive do
  use CdfyWeb, :live_view
  require Logger

  alias Cdfy.Room
  alias Phoenix.PubSub

  @impl true
  def mount(%{"room_id" => room_id}, _session, socket) do
    player_id = socket.id

    if Room.exists?(room_id) do
      if connected?(socket) do
        PubSub.subscribe(Cdfy.PubSub, "room:#{room_id}")
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
    else
      {:ok, push_redirect(socket, to: "/")}
    end
  end

  @impl true
  def handle_event("load_or_finish_game", _params, %{assigns: %{room_id: room_id}} = socket) do
    %{phase: phase} = Room.state(room_id)

    case phase do
      :waiting -> Room.load_game(room_id)
      :ingame -> Room.finish_game(room_id)
    end

    Room.broadcast_game_state(room_id, socket.assigns.version + 1)
    {:noreply, socket}
  end

  @impl true
  def handle_event("toggle_debug", _params, socket) do
    {:noreply, assign(socket, debug: not socket.assigns.debug)}
  end

  @impl true
  def handle_event(
        "refresh_plugin",
        _params,
        %{assigns: %{room_id: room_id, version: version}} = socket
      ) do
    Room.refresh_plugin(room_id)
    Room.broadcast_game_state(room_id, version + 1)
    {:noreply, socket}
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

    html = Room.render(room_id)
    %{phase: phase} = Room.state(room_id)
    error = Map.get(e, player_id)

    ~H"""
    <button class="p-2 bg-red-500 text-white font-bold rounded" phx-click="load_or_finish_game">
      <%= if phase == :waiting do %>
        start
      <% else %>
        finish
      <% end %>
    </button>
    <p>player_id: <%= player_id %></p>
    <%= if error != nil do %>
      <p class="text-red-500">error: <%= error %></p>
    <% end %>

    <%= raw(html) %>

    <input id="debug" type="checkbox" phx-click="toggle_debug" checked={debug} />
    <label for="debug">debug</label>
    <%= if debug do %>
      <button class="p-2 bg-red-500 text-white font-bold rounded" phx-click="refresh_plugin">
        refresh_plugin
      </button>
      <p><%= inspect(state) %></p>
    <% end %>
    """
  end
end
