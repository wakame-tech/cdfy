defmodule Cdfy.Repo.Plugins do
  @moduledoc """
  The Plugins context.
  """

  import Ecto.Query, warn: false
  alias Cdfy.Repo
  alias Cdfy.Model.Plugin

  @doc """
  Returns the list of plugins.

  ## Examples

      iex> list_plugins()
      [%Plugin{}, ...]

  """
  def list_plugins() do
    Repo.all(Plugin)
  end

  def exists_plugin?(title), do: Plugin |> where([p], p.title == ^title) |> Repo.exists?()

  def get_plugin!(id), do: Repo.get!(Plugin, id)

  def get_plugin_by_title(title), do: Plugin |> where([p], p.title == ^title) |> Repo.one()

  @doc """
  Creates a plugin.

  ## Examples

      iex> create_plugin(%{field: value})
      {:ok, %Plugin{}}

      iex> create_plugin(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_plugin(attrs \\ %{}) do
    %Plugin{}
    |> Plugin.changeset(attrs)
    |> Repo.insert(on_conflict: :nothing)
  end

  @doc """
  Updates a plugin.

  ## Examples

      iex> update_plugin(plugin, %{field: new_value})
      {:ok, %Plugin{}}

      iex> update_plugin(plugin, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_plugin(%Plugin{} = plugin, attrs) do
    plugin
    |> Plugin.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a plugin.

  ## Examples

      iex> delete_plugin(plugin)
      {:ok, %Plugin{}}

      iex> delete_plugin(plugin)
      {:error, %Ecto.Changeset{}}

  """
  def delete_plugin(%Plugin{} = plugin) do
    Repo.delete(plugin)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking plugin changes.

  ## Examples

      iex> change_plugin(plugin)
      %Ecto.Changeset{data: %Plugin{}}

  """
  def change_plugin(%Plugin{} = plugin, attrs \\ %{}) do
    Plugin.changeset(plugin, attrs)
  end
end
