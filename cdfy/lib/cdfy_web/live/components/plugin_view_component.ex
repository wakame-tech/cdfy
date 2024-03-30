defmodule CdfyWeb.PluginViewComponent do
  use Phoenix.LiveComponent
  import Phoenix.HTML
  alias Cdfy.Room

  @impl true
  def render(%{room_id: room_id, player_id: player_id, state: state} = assigns) do
    {:ok, plugin_state} =
      case state.debug do
        true -> Room.get_plugin_state(room_id)
        false -> {:ok, nil}
      end

    html = Room.render(room_id)
    %{phase: phase} = Room.get_state(room_id)
    error = Map.get(state.errors, player_id)

    ~H"""
    <div>
      <button
        class="px-2 py-1 bg-red-500 text-white font-bold rounded"
        phx-click="load_or_finish_game"
      >
        <%= if phase == :waiting do %>
          start
        <% else %>
          finish
        <% end %>
      </button>
      <%= if error != nil do %>
        <p class="text-red-500">error: <%= error %></p>
      <% end %>

      <%= raw(html) %>

      <input id="debug" type="checkbox" phx-click="toggle_debug" checked={state.debug} />
      <label for="debug">debug</label>
      <%= if state.debug do %>
        <button class="p-2 bg-red-500 text-white font-bold rounded" phx-click="refresh_plugin">
          refresh_plugin
        </button>
        <p><%= inspect(plugin_state) %></p>
      <% end %>
    </div>
    """
  end
end
