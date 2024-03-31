defmodule CdfyWeb.RoomLive do
  alias Cdfy.Plugin.State
  use CdfyWeb, :live_view
  require Logger

  alias Cdfy.Room
  alias Phoenix.PubSub

  defp notify(%{assigns: %{room_id: room_id, version: version}} = socket) do
    PubSub.broadcast(Cdfy.PubSub, "room:#{room_id}", %{room_version: version + 1})
    socket
  end

  @impl true
  def handle_info(
        %{room_version: version},
        socket
      ) do
    {:noreply, assign(socket, version: version)}
  end

  defp notify_state(%{assigns: %{room_id: room_id, states: states}} = socket, state_id) do
    %{version: version} = states[state_id]

    PubSub.broadcast(Cdfy.PubSub, "room:#{room_id}:#{state_id}", %{
      state_id: state_id,
      state_version: version + 1
    })

    socket
  end

  @impl true
  def handle_info(
        %{state_id: state_id, state_version: version},
        %{assigns: %{states: states}} = socket
      ) do
    states = Map.put(states, state_id, states[state_id] |> State.set_version(version))

    {:noreply, assign(socket, states: states)}
  end

  @impl true
  def mount(%{"room_id" => room_id}, _session, socket) do
    player_id = socket.id

    if Room.exists?(room_id) do
      if connected?(socket) do
        PubSub.subscribe(Cdfy.PubSub, "room:#{room_id}")
        Room.monitor(room_id, player_id)
      end

      state_ids = Room.get_state(room_id).plugins |> Map.keys()

      Enum.each(state_ids, fn state_id ->
        PubSub.subscribe(Cdfy.PubSub, "room:#{room_id}:#{state_id}")
      end)

      socket =
        socket
        |> assign(:version, 0)
        |> assign(:room_id, room_id)
        |> assign(:player_id, player_id)
        |> assign(:states, Map.new(state_ids, fn state_id -> {state_id, State.new()} end))

      {:ok, socket}
    else
      {:ok, push_redirect(socket, to: "/")}
    end
  end

  @impl true
  def handle_event(
        "load_plugin",
        %{"plugin_id" => plugin_id},
        %{assigns: %{room_id: room_id, states: states}} = socket
      ) do
    {:ok, state_id} =
      Room.load_plugin(room_id, plugin_id)

    states =
      Map.put(states, state_id, State.new())

    PubSub.subscribe(Cdfy.PubSub, "room:#{room_id}:#{state_id}")

    {:noreply, assign(socket, :states, states) |> notify()}
  end

  @impl true
  def handle_event(
        "toggle_debug",
        %{"state_id" => state_id},
        %{assigns: %{states: states}} = socket
      ) do
    states = Map.put(states, state_id, states[state_id] |> State.toggle_debug())
    {:noreply, socket |> assign(:states, states) |> notify_state(state_id)}
  end

  @impl true
  def handle_event(
        "unload",
        %{"state_id" => state_id},
        %{assigns: %{room_id: room_id, states: states}} = socket
      ) do
    :ok = Room.unload_plugin(room_id, state_id)
    PubSub.unsubscribe(Cdfy.PubSub, "room:#{room_id}:#{state_id}")
    {:noreply, assign(socket, :states, Map.delete(states, state_id)) |> notify()}
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

    {:noreply, socket |> notify_state(state_id)}
  end

  @impl true
  def handle_event(
        event_name,
        value,
        %{assigns: %{room_id: room_id, player_id: player_id, states: states}} =
          socket
      ) do
    Logger.info("event: #{event_name} #{inspect(value)}")

    states =
      Enum.map(states, fn {state_id, state} ->
        {state_id, State.dispatch_event(state, room_id, state_id, player_id, event_name, value)}
      end)
      |> Map.new()

    # Enum.each(states, fn {state_id, _} -> notify_state(socket, state_id) end)

    {:noreply, socket |> assign(:states, states)}
  end

  @impl true
  def render(assigns) do
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

    <%= for {state_id, state} <- @states do %>
      <.live_component
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
