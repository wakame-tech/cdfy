defmodule Cdfy.PluginRunner do
  require Logger

  def log_wasm_error(e) do
    Logger.error(String.split(e, "\n") |> Enum.join("\n"))
  end

  @doc """
  call when a room is created
  """
  def new(_url) do
    # manifest =
    #   %{wasm: [%{url: plugin.url}]}

    # TODO
    manifest = %{
      wasm: [
        # %{path: "plugins/cdfy_career_poker_plugin.wasm"}
        # %{path: "plugins/cdfy_template_plugin.wasm"}
        %{path: "plugins/cdfy_plugin_janken.wasm"}
      ]
    }

    Extism.Plugin.new(manifest, true)
  end

  @doc """
  call when the "load" button is clicked
  """
  def init(plugin, player_ids) do
    game_config = %{player_ids: player_ids}

    case Extism.Plugin.call(plugin, "init_game", Jason.encode!(game_config)) |> IO.inspect() do
      {:ok, _res} ->
        {:ok, nil}

      {:error, e} ->
        log_wasm_error(e)
        {:error, nil}
    end
  end

  @doc """
  call when plugin state is updated
  """
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
  def handle_event(plugin, event) do
    case Extism.Plugin.call(plugin, "handle_event", Jason.encode!(event)) do
      {:ok, status} ->
        {:ok, :binary.decode_unsigned(status, :little)}

      {:error, e} ->
        log_wasm_error(e)
        {:error, e}
    end
  end

  @doc """
  for debugging
  """
  def get_state(plugin) do
    case Extism.Plugin.call(plugin, "get_state", Jason.encode!(%{})) do
      {:ok, res} ->
        Jason.decode(res)

      {:error, e} ->
        log_wasm_error(e)
        {:error, nil}
    end
  end

  def free(plugin) do
    Extism.Plugin.free(plugin)
  end
end
