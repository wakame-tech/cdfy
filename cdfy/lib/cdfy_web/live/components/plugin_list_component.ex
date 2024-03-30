defmodule CdfyWeb.PluginListComponent do
  use Phoenix.LiveComponent

  def render(assigns) do
    ~H"""
    <div>
      <%= for plugin <- @plugins do %>
        <div class="py-2 flex">
          <p><%= plugin.title %> <span class="text-gray-400">v<%= plugin.version %></span></p>
          <button
            class="ml-2 px-2 bg-blue-700 text-white font-bold rounded"
            phx-click="select_plugin"
            phx-value-id={plugin.id}
          >
            use
          </button>
        </div>
      <% end %>
    </div>
    """
  end
end
