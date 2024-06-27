defmodule Cdfy.Plugin do
  alias Cdfy.Plugin.Caller
  alias Cdfy.Event
  require Logger

  defstruct [:phase, :plugin, :errors]

  @type t :: %__MODULE__{
          phase: atom(),
          plugin: any(),
          errors: map()
        }

  @spec new(plugin :: any()) :: t()
  def new(plugin) do
    %__MODULE__{
      plugin: plugin,
      phase: :waiting,
      errors: %{}
    }
  end

  @spec get_plugin_state(self :: t()) :: {:ok, map()} | {:error, nil}
  def get_plugin_state(self) do
    case self.phase do
      :waiting ->
        {:ok, %{}}

      :ingame ->
        plugin = self.plugin
        Caller.get_state(plugin)

      _ ->
        {:error, nil}
    end
  end

  @spec render(self :: t(), player_id :: String.t()) :: String.t()
  def render(self, player_id) do
    case self.phase do
      :waiting ->
        ""

      :ingame ->
        Caller.render(self.plugin, player_id)

      _ ->
        ""
    end
  end

  @spec init(self :: t(), player_ids :: [String.t()]) :: t()
  def init(self, player_ids) do
    case self.phase do
      :waiting ->
        :ok = Caller.init(self.plugin, player_ids)
        %{self | phase: :ingame}

      :ingame ->
        self
    end
  end

  @spec set_phase(self :: t(), phase :: atom()) :: t()
  def set_phase(self, phase) do
    %{self | phase: phase}
  end

  @spec finish(self :: t()) :: t()
  def finish(self) do
    case self.phase do
      :waiting ->
        self

      :ingame ->
        %{self | phase: :waiting}
    end
  end

  @spec dispatch_event(self :: t(), player_id :: String.t(), event :: Event.t()) ::
          {:ok, {t(), Event.t()}} | {:error, any()}
  def dispatch_event(self, player_id, event) do
    case Caller.handle_event(self.plugin, player_id, event) do
      {:ok, reply} ->
        new_self = %{self | errors: Map.delete(self.errors, player_id)}
        {:ok, {new_self, reply}}

      {:error, e} ->
        Logger.error("error handling event: #{inspect(e)}")
        new_self = %{self | errors: Map.put(self.errors, player_id, e)}
        {:ok, {new_self, %{name: "None"}}}
    end
  end
end
