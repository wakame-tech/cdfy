defmodule Cdfy.Room do
  use GenServer
  require Logger
  alias Phoenix.PubSub
  alias Cdfy.PluginRunner
  alias Cdfy.PluginFile

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
  def init(opts) do
    room_id = Keyword.get(opts, :room_id)
    title = Keyword.get(opts, :plugin_title)
    # plugin = Plugins.get_plugin_by_title(title)

    bin = PluginFile.download(title)
    path = "./cache/#{title}.wasm"
    File.write(path, bin)

    {:ok, plugin} = PluginRunner.new(path)

    state = %{
      room_id: room_id,
      # PID to player_id
      player_ids: %{},
      phase: :waiting,
      plugin: plugin,
      pids: []
    }

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

  def state(room_id) do
    if exists?(room_id) do
      GenServer.call(via_tuple(room_id), :state)
    else
      %{room_id: room_id}
    end
  end

  def broadcast_game_state(room_id, version) do
    PubSub.broadcast(Cdfy.PubSub, "room:#{room_id}", {:version, version})
  end

  def refresh_plugin(room_id) do
    GenServer.call(via_tuple(room_id), :refresh_plugin)
  end

  def load_game(room_id) do
    GenServer.call(via_tuple(room_id), :load_game)
  end

  def finish_game(room_id) do
    GenServer.call(via_tuple(room_id), :finish_game)
  end

  def monitor(room_id, player_id) do
    GenServer.cast(via_tuple(room_id), {:monitor, player_id, self()})
  end

  def new_event(room_id, event) do
    GenServer.call(via_tuple(room_id), {:new_event, event})
  end

  # for debug
  def get_plugin_state(room_id) do
    GenServer.call(via_tuple(room_id), :get_plugin_state)
  end

  def get_player_ids(room_id) do
    GenServer.call(via_tuple(room_id), :get_player_ids)
  end

  def render(room_id) do
    GenServer.call(via_tuple(room_id), :render)
  end

  @impl true
  def handle_call(:refresh_plugin, _from, %{plugin_info: plugin_info} = state) do
    Logger.info("refresh plugin #{inspect(plugin_info)}")
    {:ok, plugin} = PluginRunner.new(plugin_info.url)

    state =
      state
      |> Map.put(:plugin, plugin)
      |> Map.put(:phase, :waiting)

    {:reply, {:ok, nil}, state}
  end

  @impl true
  def handle_call(
        :load_game,
        _from,
        %{plugin: plugin, player_ids: player_ids, phase: phase} = state
      ) do
    case phase do
      :waiting ->
        player_ids = Map.values(player_ids)
        res = PluginRunner.init(plugin, player_ids)
        state = Map.put(state, :phase, :ingame)
        {:reply, res, state}

      :ingame ->
        {:reply, {:ok, nil}, state}
    end
  end

  @impl true
  def handle_call(:finish_game, _from, %{phase: phase} = state) do
    case phase do
      :waiting ->
        {:reply, {:ok, nil}, state}

      :ingame ->
        state = Map.put(state, :phase, :waiting)
        {:reply, {:ok, nil}, state}
    end
  end

  def handle_call({:new_event, event}, _from, %{plugin: plugin, phase: phase} = state) do
    case phase do
      :waiting ->
        {:reply, {:error, :not_loaded}, state}

      :ingame ->
        Logger.info("event: #{inspect(event)}")

        {res, state} =
          case PluginRunner.handle_event(plugin, event) do
            {:ok, status} when status != 0 ->
              Logger.info("game finished status: #{status}")
              last_game_state = PluginRunner.get_state(plugin)
              Logger.info("last game_state: #{inspect(last_game_state)}")
              {{:ok, nil}, Map.put(state, :phase, :waiting)}

            res ->
              {res, Map.put(state, :plugin, plugin)}
          end

        {:reply, res, state}
    end
  end

  def handle_call(:state, _from, state) do
    {:reply, state, state}
  end

  def handle_call(:get_plugin_state, _from, %{plugin: plugin, phase: phase} = state) do
    case phase do
      :waiting ->
        {:reply, {:ok, nil}, state}

      :ingame ->
        res = PluginRunner.get_state(plugin)
        {:reply, res, state}
    end
  end

  def handle_call(
        :render,
        {pid, _},
        %{plugin: plugin, player_ids: player_ids, phase: phase} = state
      ) do
    case phase do
      :waiting ->
        {:reply, "", state}

      :ingame ->
        player_id = Map.get(player_ids, pid)
        html = PluginRunner.render(plugin, player_id)
        {:reply, html, state}
    end
  end

  @impl true
  def handle_cast({:monitor, player_id, pid}, %{pids: pids} = state) do
    Process.monitor(pid)

    state =
      state
      |> Map.put(:pids, Enum.concat([pid], pids))
      |> Map.put(:player_ids, Map.put(state.player_ids, pid, player_id))

    {:noreply, state}
  end

  @impl true
  def handle_info({:DOWN, _ref, :process, pid, _}, %{pids: pids} = state) do
    Logger.info("Player disconnected: #{inspect(pid)}")
    pids = List.delete(pids, pid)

    state =
      state
      |> Map.put(:pids, pids)
      |> Map.put(:player_ids, Map.delete(state.player_ids, pid))

    if Enum.empty?(pids) do
      # {:noreply, %{state | phase: :waiting}}
      {:stop, :normal, state}
    else
      {:noreply, state}
    end
  end
end
