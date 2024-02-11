defmodule Cdfy.PluginsFixtures do
  @moduledoc """
  This module defines test helpers for creating
  entities via the `Cdfy.Plugins` context.
  """

  @doc """
  Generate a plugin.
  """
  def plugin_fixture(attrs \\ %{}) do
    {:ok, plugin} =
      attrs
      |> Enum.into(%{
        title: "some title",
        url: "some url",
        version: "some version"
      })
      |> Cdfy.Plugins.create_plugin()

    plugin
  end
end
