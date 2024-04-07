defmodule Cdfy.PluginSupervisor do
  use DynamicSupervisor
  require Logger

  alias Cdfy.PluginServer

  def start_link() do
    DynamicSupervisor.start_link(__MODULE__, [], name: __MODULE__)
  end

  @impl true
  def init(_) do
    DynamicSupervisor.init(strategy: :one_for_one)
  end

  def start_child(args) do
    spec = {PluginServer, args}
    DynamicSupervisor.start_child(__MODULE__, spec)
  end
end
