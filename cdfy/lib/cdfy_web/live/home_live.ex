defmodule CdfyWeb.HomeLive do
  alias Cdfy.RoomSupervisor
  use CdfyWeb, :live_view
  require Logger
  alias Cdfy.RoomServer
  alias Cdfy.Repo.Plugins

  @impl true
  def mount(_params, _session, socket) do
    socket =
      socket
      |> assign(:rooms, RoomSupervisor.states())
      |> assign(:plugins, Plugins.list_plugins())

    {:ok, socket}
  end

  @impl true
  def handle_event("select_plugin", %{"id" => plugin_id}, socket) do
    room_id = Ecto.UUID.generate()
    {:ok, _} = RoomSupervisor.start_child(room_id: room_id)

    state_id = Ecto.UUID.generate()
    :ok = RoomServer.add_plugin(room_id, plugin_id, state_id)

    {:noreply, push_redirect(socket, to: "/rooms/#{room_id}")}
  end

  @impl true
  def render(assigns) do
    ~H"""
    <.live_component
      module={CdfyWeb.PluginListComponent}
      id="plugin_list_component"
      plugins={@plugins}
    />

    <h1 class="text-2xl font-bold py-4">Rooms</h1>
    <%= for room <- @rooms do %>
      <div class="py-2">
        <h2 class="font-bold text-xl">
          <a class="underline" href={~p"/rooms/#{room.room_id}"}>
            <%= room.room_id %>
          </a>
        </h2>
        <span class="text-gray-500"><%= Enum.count(room.player_ids) %> players</span>
      </div>
    <% end %>
    """
  end
end
