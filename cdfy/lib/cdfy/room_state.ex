defmodule Cdfy.RoomState do
  alias Cdfy.Repo.Plugins
  alias Cdfy.Storage
  alias Cdfy.Plugin.Caller
  require Logger

  defstruct [:room_id, :player_ids, :phase, :plugin_id, :plugin]

  @type t :: %__MODULE__{
          room_id: String.t(),
          player_ids: map(),
          phase: atom(),
          plugin_id: String.t(),
          plugin: any()
        }

  @spec cache_plugin(String.t()) :: String.t()
  defp cache_plugin(plugin_id) do
    plugin = Plugins.get_plugin!(plugin_id)
    bin = Storage.download(plugin.title)
    path = Path.absname("./cache/#{plugin.title}.wasm")
    File.rm(path)
    File.write(path, bin)
    IO.inspect("plugin: #{plugin.title} v#{plugin.version} @ #{path} loaded")
    path
  end

  @spec new(String.t(), String.t()) :: {:ok, t()}
  def new(room_id, plugin_id) do
    state = %__MODULE__{
      room_id: room_id,
      player_ids: %{},
      phase: :waiting,
      plugin_id: plugin_id,
      plugin: nil
    }

    {:ok, state}
  end

  @spec render(t(), any()) :: String.t()
  def render(self, pid) do
    case self.phase do
      :waiting ->
        ""

      :ingame ->
        player_id = Map.get(self.player_ids, pid)
        Caller.render(self.plugin, player_id)
    end
  end

  @spec load_plugin(t()) :: {:ok, t()}
  def load_plugin(self) do
    Logger.info("load plugin #{self.plugin_id}")
    path = cache_plugin(self.plugin_id)
    {:ok, plugin} = Caller.new(path)
    {:ok, %{self | plugin: plugin, phase: :waiting}}
  end

  @spec join(t(), any(), String.t()) :: t()
  def join(self, pid, player_id) do
    Logger.info("join #{inspect(self)}")
    %{self | player_ids: self.player_ids |> Map.put(pid, player_id)}
  end

  @spec leave(t(), any()) :: t()
  def leave(self, pid) do
    Logger.info("leave #{inspect(self)}")
    %{self | player_ids: self.player_ids |> Map.delete(pid)}
  end

  @spec load_game(t()) :: t()
  def load_game(self) do
    case self.phase do
      :waiting ->
        player_ids = Map.values(self.player_ids)
        :ok = Caller.init(self.plugin, player_ids)
        %{self | phase: :ingame}

      :ingame ->
        self
    end
  end

  @spec finish_game(t()) :: t()
  def finish_game(self) do
    case self.phase do
      :waiting ->
        self

      :ingame ->
        %{self | phase: :waiting}
    end
  end

  @spec new_event(t(), map()) :: {:ok, t()} | {:error, any()}
  def new_event(self, event) do
    case self.phase do
      :waiting ->
        {:error, :not_loaded}

      :ingame ->
        Logger.info("event: #{inspect(event)}")

        state =
          case Caller.handle_event(self.plugin, event) do
            {:ok, status} when status != 0 ->
              Logger.info("game finished status: #{status}")
              {:ok, last_game_state} = Caller.get_state(self.plugin)
              Logger.info("last game_state: #{inspect(last_game_state)}")
              %{self | phase: :waiting}

            _ ->
              %{self | plugin: self.plugin}
          end

        {:ok, state}
    end
  end
end
