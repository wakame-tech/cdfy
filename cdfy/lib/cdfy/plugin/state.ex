defmodule Cdfy.Plugin.State do
  alias Cdfy.Room
  require Logger

  defstruct [:version, :errors, :debug]

  @type t :: %__MODULE__{
          version: integer(),
          errors: map(),
          debug: boolean()
        }

  @spec new() :: t()
  def new() do
    %__MODULE__{
      version: 0,
      errors: %{},
      debug: false
    }
  end

  @spec toggle_debug(t()) :: t()
  def toggle_debug(self) do
    %{self | debug: not self.debug}
  end

  @spec set_version(t(), integer()) :: t()
  def set_version(self, version) do
    %{self | version: version}
  end

  @spec dispatch_event(t(), String.t(), String.t(), String.t(), map()) :: t()
  def dispatch_event(self, room_id, player_id, event_name, value) do
    event =
      %{
        player_id: player_id,
        event_name: event_name,
        value: value
      }

    case Room.new_event(room_id, event) do
      {:error, e} ->
        Logger.error("error handling event: #{inspect(e)}")
        %{self | errors: Map.put(self.errors, player_id, e)}

      :ok ->
        self
    end
  end
end
