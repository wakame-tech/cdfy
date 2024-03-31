defmodule Cdfy.RoomState do
  alias Cdfy.Repo.Plugins
  alias Cdfy.Storage
  alias Cdfy.Plugin.Caller
  alias Cdfy.Plugin.State
  require Logger

  defstruct [:room_id, :player_ids, :states]

  @type t :: %__MODULE__{
          room_id: String.t(),
          player_ids: map(),
          states: map()
        }

  @spec cache_plugin(plugin_id :: String.t()) :: String.t()
  defp cache_plugin(plugin_id) do
    plugin = Plugins.get_plugin!(plugin_id)
    bin = Storage.download(plugin.title)
    path = Path.absname("./cache/#{plugin.title}.wasm")
    File.rm(path)
    File.write(path, bin)
    IO.inspect("plugin: #{plugin.title} v#{plugin.version} @ #{path} loaded")
    path
  end

  @spec new(room_id :: String.t()) :: t()
  def new(room_id) do
    %__MODULE__{
      room_id: room_id,
      player_ids: %{},
      states: %{}
    }
  end

  @spec get_plugin_state(self :: t(), state_id :: String.t()) :: {:ok, map()} | {:error, nil}
  def get_plugin_state(self, state_id) do
    self.states[state_id] |> State.get_plugin_state()
  end

  @spec get_phase(self :: t(), state_id :: String.t()) :: atom()
  def get_phase(self, state_id) do
    self.states[state_id].phase
  end

  @spec set_phase(self :: t(), state_id :: String.t(), phase :: atom()) :: t()
  def set_phase(self, state_id, phase) do
    states = self.states |> Map.put(state_id, self.states[state_id] |> Map.put(:phase, phase))
    %{self | states: states}
  end

  @spec render(self :: t(), state_id :: String.t(), pid :: any()) :: String.t()
  def render(self, state_id, pid) do
    State.render(self.states[state_id], Map.get(self.player_ids, pid))
  end

  @spec load_plugin(self :: t(), plugin_id :: String.t()) :: {t(), String.t()}
  def load_plugin(self, plugin_id) do
    Logger.info("load plugin #{plugin_id}")
    path = cache_plugin(plugin_id)
    state_id = Ecto.UUID.generate()
    {:ok, plugin} = Caller.new(path)

    states = self.states |> Map.put(state_id, State.new(plugin_id, plugin))
    {%{self | states: states}, state_id}
  end

  @spec unload_plugin(self :: t(), state_id :: String.t()) :: t()
  def unload_plugin(self, state_id) do
    Logger.info("unload state #{state_id}")
    plugin = self.states[state_id].plugin
    Caller.free(plugin)

    %{self | states: Map.delete(self.states, state_id)}
  end

  @spec join(self :: t(), pid :: any(), player_id :: String.t()) :: t()
  def join(self, pid, player_id) do
    Logger.info("join #{inspect(pid)} as #{player_id}")
    %{self | player_ids: self.player_ids |> Map.put(pid, player_id)}
  end

  @spec leave(self :: t(), pid :: any()) :: t()
  def leave(self, pid) do
    Logger.info("leave #{inspect(pid)}")
    %{self | player_ids: self.player_ids |> Map.delete(pid)}
  end

  @spec init_game(self :: t(), state_id :: String.t()) :: t()
  def init_game(self, state_id) do
    Logger.info("init_game #{state_id}")

    states =
      Map.put(
        self.states,
        state_id,
        State.init(self.states[state_id], Map.values(self.player_ids))
      )

    %{self | states: states}
  end

  @spec finish_game(self :: t(), state_id :: String.t()) :: t()
  def finish_game(self, state_id) do
    Logger.info("finish_game #{state_id}")
    states = Map.put(self.states, state_id, State.finish(self.states[state_id]))
    %{self | states: states}
  end

  @spec toggle_debug(self :: t(), state_id :: String.t()) :: t()
  def toggle_debug(self, state_id) do
    Logger.info("toggle_debug #{state_id}")
    states = Map.put(self.states, state_id, State.toggle_debug(self.states[state_id]))
    %{self | states: states}
  end

  @spec handle_plugin_event(self :: t(), state_id :: String.t(), ev :: map()) :: t()
  def handle_plugin_event(self, state_id, ev) do
    case ev do
      "GameFinished" ->
        Logger.info("game finished")
        self = set_phase(self, state_id, :waiting)

        {:ok, plugin_state} = Caller.get_state(self.states[state_id].plugin)

        event =
          %{
            player_id: state_id,
            event_name: "plugin_finished",
            value: plugin_state
          }

        dispatch_event(self, "", event)

      %{"StartPlugin" => %{"plugin_name" => plugin_name}} ->
        Logger.info("start plugin: #{plugin_name}")
        %{id: plugin_id} = Plugins.get_plugin_by_title(plugin_name)
        {self, state_id} = load_plugin(self, plugin_id)

        event =
          %{
            player_id: "",
            event_name: "plugin_started",
            value: %{plugin_id: plugin_id, state_id: state_id}
          }

        dispatch_event(self, "", event)

      _ ->
        self
    end
  end

  @spec dispatch_event(self :: t(), player_id :: String.t(), event :: map()) :: t()
  def dispatch_event(self, player_id, event) do
    Logger.info("dispatch event: #{inspect(event)}")

    Map.to_list(self.states)
    |> Enum.reduce(self, fn {state_id, state}, acc ->
      if state.phase == :waiting do
        acc
      else
        {:ok, {state, ev}} = State.dispatch_event(state, player_id, event)
        acc = %{acc | states: Map.put(acc.states, state_id, state)}
        handle_plugin_event(acc, state_id, ev)
      end
    end)
  end
end
