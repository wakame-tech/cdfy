defmodule Cdfy.Event do
  defstruct [:room_id, :player_id, :event_name, :value]

  @type t :: %__MODULE__{
          room_id: String.t(),
          player_id: String.t(),
          event_name: String.t(),
          value: map()
        }

  @spec new(
          room_id :: String.t(),
          player_id :: String.t(),
          event_name :: String.t(),
          value :: map()
        ) :: t()
  def new(room_id, player_id, event_name, value) do
    %__MODULE__{
      room_id: room_id,
      player_id: player_id,
      event_name: event_name,
      value: value
    }
  end
end
