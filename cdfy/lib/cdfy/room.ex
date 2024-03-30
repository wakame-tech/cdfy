defmodule Cdfy.Room do
  use GenServer
  require Logger
  alias Phoenix.PubSub
  alias Cdfy.Plugin.Caller
  alias Cdfy.RoomState

  def start(opts) do
    case DynamicSupervisor.start_child(
           Cdfy.RoomSupervisor,
           {__MODULE__, opts}
         ) do
      {:ok, _pid} ->
        Logger.info("Started game server #{inspect(opts)}")
        {:ok, :initiated}

      :ignore ->
        Logger.info("Game server #{inspect(opts)} already running. Returning error")
        {:error, :already_exists}
    end
  end

  def child_spec(opts) do
    room_id = Keyword.fetch!(opts, :room_id)

    %{
      id: room_id,
      start: {__MODULE__, :start_link, [opts]},
      shutdown: 3600_000,
      restart: :transient
    }
  end

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
  def init(room_id: room_id, plugin_id: plugin_id) do
    Logger.info("init room #{room_id} with plugin #{plugin_id}")

    {:ok, state} = RoomState.new(room_id, plugin_id)
    {:ok, state} = RoomState.load_plugin(state)
    {:ok, state}
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

  @spec get_state(room_id :: String.t()) :: map()
  def get_state(room_id) do
    if exists?(room_id) do
      GenServer.call(via_tuple(room_id), :state)
    else
      %{room_id: room_id}
    end
  end

  def handle_call(:state, _from, state) do
    {:reply, state, state}
  end

  @spec broadcast(room_id :: String.t(), version :: integer()) :: :ok
  def broadcast(room_id, version) do
    PubSub.broadcast(Cdfy.PubSub, "room:#{room_id}", {:version, version})
  end

  @spec refresh_plugin(String.t()) :: :ok
  def refresh_plugin(room_id) do
    GenServer.call(via_tuple(room_id), :refresh_plugin)
  end

  def handle_call(:refresh_plugin, _from, state) do
    {:ok, state} = RoomState.load_plugin(state)
    {:reply, :ok, state}
  end

  @spec load_game(String.t()) :: :ok
  def load_game(room_id) do
    GenServer.call(via_tuple(room_id), :load_game)
  end

  def handle_call(:load_game, _from, state) do
    {:reply, :ok, RoomState.load_game(state)}
  end

  @spec finish_game(String.t()) :: :ok
  def finish_game(room_id) do
    GenServer.call(via_tuple(room_id), :finish_game)
  end

  def handle_call(:finish_game, _from, state) do
    {:reply, :ok, state |> RoomState.finish_game()}
  end

  @spec monitor(String.t(), String.t()) :: :ok
  def monitor(room_id, player_id) do
    GenServer.cast(via_tuple(room_id), {:monitor, player_id, self()})
  end

  def handle_cast({:monitor, player_id, pid}, state) do
    Process.monitor(pid)
    {:noreply, state |> RoomState.join(pid, player_id)}
  end

  @spec new_event(String.t(), map()) :: :ok | {:error, any()}
  def new_event(room_id, event) do
    GenServer.call(via_tuple(room_id), {:new_event, event})
  end

  def handle_call({:new_event, event}, _from, state) do
    case RoomState.new_event(state, event) do
      {:ok, state} -> {:reply, :ok, state}
      {:error, e} -> {:reply, {:error, e}, state}
    end
  end

  @spec get_plugin_state(String.t()) :: {:ok, any()} | {:error, any()}
  def get_plugin_state(room_id) do
    GenServer.call(via_tuple(room_id), :get_plugin_state)
  end

  def handle_call(:get_plugin_state, _from, %{plugin: plugin, phase: phase} = state) do
    case phase do
      :waiting ->
        {:reply, {:ok, nil}, state}

      :ingame ->
        res = Caller.get_state(plugin)
        {:reply, res, state}
    end
  end

  @spec render(String.t()) :: String.t()
  def render(room_id) do
    GenServer.call(via_tuple(room_id), :render)
  end

  def handle_call(:render, {pid, _}, state) do
    {:reply, RoomState.render(state, pid), state}
  end

  @impl true
  def handle_info({:DOWN, _ref, :process, pid, _}, state) do
    Logger.info("Player disconnected: #{inspect(pid)}")
    state = RoomState.leave(state, pid)

    if Enum.empty?(state.player_ids) do
      # {:noreply, %{state | phase: :waiting}}
      {:stop, :normal, state}
    else
      {:noreply, state}
    end
  end
end
