defmodule Cdfy.Plugin.Caller do
  require Logger

  alias Cdfy.Event

  def log_wasm_error(e) do
    Logger.error(String.split(e, "\n") |> Enum.join("\n"))
  end

  @doc """
  call when a room is created
  """
  @spec new(String.t()) :: {:ok, any()} | {:error, any()}
  def new(wasm_path) do
    manifest =
      %{
        wasm: [
          %{path: wasm_path}
        ]
      }

    Extism.Plugin.new(manifest, true)
  end

  @doc """
  call when the "load" button is clicked
  """
  @spec init(any(), [String.t()]) :: :ok | :error
  def init(plugin, player_ids) do
    game_config = %{player_ids: player_ids}

    case Extism.Plugin.call(plugin, "init_game", Jason.encode!(game_config)) do
      {:ok, _res} ->
        :ok

      {:error, e} ->
        log_wasm_error(e)
        :error
    end
  end

  @doc """
  call when plugin state is updated, return html
  """
  @spec render(any(), String.t()) :: String.t()
  def render(plugin, player_id) do
    render_config = %{player_id: player_id}

    case Extism.Plugin.call(plugin, "render", Jason.encode!(render_config)) do
      {:ok, res} ->
        res

      {:error, e} ->
        log_wasm_error(e)
        ""
    end
  end

  @doc """
  call when a player clicks a button
  """
  @spec handle_event(plugin :: any(), player_id :: String.t(), event :: Event.t()) ::
          {:ok, Event.t()} | {:error, String.t()}
  def handle_event(plugin, player_id, event) do
    arg = %{
      player_id: player_id,
      event: event
    }

    case Extism.Plugin.call(plugin, "handle_event", Jason.encode!(arg)) do
      {:ok, r} ->
        event = Jason.decode!(r, keys: :atoms)
        {:ok, event}

      {:error, e} ->
        log_wasm_error(e)
        {:error, e}
    end
  end

  @doc """
  for debugging
  """
  @spec get_state(any()) :: {:ok, map()} | {:error, nil}
  def get_state(plugin) do
    case Extism.Plugin.call(plugin, "get_state", Jason.encode!(%{})) do
      {:ok, res} ->
        Jason.decode(res, keys: :atoms)

      {:error, e} ->
        log_wasm_error(e)
        {:error, nil}
    end
  end

  @spec free(any()) :: :ok
  def free(plugin) do
    Extism.Plugin.free(plugin)
  end
end
