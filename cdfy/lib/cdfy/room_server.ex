defmodule Cdfy.RoomServer do
  use GenServer
  require Logger
  alias Cdfy.Room
  alias Cdfy.PluginServer

  def start(opts) do
    case DynamicSupervisor.start_child(
           Cdfy.RoomSupervisor,
           {__MODULE__, opts}
         )
         |> IO.inspect() do
      {:ok, pid} ->
        Logger.info("started room server #{inspect(opts)} #{inspect(pid)}")
        :ok

      :ignore ->
        Logger.info("room server #{inspect(opts)} already running. Returning error")
        {:error, :already_exists}
    end
  end

  @impl true
  def child_spec(opts) do
    room_id = Keyword.fetch!(opts, :room_id)

    %{
      id: room_id,
      start: {__MODULE__, :start_link, [opts]},
      shutdown: 3600_000,
      restart: :transient
    }
  end

  @impl true
  def start_link(opts) do
    room_id = Keyword.fetch!(opts, :room_id)
    name = {:via, Registry, {Cdfy.RoomRegistry, room_id}}

    case GenServer.start_link(__MODULE__, opts, name: name) do
      {:ok, pid} ->
        {:ok, pid}

      {:error, {:already_started, pid}} ->
        Logger.info("Already started at #{inspect(pid)}, returning :ignore")
        :ignore
    end
  end

  @impl true
  @spec init(Keyword.t()) :: {:ok, Room.t()}
  def init(room_id: room_id) do
    Logger.info("init room #{room_id}")
    {:ok, Room.new(room_id)}
  end

  def via_tuple(room_id), do: {:via, Registry, {Cdfy.RoomRegistry, room_id}}

  def room_states() do
    DynamicSupervisor.which_children(Cdfy.RoomSupervisor)
    |> Enum.map(&elem(&1, 1))
    |> Enum.map(fn pid -> :sys.get_state(pid) end)
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
    GenServer.cast(via_tuple(room_id), {:add_plugin, plugin_id, state_id})
  end

  def handle_cast({:add_plugin, plugin_id, state_id}, state) do
    case PluginServer.start(plugin_id: plugin_id, state_id: state_id) do
      :ok ->
        {:noreply, %{state | state_ids: state.state_ids ++ [state_id]}}

      {:error, :already_exists} ->
        {:noreply, state}
    end
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
    {:noreply, state |> Room.join(pid, player_id)}
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
  def handle_info({:DOWN, _ref, :process, pid, _}, state) do
    Logger.info("Player disconnected: #{inspect(pid)}")
    state = Room.leave(state, pid)

    if Enum.empty?(state.player_ids) do
      # {:noreply, %{state | phase: :waiting}}
      {:stop, :normal, state}
    else
      {:noreply, state}
    end
  end
end
