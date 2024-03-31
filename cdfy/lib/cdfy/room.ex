defmodule Cdfy.Room do
  use GenServer
  require Logger
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
  @spec init(Keyword.t()) :: {:ok, RoomState.t()}
  def init(room_id: room_id, plugin_id: plugin_id) do
    Logger.info("init room #{room_id} with plugin #{plugin_id}")

    state = RoomState.new(room_id)
    {state, _} = RoomState.load_plugin(state, plugin_id)
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

  @spec get_state(room_id :: String.t()) :: RoomState.t()
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

  @spec get_phase(room_id :: String.t(), state_id :: String.t()) :: atom() | nil
  def get_phase(room_id, state_id) do
    get_state(room_id).states[state_id].phase
  end

  @spec load_plugin(room_id :: String.t(), plugin_id :: String.t()) :: {:ok, String.t()}
  def load_plugin(room_id, plugin_id) do
    GenServer.call(via_tuple(room_id), {:load_plugin, plugin_id})
  end

  def handle_call({:load_plugin, plugin_id}, _from, state) do
    {state, state_id} = RoomState.load_plugin(state, plugin_id)
    {:reply, {:ok, state_id}, state}
  end

  @spec unload_plugin(room_id :: String.t(), state_id :: String.t()) :: :ok
  def unload_plugin(room_id, state_id) do
    GenServer.call(via_tuple(room_id), {:unload_plugin, state_id})
  end

  def handle_call({:unload_plugin, state_id}, _from, state) do
    {:reply, :ok, RoomState.unload_plugin(state, state_id)}
  end

  @spec refresh_plugin(room_id :: String.t(), state_id :: String.t()) :: :ok
  def refresh_plugin(room_id, state_id) do
    GenServer.call(via_tuple(room_id), {:refresh_plugin, state_id})
  end

  @spec init_game(room_id :: String.t(), plugin_id :: String.t()) :: :ok
  def init_game(room_id, plugin_id) do
    GenServer.call(via_tuple(room_id), {:init_game, plugin_id})
  end

  def handle_call({:init_game, plugin_id}, _from, state) do
    {:reply, :ok, RoomState.init_game(state, plugin_id)}
  end

  @spec finish_game(room_id :: String.t(), plugin_id :: String.t()) :: :ok
  def finish_game(room_id, plugin_id) do
    GenServer.call(via_tuple(room_id), {:finish_game, plugin_id})
  end

  def handle_call({:finish_game, plugin_id}, _from, state) do
    {:reply, :ok, RoomState.finish_game(state, plugin_id)}
  end

  @spec monitor(room_id :: String.t(), plugin_id :: String.t()) :: :ok
  def monitor(room_id, player_id) do
    GenServer.cast(via_tuple(room_id), {:monitor, player_id, self()})
  end

  def handle_cast({:monitor, player_id, pid}, state) do
    Process.monitor(pid)
    {:noreply, state |> RoomState.join(pid, player_id)}
  end

  @spec toggle_debug(room_id :: String.t(), state_id :: String.t()) :: :ok
  def toggle_debug(room_id, state_id) do
    GenServer.call(via_tuple(room_id), {:toggle_debug, state_id})
  end

  def handle_call({:toggle_debug, state_id}, _from, state) do
    state = RoomState.toggle_debug(state, state_id)
    {:reply, :ok, state}
  end

  @spec dispatch_event(room_id :: String.t(), player_id :: String.t(), event :: map()) :: :ok
  def dispatch_event(room_id, player_id, event) do
    GenServer.call(via_tuple(room_id), {:dispatch_event, player_id, event})
  end

  def handle_call({:dispatch_event, player_id, event}, _from, state) do
    state = RoomState.dispatch_event(state, player_id, event)
    {:reply, :ok, state}
  end

  @spec get_plugin_state(room_id :: String.t(), state_id :: String.t()) ::
          {:ok, map()} | {:error, nil}
  def get_plugin_state(room_id, state_id) do
    GenServer.call(via_tuple(room_id), {:get_plugin_state, state_id})
  end

  def handle_call({:get_plugin_state, state_id}, _from, state) do
    {:reply, RoomState.get_plugin_state(state, state_id), state}
  end

  @spec render(room_id :: String.t(), plugin_id :: String.t()) :: String.t()
  def render(room_id, plugin_id) do
    GenServer.call(via_tuple(room_id), {:render, plugin_id})
  end

  def handle_call({:render, plugin_id}, {pid, _}, state) do
    {:reply, RoomState.render(state, plugin_id, pid), state}
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
