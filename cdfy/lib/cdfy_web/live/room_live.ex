defmodule CdfyWeb.RoomLive do
  use CdfyWeb, :live_view
  require Logger

  alias Cdfy.Room
  alias Phoenix.PubSub
  alias Cdfy.Plugin.State

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
        |> assign(:player_name, String.slice(player_id, 0..6))
        |> assign(:state, State.new())

      {:ok, socket}
    else
      {:ok, push_redirect(socket, to: "/")}
    end
  end

  defp notify(%{assigns: %{state: state, room_id: room_id}} = socket) do
    Room.broadcast(room_id, state.version + 1)
    socket
  end

  @impl true
  def handle_event(
        "load_or_finish_game",
        _params,
        %{assigns: %{room_id: room_id}} = socket
      ) do
    %{phase: phase} = Room.get_state(room_id)

    case phase do
      :waiting -> Room.load_game(room_id)
      :ingame -> Room.finish_game(room_id)
    end

    {:noreply, socket |> notify()}
  end

  @impl true
  def handle_event("toggle_debug", _params, %{assigns: %{state: state}} = socket) do
    {:noreply, socket |> assign(:state, State.toggle_debug(state))}
  end

  @impl true
  def handle_event(
        "refresh_plugin",
        _params,
        %{assigns: %{room_id: room_id}} = socket
      ) do
    Room.refresh_plugin(room_id)
    {:noreply, socket |> notify()}
  end

  @impl true
  def handle_event(
        event_name,
        value,
        %{assigns: %{room_id: room_id, player_id: player_id, state: state}} = socket
      ) do
    state = State.dispatch_event(state, room_id, player_id, event_name, value)
    {:noreply, socket |> assign(:state, state) |> notify()}
  end

  @impl true
  def handle_info({:version, version}, %{assigns: %{state: state}} = socket) do
    {:noreply, assign(socket, state: State.set_version(state, version))}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <p>player_id: <%= @player_id %></p>

    <.live_component
      module={CdfyWeb.PluginViewComponent}
      id="plugin_view_component"
      room_id={@room_id}
      player_id={@player_id}
      state={@state}
    />
    """
  end
end
