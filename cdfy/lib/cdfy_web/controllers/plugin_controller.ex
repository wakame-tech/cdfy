defmodule CdfyWeb.PluginController do
  use CdfyWeb, :controller

  alias Cdfy.Plugins

  action_fallback CdfyWeb.FallbackController

  def index(conn, _params) do
    plugins = Plugins.list_plugins()
    render(conn, :index, plugins: plugins)
  end

  def show(conn, %{"id" => id}) do
    plugin = Plugins.get_plugin!(id)
    render(conn, :show, plugin: plugin)
  end
end
