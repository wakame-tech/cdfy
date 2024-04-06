defmodule CdfyWeb.PluginLive do
  use CdfyWeb, :live_view

  require Logger
  import Phoenix.HTML
  alias Phoenix.PubSub
  alias Cdfy.RoomServer
  alias Cdfy.PluginServer
  alias Cdfy.Event

  defp refresh(%{assigns: %{state_id: state_id, player_id: player_id}} = socket) do
    plugin_state =
      case PluginServer.get_plugin_state(state_id) do
        {:ok, state} -> state
        _ -> nil
      end

    html = PluginServer.render(state_id, player_id)

    %{errors: errors, phase: phase} =
      PluginServer.get_state(state_id)

    socket
    |> assign(
      error: Map.get(errors, player_id),
      phase: phase,
      html: html,
      plugin_state: plugin_state
    )
  end

  @impl true
  def mount(
        _params,
        %{"room_id" => room_id, "player_id" => player_id, "state_id" => state_id},
        socket
      ) do
    socket =
      socket
      |> assign(
        version: 0,
        debug: false,
        state_id: state_id,
        room_id: room_id,
        player_id: player_id
      )
      |> refresh()

    if connected?(socket) do
      PubSub.subscribe(Cdfy.PubSub, "plugin:#{state_id}")
    end

    {:ok, socket}
  end

  defp notify(%{assigns: %{state_id: state_id, version: version}} = socket) do
    PubSub.broadcast(Cdfy.PubSub, "plugin:#{state_id}", %{version: version + 1})
    socket
  end

  @impl true
  def handle_info(
        %{version: version},
        socket
      ) do
    {:noreply, socket |> assign(:version, version) |> refresh()}
  end

  @impl true
  def handle_event(
        "init_or_finish_game",
        _params,
        %{assigns: %{room_id: room_id, state_id: state_id}} = socket
      ) do
    case PluginServer.get_state(state_id).phase do
      :waiting ->
        player_ids = RoomServer.get_player_ids(room_id)
        PluginServer.init_game(state_id, player_ids)

      :ingame ->
        PluginServer.finish_game(state_id)
    end

    {:noreply, socket |> notify()}
  end

  @impl true
  def handle_event(
        "toggle_debug",
        _params,
        %{assigns: %{debug: debug}} = socket
      ) do
    {:noreply, socket |> assign(debug: !debug)}
  end

  @impl true
  def handle_event(
        "unload",
        _value,
        %{assigns: %{room_id: room_id, state_id: state_id}} = socket
      ) do
    :ok = RoomServer.unload_plugin(room_id, state_id)
    PubSub.local_broadcast(Cdfy.PubSub, "room:#{room_id}", :refresh)
    {:noreply, socket}
  end

  @impl true
  def handle_event(
        event_name,
        value,
        %{assigns: %{room_id: room_id, state_id: state_id, player_id: player_id}} =
          socket
      ) do
    ev = Event.new(room_id, player_id, event_name, value)
    :ok = PluginServer.dispatch_event(state_id, ev)
    PubSub.local_broadcast(Cdfy.PubSub, "room:#{room_id}", :refresh)
    {:noreply, socket |> notify()}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <div class="my-2 p-2 border border-1 border-gray-400 rounded">
      <p>state_id: <%= @state_id %></p>
      <button
        class="px-2 py-1 bg-red-500 text-white font-bold rounded"
        phx-click="init_or_finish_game"
      >
        <%= if @phase == :waiting do %>
          start
        <% else %>
          finish
        <% end %>
      </button>
      <button class="px-2 py-1 bg-red-500 text-white font-bold rounded" phx-click="unload">
        unload
      </button>

      <%= if @error != nil do %>
        <p class="text-red-500">error: <%= @error %></p>
      <% end %>

      <%= raw(@html) %>

      <input type="checkbox" phx-click="toggle_debug" checked={@debug} />
      <label for="debug">debug</label>
      <%= if @debug do %>
        <p><%= inspect(@plugin_state) %></p>
      <% end %>
    </div>
    """
  end
end
