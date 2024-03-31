defmodule Cdfy.RoomState do
  alias Cdfy.Repo.Plugins
  alias Cdfy.Storage
  alias Cdfy.Plugin.Caller
  require Logger

  defstruct [:room_id, :player_ids, :plugin_ids, :phases, :plugins]

  @type t :: %__MODULE__{
          room_id: String.t(),
          player_ids: map(),
          plugin_ids: map(),
          phases: map(),
          plugins: map()
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

  @spec new(room_id :: String.t()) :: {:ok, t()}
  def new(room_id) do
    state = %__MODULE__{
      room_id: room_id,
      player_ids: %{},
      plugin_ids: %{},
      phases: %{},
      plugins: %{}
    }

    {:ok, state}
  end

  @spec get_plugin_state(self :: t(), state_id :: String.t()) :: {:ok, map()} | {:error, nil}
  def get_plugin_state(self, state_id) do
    case self.phases[state_id] do
      :waiting ->
        {:ok, %{}}

      :ingame ->
        plugin = self.plugins[state_id]
        Caller.get_state(plugin)

      _ ->
        {:error, nil}
    end
  end

  @spec render(self :: t(), state_id :: String.t(), pid :: any()) :: String.t()
  def render(self, state_id, pid) do
    phase = self.phases[state_id]
    plugin = self.plugins[state_id]

    case phase do
      :waiting ->
        ""

      :ingame ->
        player_id = Map.get(self.player_ids, pid)
        Caller.render(plugin, player_id)

      _ ->
        ""
    end
  end

  @spec load_plugin(self :: t(), plugin_id :: String.t()) :: {:ok, {t(), String.t()}}
  def load_plugin(self, plugin_id) do
    Logger.info("load plugin #{plugin_id}")
    path = cache_plugin(plugin_id)
    state_id = Ecto.UUID.generate()
    {:ok, plugin} = Caller.new(path)

    self =
      %__MODULE__{
        self
        | plugin_ids: Map.put(self.plugin_ids, state_id, plugin_id),
          plugins: Map.put(self.plugins, state_id, plugin),
          phases: Map.put(self.phases, state_id, :waiting)
      }

    {:ok, {self, state_id}}
  end

  @spec unload_plugin(self :: t(), state_id :: String.t()) :: t()
  def unload_plugin(self, state_id) do
    Logger.info("unload state #{state_id}")
    plugin = self.plugins[state_id]
    Caller.free(plugin)

    %{
      self
      | plugin_ids: Map.delete(self.plugin_ids, state_id),
        plugins: Map.delete(self.plugins, state_id),
        phases: Map.delete(self.phases, state_id)
    }
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
    plugin = self.plugins[state_id]

    case self.phases[state_id] do
      :waiting ->
        player_ids = Map.values(self.player_ids)
        :ok = Caller.init(plugin, player_ids)

        %{
          self
          | plugins: Map.put(self.plugins, state_id, plugin),
            phases: Map.put(self.phases, state_id, :ingame)
        }

      :ingame ->
        self
    end
  end

  @spec finish_game(self :: t(), state_id :: String.t()) :: t()
  def finish_game(self, state_id) do
    Logger.info("finish_game #{state_id}")

    case self.phases[state_id] do
      :waiting ->
        self

      :ingame ->
        %{self | phases: Map.put(self.phases, state_id, :waiting)}
    end
  end

  @spec new_event(self :: t(), state_id :: String.t(), event :: map()) ::
          {:ok, t()} | {:error, any()}
  def new_event(self, state_id, event) do
    Logger.info("new_event #{state_id} #{inspect(event)}")
    plugin = self.plugins[state_id]

    case self.phases[state_id] do
      :waiting ->
        {:error, :not_loaded}

      :ingame ->
        Logger.info("event: #{inspect(event)}")

        state =
          case Caller.handle_event(plugin, event) do
            {:ok, status} when status != 0 ->
              Logger.info("game finished status: #{status}")
              {:ok, last_game_state} = Caller.get_state(plugin)
              Logger.info("last game_state: #{inspect(last_game_state)}")
              %{self | phases: Map.put(self.phases, state_id, :waiting)}

            _ ->
              %{self | plugins: Map.put(self.plugins, state_id, plugin)}
          end

        {:ok, state}
    end
  end
end
