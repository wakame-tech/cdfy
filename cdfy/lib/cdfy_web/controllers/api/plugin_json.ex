defmodule CdfyWeb.Api.PluginJSON do
  alias Cdfy.Model.Plugin

  @doc """
  Renders a list of plugins.
  """
  def index(%{plugins: plugins}) do
    %{data: for(plugin <- plugins, do: data(plugin))}
  end

  @doc """
  Renders a single plugin.
  """
  def show(%{plugin: plugin}) do
    %{data: data(plugin)}
  end

  defp data(%Plugin{} = plugin) do
    %{
      id: plugin.id,
      title: plugin.title,
      version: plugin.version,
      url: plugin.url
    }
  end
end
