defmodule CdfyWeb.PluginController do
  use CdfyWeb, :controller
  import Phoenix.HTML.Form
  alias Cdfy.Repo
  alias Cdfy.Repo.Plugins
  alias Cdfy.Model.Plugin
  alias Cdfy.Storage

  action_fallback(CdfyWeb.FallbackController)

  def index(conn, _params) do
    user_id = conn.assigns.current_user.id
    plugins = Plugins.list_plugins_by_user(user_id)
    render(conn, :index, plugins: plugins)
  end

  def new(conn, _params) do
    changeset = Plugin.changeset(%Plugin{}, %{})
    render(conn, :new, changeset: changeset)
  end

  def create(conn, %{"plugin" => params}) do
    case IO.inspect(params) do
      %{
        "title" => title,
        "url" => url,
        "version" => version,
        "wasm" => %{path: path, content_type: "application/wasm", filename: filename}
      } ->
        IO.inspect({filename, path})
        {:ok, res} = Storage.upload(filename, path)

        changeset =
          %Plugin{title: title, url: url, version: version}
          |> Plugin.changeset(%{user_id: conn.assigns.current_user.id})

        case IO.inspect(Repo.insert(changeset)) do
          {:ok, plugin} ->
            redirect(conn, to: ~p"/plugins")

          {:error, %Ecto.Changeset{} = changeset} ->
            Storage.delete(filename)
            render(conn, :new)
        end

        redirect(conn, to: ~p"/plugins")

      _ ->
        render(conn, :new, changeset: Plugin.changeset(%Plugin{}, %{}))
    end
  end

  def show(conn, %{"id" => id}) do
    plugin = Plugins.get_plugin!(id)
    render(conn, :show, plugin: plugin)
  end
end
