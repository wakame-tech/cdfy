defmodule CdfyWeb.PluginViewComponent do
  use Phoenix.LiveComponent
  require Logger
  import Phoenix.HTML
  alias Cdfy.PluginServer

  @impl true
  def render(%{player_id: player_id, state_id: state_id} = assigns) do
    plugin_state =
      case PluginServer.get_plugin_state(state_id) do
        {:ok, state} -> state
        _ -> nil
      end

    # TODO: cannot distinct state_id because RoomLive.handle_event/3 is called when button in html is clicked
    html = PluginServer.render(state_id, player_id)

    %{debug: debug, errors: errors, phase: phase} =
      PluginServer.get_state(state_id)

    assigns =
      assigns
      |> assign(
        debug: debug,
        error: Map.get(errors, player_id),
        phase: phase,
        html: html,
        plugin_state: plugin_state
      )

    ~H"""
    <div class="p-2 border border-2 rounded">
      <p>state_id: <%= @state_id %></p>
      <button
        class="px-2 py-1 bg-red-500 text-white font-bold rounded"
        id={@state_id <> "_init_or_finish_game"}
        phx-click="init_or_finish_game"
        phx-value-state_id={@state_id}
      >
        <%= if @phase == :waiting do %>
          start
        <% else %>
          finish
        <% end %>
      </button>
      <%= if @error != nil do %>
        <p class="text-red-500">error: <%= @error %></p>
      <% end %>

      <%= raw(@html) %>

      <input
        id={@state_id <> "_debug"}
        type="checkbox"
        phx-click="toggle_debug"
        phx-value-state_id={@state_id}
        checked={@debug}
      />
      <label for="debug">debug</label>
      <%= if @debug do %>
        <button
          class="px-2 bg-red-500 text-white font-bold rounded"
          phx-click="unload"
          phx-value-state_id={@state_id}
        >
          unload
        </button>
        <p><%= inspect(@plugin_state) %></p>
      <% end %>
    </div>
    """
  end
end
