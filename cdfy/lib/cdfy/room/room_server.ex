defmodule Cdfy.RoomServer do
  use GenServer, restart: :temporary
  require Logger
  alias Cdfy.PluginSupervisor
  alias Phoenix.PubSub
  alias Cdfy.Room
  alias Cdfy.PluginServer

  def start_link(room_id: room_id) do
    name = {:via, Registry, {Cdfy.RoomRegistry, room_id}}
    opts = [room_id: room_id]
    GenServer.start_link(__MODULE__, opts, name: name)
  end

  def via_tuple(room_id), do: {:via, Registry, {Cdfy.RoomRegistry, room_id}}

  @impl true
  @spec init(Keyword.t()) :: {:ok, Room.t()}
  def init(room_id: room_id) do
    Logger.info("RoomServer.init #{room_id}")
    {:ok, Room.new(room_id)}
  end

  @spec exists?(room_id :: String.t()) :: boolean()
  def exists?(room_id) do
    Cdfy.RoomRegistry
    |> Registry.lookup(room_id)
    |> Enum.any?()
  end

  @spec get_state_ids(room_id :: String.t()) :: list(String.t())
  def get_state_ids(room_id) do
    GenServer.call(via_tuple(room_id), :get_state_ids)
  end

  def handle_call(:get_state_ids, _from, state) do
    {:reply, state.state_ids, state}
  end

  @spec get_player_ids(room_id :: String.t()) :: list(String.t())
  def get_player_ids(room_id) do
    GenServer.call(via_tuple(room_id), :get_player_ids)
  end

  def handle_call(:get_player_ids, _from, state) do
    {:reply, Map.values(state.player_ids), state}
  end

  @spec add_plugin(room_id :: String.t(), plugin_id :: String.t(), state_id :: String.t()) :: :ok
  def add_plugin(room_id, plugin_id, state_id) do
    Logger.info("add_plugin: #{room_id} #{exists?(room_id)} #{plugin_id} #{state_id}")
    GenServer.call(via_tuple(room_id), {:add_plugin, plugin_id, state_id})
  end

  def handle_call({:add_plugin, plugin_id, state_id}, _from, state) do
    args = [plugin_id: plugin_id, state_id: state_id]
    PluginSupervisor.start_child(args)
    {:reply, :ok, %{state | state_ids: state.state_ids ++ [state_id]}}
  end

  @spec unload_plugin(room_id :: String.t(), state_id :: String.t()) :: :ok
  def unload_plugin(room_id, state_id) do
    GenServer.call(via_tuple(room_id), {:unload_plugin, state_id})
  end

  def handle_call({:unload_plugin, state_id}, _from, state) do
    :ok = PluginServer.stop(state_id)
    {:reply, :ok, %{state | state_ids: state.state_ids -- [state_id]}}
  end

  @spec monitor(room_id :: String.t(), player_id :: String.t()) :: :ok
  def monitor(room_id, player_id) do
    GenServer.cast(via_tuple(room_id), {:monitor, player_id, self()})
  end

  def handle_cast({:monitor, player_id, pid}, state) do
    Process.monitor(pid)
    state = Room.join(state, pid, player_id)
    {:noreply, state}
  end

  @spec dispatch_event_all(room_id :: String.t(), event :: map()) :: :ok
  def dispatch_event_all(room_id, event) do
    GenServer.call(via_tuple(room_id), {:dispatch_event_all, event})
  end

  def handle_call({:dispatch_event_all, event}, _from, state) do
    Logger.info("dispatch_event_all: #{inspect(event)}")

    state.state_ids
    |> Enum.map(fn state_id ->
      PluginServer.dispatch_event(state_id, event)
    end)

    {:reply, :ok, state}
  end

  @impl true
  def handle_info({:DOWN, _ref, :process, pid, _}, %{room_id: room_id} = state) do
    state = Room.leave(state, pid)
    PubSub.broadcast(Cdfy.PubSub, "room:#{room_id}", :refresh)

    Logger.info(
      "room=#{room_id} player=#{inspect(pid)} left players=#{inspect(state.player_ids)}"
    )

    if Enum.empty?(state.player_ids) do
      # FIXME: I don't know how to nest dynamic supervisors
      Enum.each(state.state_ids, &PluginServer.stop(&1))
      {:stop, :normal, state}
    else
      {:noreply, state}
    end
  end
end
