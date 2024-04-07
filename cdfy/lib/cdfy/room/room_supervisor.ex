defmodule Cdfy.RoomSupervisor do
  use DynamicSupervisor
  require Logger

  alias Cdfy.RoomServer

  def states() do
    DynamicSupervisor.which_children(__MODULE__)
    |> Enum.map(&elem(&1, 1))
    |> Enum.map(fn pid -> :sys.get_state(pid) end)
  end

  def start_link(_opts) do
    Logger.info("RoomSupervisor.start_link")
    DynamicSupervisor.start_link(__MODULE__, [], name: __MODULE__)
  end

  def start_child(args) do
    spec = {RoomServer, args}
    Logger.info("RoomSupervisor.start_child #{inspect(spec)}")
    DynamicSupervisor.start_child(__MODULE__, spec)
  end

  @impl true
  def init(_) do
    Logger.info("RoomSupervisor.init")
    DynamicSupervisor.init(strategy: :one_for_one)
  end
end
