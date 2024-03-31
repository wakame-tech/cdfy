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
        |> assign(:version, 0)
        |> assign(:room_id, room_id)
        |> assign(:room_state, Room.get_state(room_id))
        |> assign(:player_id, player_id)

      {:ok, socket}
    else
      {:ok, push_redirect(socket, to: "/")}
    end
  end

  defp notify(%{assigns: %{room_id: room_id, version: version}} = socket) do
    PubSub.broadcast(Cdfy.PubSub, "room:#{room_id}", %{version: version + 1})
    socket
  end

  @impl true
  def handle_info(
        %{version: version},
        %{assigns: %{room_id: room_id}} = socket
      ) do
    state = Room.get_state(room_id)
    Logger.info("room_state: #{inspect(state)}")

    socket =
      socket
      |> assign(:version, version)
      |> assign(:room_state, state)

    {:noreply, socket}
  end

  @impl true
  def handle_event(
        "load_plugin",
        %{"plugin_id" => plugin_id},
        %{assigns: %{room_id: room_id}} = socket
      ) do
    {:ok, state_id} =
      Room.load_plugin(room_id, plugin_id)

    PubSub.subscribe(Cdfy.PubSub, "room:#{room_id}:#{state_id}")

    {:noreply, socket |> notify()}
  end

  @impl true
  def handle_event(
        "toggle_debug",
        %{"state_id" => state_id},
        %{assigns: %{room_id: room_id}} = socket
      ) do
    :ok = Room.toggle_debug(room_id, state_id)
    {:noreply, socket |> notify()}
  end

  @impl true
  def handle_event(
        "unload",
        %{"state_id" => state_id},
        %{assigns: %{room_id: room_id}} = socket
      ) do
    :ok = Room.unload_plugin(room_id, state_id)
    PubSub.unsubscribe(Cdfy.PubSub, "room:#{room_id}:#{state_id}")
    {:noreply, socket |> notify()}
  end

  @impl true
  def handle_event(
        "init_or_finish_game",
        %{"state_id" => state_id},
        %{assigns: %{room_id: room_id}} = socket
      ) do
    case Room.get_phase(room_id, state_id) do
      :waiting -> Room.init_game(room_id, state_id)
      :ingame -> Room.finish_game(room_id, state_id)
    end

    {:noreply, socket |> notify()}
  end

  @impl true
  def handle_event(
        event_name,
        value,
        %{assigns: %{room_id: room_id, player_id: player_id}} =
          socket
      ) do
    event =
      %{
        player_id: player_id,
        event_name: event_name,
        value: value
      }

    :ok = Room.dispatch_event(room_id, player_id, event)

    {:noreply, socket |> notify()}
  end

  @impl true
  def render(assigns) do
    states = assigns.room_state.states

    ~H"""
    <p>version: <%= @version %></p>
    <p>player_id: <%= @player_id %></p>

    <button
      class="px-2 py-1 bg-red-500 text-white font-bold rounded"
      phx-click="load_plugin"
      phx-value-plugin_id="902d2c13-2a36-4f24-bd0c-99e105111545"
    >
      add
    </button>

    <%= for {state_id, state} <- states do %>
      <.live_component
        version={@version}
        module={CdfyWeb.PluginViewComponent}
        id={state_id}
        state_id={state_id}
        room_id={@room_id}
        player_id={@player_id}
        state={state}
      />
    <% end %>
    """
  end
end
