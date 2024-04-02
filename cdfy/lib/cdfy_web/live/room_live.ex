defmodule CdfyWeb.RoomLive do
  alias Phoenix.PubSub
  alias Cdfy.PluginServer
  alias Cdfy.RoomServer
  use CdfyWeb, :live_view
  require Logger

  @impl true
  def mount(%{"room_id" => room_id}, _session, socket) do
    player_id = socket.id

    if RoomServer.exists?(room_id) do
      if connected?(socket) do
        PubSub.subscribe(Cdfy.PubSub, "room:#{room_id}")
        RoomServer.monitor(room_id, player_id)
      end

      socket =
        socket
        |> assign(:version, 0)
        |> assign(:room_id, room_id)
        |> assign(:player_id, player_id)
        |> assign(:state_ids, RoomServer.get_state_ids(room_id))

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
    state_ids = RoomServer.get_state_ids(room_id)

    socket =
      socket
      |> assign(:version, version)
      |> assign(:state_ids, state_ids)

    {:noreply, socket}
  end

  @impl true
  def handle_event(
        "load_plugin",
        %{"plugin_id" => plugin_id},
        %{assigns: %{room_id: room_id}} = socket
      ) do
    state_id = Ecto.UUID.generate()
    :ok = RoomServer.add_plugin(room_id, plugin_id, state_id)
    {:noreply, socket |> notify()}
  end

  @impl true
  def handle_event(
        "toggle_debug",
        %{"state_id" => state_id},
        socket
      ) do
    :ok = PluginServer.toggle_debug(state_id)
    {:noreply, socket |> notify()}
  end

  @impl true
  def handle_event(
        "unload",
        %{"state_id" => state_id},
        %{assigns: %{room_id: room_id}} = socket
      ) do
    :ok = RoomServer.unload_plugin(room_id, state_id)
    {:noreply, socket |> notify()}
  end

  @impl true
  def handle_event(
        "init_or_finish_game",
        %{"state_id" => state_id},
        %{assigns: %{room_id: room_id}} = socket
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
        event_name,
        value,
        %{assigns: %{room_id: room_id, player_id: player_id}} =
          socket
      ) do
    ev =
      %{
        room_id: room_id,
        player_id: player_id,
        event_name: event_name,
        value: value
      }

    :ok = RoomServer.dispatch_event_all(room_id, ev)
    {:noreply, socket |> notify()}
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

    <%= for state_id <- @state_ids do %>
      <.live_component
        version={@version}
        module={CdfyWeb.PluginViewComponent}
        id={state_id}
        player_id={@player_id}
        state_id={state_id}
      />
    <% end %>
    """
  end
end
