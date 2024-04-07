defmodule Cdfy.Room do
  require Logger

  defstruct [:room_id, :player_ids, :state_ids]

  @type t :: %__MODULE__{
          room_id: String.t(),
          player_ids: map(),
          state_ids: list(String.t())
        }

  def child_spec(opts) do
    room_id = Keyword.fetch!(opts, :room_id)

    %{
      id: room_id,
      start: {__MODULE__, :start_link, [opts]},
      shutdown: 3600_000,
      restart: :transient
    }
  end

  @spec new(room_id :: String.t()) :: t()
  def new(room_id) do
    %__MODULE__{
      room_id: room_id,
      player_ids: %{},
      state_ids: []
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
end
