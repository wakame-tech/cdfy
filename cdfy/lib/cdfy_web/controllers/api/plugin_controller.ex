defmodule CdfyWeb.Api.PluginController do
  use CdfyWeb, :controller

  alias Cdfy.Plugins
  alias Cdfy.Plugin

  action_fallback CdfyWeb.FallbackController

  def index(conn, _params) do
    plugins = Plugins.list_plugins()
    render(conn, :index, plugins: plugins)
  end

  def create(conn, %{"plugin" => plugin_params}) do
    with {:ok, %Plugin{} = plugin} <- Plugins.create_plugin(plugin_params) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", ~p"/api/plugins/#{plugin}")
      |> render(:show, plugin: plugin)
    end
  end

  def show(conn, %{"id" => id}) do
    plugin = Plugins.get_plugin!(id)
    render(conn, :show, plugin: plugin)
  end

  def update(conn, %{"id" => id, "plugin" => plugin_params}) do
    plugin = Plugins.get_plugin!(id)

    with {:ok, %Plugin{} = plugin} <- Plugins.update_plugin(plugin, plugin_params) do
      render(conn, :show, plugin: plugin)
    end
  end

  def delete(conn, %{"id" => id}) do
    plugin = Plugins.get_plugin!(id)

    with {:ok, %Plugin{}} <- Plugins.delete_plugin(plugin) do
      send_resp(conn, :no_content, "")
    end
  end
end
