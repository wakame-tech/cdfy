defmodule CdfyWeb.RoomLive do
  use CdfyWeb, :live_view

  alias Phoenix.PubSub
  alias Cdfy.RoomServer
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
        |> assign(:player_ids, RoomServer.get_player_ids(room_id))

      {:ok, socket |> notify()}
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
  def render(assigns) do
    ~H"""
    <p>player_id: <%= @player_id %></p>
    <p>player_ids: <%= inspect(@player_ids) %></p>

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
