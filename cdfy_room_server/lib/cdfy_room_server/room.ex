defmodule CdfyRoomServer.Room do
  use GenServer
  require Logger
  alias Phoenix.PubSub

  @wasm_path "plugins/template.wasm"

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

  def monitor(room_id) do
    GenServer.cast(via_tuple(room_id), {:monitor, self()})
  end

  def new_event(room_id, event) do
    GenServer.cast(via_tuple(room_id), {:new_event, event})
  end

  def render(room_id) do
    GenServer.call(via_tuple(room_id), :render)
  end

  def handle_call(:load_game, _from, state) do
    {:ok, plugin} =
      Extism.Plugin.new(%{wasm: [%{path: @wasm_path}]}, false)

    game_config = %{player_ids: []}

    {:ok, _res} =
      Extism.Plugin.call(plugin, "init_game", Jason.encode!(game_config))

    state = Map.put(state, :plugin, plugin)

    {:reply, :ok, state}
  end

  def handle_cast({:new_event, event}, %{plugin: plugin} = state) do
    {:ok, _res} =
      Extism.Plugin.call(plugin, "handle_event", Jason.encode!(event))

    state = Map.put(state, :plugin, plugin)

    {:noreply, state}
  end

  def handle_call(:render, _from, %{plugin: plugin} = state) do
    if plugin do
      {:ok, html} =
        Extism.Plugin.call(plugin, "render", Jason.encode!(%{}))

      {:reply, html, state}
    else
      {:reply, "", state}
    end
  end

  @impl true
  def handle_cast({:monitor, pid}, %{pids: pids} = state) do
    Process.monitor(pid)
    state = state |> Map.put(:pids, Enum.concat([pid], pids))
    {:noreply, state}
  end

  @impl true
  def handle_info({:DOWN, _ref, :process, pid, _}, %{pids: pids} = state) do
    pids = List.delete(pids, pid)
    state = Map.put(state, :pids, pids)

    if Enum.empty?(pids) do
      {:stop, :normal, state}
    else
      {:noreply, state}
    end
  end
end
