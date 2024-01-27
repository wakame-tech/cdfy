defmodule CdfyRoomServer.Room do
  use GenServer
  require Logger
  alias Phoenix.PubSub

  # @wasm_path "plugins/template.wasm"
  @wasm_path "plugins/cdfy_career_poker_plugin.wasm"

  def start(room_id) do
    case DynamicSupervisor.start_child(
           CdfyRoomServer.RoomSupervisor,
           {__MODULE__, [room_id: room_id]}
         ) do
      {:ok, _pid} ->
        Logger.info("Started game server #{inspect(room_id)}")
        {:ok, :initiated}

      :ignore ->
        Logger.info("Game server #{inspect(room_id)} already running. Returning error")
        {:error, :already_exists}
    end
  end

  def child_spec(opts) do
    room_id = Keyword.fetch!(opts, :room_id)

    %{
      id: "room_#{room_id}",
      start: {__MODULE__, :start_link, [room_id]},
      shutdown: 10_000,
      restart: :transient
    }
  end

  def start_link(room_id) do
    name = {:via, Registry, {CdfyRoomServer.RoomRegistry, room_id}}

    case GenServer.start_link(__MODULE__, %{room_id: room_id}, name: name) do
      {:ok, pid} ->
        {:ok, pid}

      {:error, {:already_started, pid}} ->
        Logger.info("Already started at #{inspect(pid)}, returning :ignore")
        :ignore
    end
  end

  def init(%{room_id: room_id}) do
    {:ok,
     %{
       room_id: room_id,
       # PID to player_id
       player_ids: %{},
       plugin: nil,
       pids: []
     }}
  end

  def via_tuple(room_id), do: {:via, Registry, {CdfyRoomServer.RoomRegistry, room_id}}

  @spec exists?(room_id :: String.t()) :: boolean()
  def exists?(room_id) do
    CdfyRoomServer.RoomRegistry
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
    PubSub.broadcast(CdfyRoomServer.PubSub, "room:#{room_id}", {:version, version})
  end

  def load_game(room_id) do
    GenServer.call(via_tuple(room_id), :load_game)
  end

  def monitor(room_id, player_id) do
    GenServer.cast(via_tuple(room_id), {:monitor, player_id, self()})
  end

  def new_event(room_id, event) do
    GenServer.cast(via_tuple(room_id), {:new_event, event})
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

  def handle_call(:load_game, _from, state) do
    {:ok, plugin} =
      Extism.Plugin.new(%{wasm: [%{path: @wasm_path}]}, true)

    game_config = %{player_ids: Map.values(state.player_ids)}

    {:ok, _res} =
      Extism.Plugin.call(plugin, "init_game", Jason.encode!(game_config))

    state = Map.put(state, :plugin, plugin)

    {:reply, :ok, state}
  end

  def handle_cast({:new_event, event}, %{plugin: plugin} = state) do
    Logger.info("event: #{inspect(event)}")
    case Extism.Plugin.call(plugin, "handle_event", Jason.encode!(event)) do
      {:ok, _res} ->
        :ok

      {:error, e} ->
        Logger.error("Error handling event: #{inspect(e)}")
    end

    state = Map.put(state, :plugin, plugin)

    {:noreply, state}
  end

  def handle_call(:get_plugin_state, _from, %{plugin: plugin} = state) do
    if plugin do
      {:ok, res} =
        Extism.Plugin.call(plugin, "get_state", Jason.encode!(%{}))

      res =
        Jason.decode!(res)
        |> IO.inspect()

      {:reply, res, state}
    else
      {:reply, %{}, state}
    end
  end

  def handle_call(:render, {pid, _}, %{plugin: plugin, player_ids: player_ids} = state) do
    render_config = %{player_id: Map.get(player_ids, pid)}
    if plugin do
      case Extism.Plugin.call(plugin, "render", Jason.encode!(render_config)) do
        {:ok, html} ->
          {:reply, html, state}

        {:error, e} ->
          Logger.error("Error rendering plugin: #{inspect(e)}")
          {:reply, "", state}
      end
    else
      {:reply, "", state}
    end
  end

  def handle_call(:get_player_ids, _from, %{player_ids: player_ids} = state) do
    {:reply, Map.values(player_ids), state}
  end

  @impl true
  def handle_cast({:monitor, player_id, pid}, %{pids: pids} = state) do
    Process.monitor(pid)
    state = state
      |> Map.put(:pids, Enum.concat([pid], pids))
      |> Map.put(:player_ids, Map.put(state.player_ids, pid, player_id))
    {:noreply, state}
  end

  @impl true
  def handle_info({:DOWN, _ref, :process, pid, _}, %{pids: pids} = state) do
    pids = List.delete(pids, pid)
    state = state
      |> Map.put(:pids, pids)
      |> Map.put(:player_ids, Map.delete(state.player_ids, pid))

    if Enum.empty?(pids) do
      {:stop, :normal, state}
    else
      {:noreply, state}
    end
  end
end
