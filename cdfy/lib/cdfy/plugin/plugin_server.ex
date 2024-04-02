defmodule Cdfy.PluginServer do
  use GenServer
  require Logger
  alias Cdfy.RoomServer
  alias Cdfy.Plugin
  alias Cdfy.Plugin.Caller
  alias Cdfy.Repo.Plugins
  alias Cdfy.Storage

  def start(opts) do
    case DynamicSupervisor.start_child(
           Cdfy.PluginSupervisor,
           {__MODULE__, opts}
         ) do
      {:ok, pid} ->
        Logger.info("started plugin server #{inspect(opts)} #{inspect(pid)}")
        :ok

      :ignore ->
        Logger.info("Plugin server #{inspect(opts)} already running. Returning error")
        {:error, :already_exists}
    end
  end

  @impl true
  def child_spec(opts) do
    state_id = Keyword.fetch!(opts, :state_id)

    %{
      id: state_id,
      start: {__MODULE__, :start_link, [opts]},
      shutdown: 3600_000,
      restart: :transient
    }
  end

  @impl true
  def start_link(opts) do
    state_id = Keyword.fetch!(opts, :state_id)
    name = {:via, Registry, {Cdfy.PluginRegistry, state_id}}

    case GenServer.start_link(__MODULE__, [plugin_id: opts[:plugin_id]], name: name) do
      {:ok, pid} ->
        {:ok, pid}

      {:error, {:already_started, pid}} ->
        Logger.info("Already started at #{inspect(pid)}, returning :ignore")
        :ignore
    end
  end

  @spec cache_plugin(plugin_id :: String.t()) :: String.t()
  defp cache_plugin(plugin_id) do
    plugin = Plugins.get_plugin!(plugin_id)
    bin = Storage.download(plugin.title)
    path = Path.absname("./cache/#{plugin.title}.wasm")
    File.rm(path)
    File.write(path, bin)
    Logger.info("plugin: #{plugin.title} v#{plugin.version} @ #{path} loaded")
    path
  end

  @impl true
  @spec init(Keyword.t()) :: {:ok, Plugin.t()}
  def init(plugin_id: plugin_id) do
    path = cache_plugin(plugin_id)
    {:ok, plugin} = Caller.new(path)
    state = Plugin.new(plugin)
    {:ok, state}
  end

  def via_tuple(state_id), do: {:via, Registry, {Cdfy.PluginRegistry, state_id}}

  @spec exists?(state_id :: String.t()) :: boolean()
  def exists?(state_id) do
    Cdfy.PluginRegistry
    |> Registry.lookup(state_id)
    |> Enum.any?()
  end

  @spec get_state(state_id :: String.t()) :: Plugin.t()
  def get_state(state_id) do
    GenServer.call(via_tuple(state_id), :state)
  end

  def handle_call(:state, _from, state) do
    {:reply, state, state}
  end

  @spec init_game(state_id :: String.t(), player_ids :: list(String.t())) :: :ok
  def init_game(state_id, player_ids) do
    GenServer.call(via_tuple(state_id), {:init_game, player_ids})
  end

  def handle_call({:init_game, player_ids}, _from, state) do
    {:reply, :ok, Plugin.init(state, player_ids)}
  end

  @spec finish_game(state_id :: String.t()) :: :ok
  def finish_game(state_id) do
    GenServer.call(via_tuple(state_id), :finish_game)
  end

  def handle_call(:finish_game, _from, state) do
    {:reply, :ok, Plugin.finish(state)}
  end

  @spec dispatch_event(state_id :: String.t(), event :: map()) :: :ok
  def dispatch_event(state_id, event) do
    GenServer.call(via_tuple(state_id), {:dispatch_event, event})
  end

  def handle_call({:dispatch_event, event}, _from, state) do
    if state.phase == :ingame do
      %{room_id: room_id} = event
      {:ok, {state, ev}} = Plugin.dispatch_event(state, event)

      state =
        case ev do
          "None" ->
            state

          "GameFinished" ->
            %{state | phase: :waiting}

          %{"StartPlugin" => %{"plugin_name" => plugin_name}} ->
            %{id: plugin_id} = Plugins.get_plugin_by_title(plugin_name)
            state_id = Ecto.UUID.generate()
            :ok = RoomServer.add_plugin(room_id, plugin_id, state_id)
            state

          ev ->
            Logger.info("unknown event: #{inspect(ev)}")
            state
        end

      {:reply, :ok, state}
    else
      {:reply, :ok, state}
    end
  end

  @spec get_plugin_state(state_id :: String.t()) ::
          {:ok, map()} | {:error, nil}
  def get_plugin_state(state_id) do
    GenServer.call(via_tuple(state_id), :get_plugin_state)
  end

  def handle_call(:get_plugin_state, _from, state) do
    {:reply, Plugin.get_plugin_state(state), state}
  end

  @spec render(state_id :: String.t(), player_id :: String.t()) :: String.t()
  def render(state_id, player_id) do
    GenServer.call(via_tuple(state_id), {:render, player_id})
  end

  def handle_call({:render, player_id}, _from, state) do
    {:reply, Plugin.render(state, player_id), state}
  end

  @spec stop(state_id :: String.t()) :: :ok
  def stop(state_id) do
    GenServer.stop(via_tuple(state_id))
  end
end
