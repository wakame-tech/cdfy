defmodule CdfyRoomServerWeb.GameLive do
  use CdfyRoomServerWeb, :live_view
  require Logger

  @wasm_path "plugins/template.wasm"

  @impl true
  def mount(_, _, socket) do
    {:ok, plugin} =
      Extism.Plugin.new(%{wasm: [%{path: @wasm_path}]}, false)

    game_config = %{player_ids: []}

    {:ok, _res} =
      Extism.Plugin.call(plugin, "init_game", Jason.encode!(game_config))

    socket =
      socket
      |> assign(:plugin, plugin)
      |> assign(:version, 0)

    {:ok, socket}
  end

  @impl true
  def handle_event(message, params, %{assigns: %{plugin: plugin}} = socket) do
    event =
      %{
        player_id: "player",
        event_name: message,
        value: params
      }

    {:ok, _res} =
      Extism.Plugin.call(plugin, "handle_event", Jason.encode!(event))

    socket =
      socket
      |> assign(:plugin, plugin)
      |> assign(:version, socket.assigns.version + 1)

    {:noreply, socket}
  end

  @impl true
  def render(%{plugin: plugin} = assigns) do
    {:ok, html} =
      Extism.Plugin.call(plugin, "render", Jason.encode!(%{}))
      |> IO.inspect()

    ~H"""
    <%= Phoenix.HTML.raw(html) %>
    """
  end
end
