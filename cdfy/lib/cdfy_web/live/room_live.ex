defmodule CdfyWeb.RoomLive do
  use CdfyWeb, :live_view

  alias Phoenix.PubSub
  alias Cdfy.RoomServer
  require Logger

  defp refresh(%{assigns: %{room_id: room_id}} = socket) do
    state_ids = RoomServer.get_state_ids(room_id)
    player_ids = RoomServer.get_player_ids(room_id)

    socket |> assign(state_ids: state_ids, player_ids: player_ids)
  end

  @impl true
  def mount(%{"room_id" => room_id}, _session, socket) do
    player_id = socket.id
    PubSub.subscribe(Cdfy.PubSub, "room:#{room_id}")

    if RoomServer.exists?(room_id) do
      if connected?(socket) do
        RoomServer.monitor(room_id, player_id)
      end

      socket =
        socket
        |> assign(:version, 0)
        |> assign(:room_id, room_id)
        |> assign(:player_id, player_id)

      {:ok, socket |> refresh() |> notify()}
    else
      {:ok, push_redirect(socket, to: "/")}
    end
  end

  defp notify(%{assigns: %{room_id: room_id, version: version}} = socket) do
    PubSub.broadcast(Cdfy.PubSub, "room:#{room_id}", %{version: version + 1})
    socket
  end

  @impl true
  def handle_info(:refresh, socket), do: {:noreply, socket |> notify()}

  @impl true
  def handle_info(%{version: version}, socket) do
    socket =
      socket
      |> assign(:version, version)

    {:noreply, socket |> refresh()}
  end

  @impl true
  def handle_event(
        "load_plugin",
        _value,
        %{assigns: %{room_id: room_id}} = socket
      ) do
    state_id = Ecto.UUID.generate()
    plugin_id = "c88bfaf4-654a-425a-a39b-e935cd74380c"
    :ok = RoomServer.add_plugin(room_id, plugin_id, state_id)
    {:noreply, socket |> notify()}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <p>version: <%= @version %></p>
    <p>player_id: <%= @player_id %></p>
    <p>player_ids: <%= inspect(@player_ids) %></p>

    <button phx-click="load_plugin">Add</button>

    <%= for state_id <- @state_ids do %>
      <%= live_render(@socket, CdfyWeb.PluginLive,
        router: CdfyWeb.Router,
        id: state_id,
        session: %{
          "room_id" => @room_id,
          "player_id" => @player_id,
          "state_id" => state_id
        }
      ) %>
    <% end %>
    """
  end
end
