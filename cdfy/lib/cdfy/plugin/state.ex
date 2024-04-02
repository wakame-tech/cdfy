defmodule Cdfy.Plugin.State do
  alias Cdfy.Plugin.Caller
  require Logger

  defstruct [:plugin_id, :phase, :plugin, :errors, :debug]

  @type t :: %__MODULE__{
          plugin_id: String.t(),
          phase: atom(),
          plugin: any(),
          errors: map(),
          debug: boolean()
        }

  @spec new(plugin_id :: String.t(), plugin :: any()) :: t()
  def new(plugin_id, plugin) do
    %__MODULE__{
      plugin_id: plugin_id,
      plugin: plugin,
      phase: :waiting,
      errors: %{},
      debug: false
    }
  end

  @spec toggle_debug(self :: t()) :: t()
  def toggle_debug(self) do
    %{self | debug: not self.debug}
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

  @spec dispatch_event(self :: t(), event :: map()) ::
          {:ok, {t(), map() | String.t()}} | {:error, any()}
  def dispatch_event(self, %{player_id: player_id} = event) do
    case Caller.handle_event(self.plugin, event) do
      {:ok, ev} ->
        {:ok, {self, ev}}

      {:error, e} ->
        Logger.error("error handling event: #{inspect(e)}")
        %{self | errors: Map.put(self.errors, player_id, e)}
        {:error, e}
    end
  end
end
