defmodule CdfyWeb.PluginController do
  use CdfyWeb, :controller

  alias Cdfy.Repo.Plugins

  action_fallback CdfyWeb.FallbackController

  def index(conn, _params) do
    user_id = conn.assigns.current_user.id
    plugins = Plugins.list_plugins_by_user(user_id)
    render(conn, :index, plugins: plugins)
  end

  def show(conn, %{"id" => id}) do
    plugin = Plugins.get_plugin!(id)
    render(conn, :show, plugin: plugin)
  end
end
