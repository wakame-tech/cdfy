defmodule Mix.Tasks.Cdfy.ValidatePlugin do
  use Mix.Task
  require Logger
  alias Cdfy.Plugin.Caller

  def run([plugin_dir]) do
    wasm_path = build_plugin(plugin_dir)

    {:ok, plugin} =
      Caller.new(wasm_path)

    :ok = Caller.init(plugin, [])

    test_builtin_events(plugin)
  end

  defp build_plugin(plugin_dir) do
    # {:ok, cargo_toml} = File.read("#{plugin_dir}/Cargo.toml")
    # {:ok, %{"package" => %{"name" => name, "version" => version}}} = Toml.decode(cargo_toml)

    # build
    0 = Mix.Shell.IO.cmd("cd #{plugin_dir} && cargo build --target wasm32-wasi --release")
    [wasm_path] = Path.wildcard("#{plugin_dir}/target/wasm32-wasi/release/*.wasm")

    wasm_path
  end

  defp test_builtin_events(plugin) do
    builtin_events = [
      %{
        name: "None"
      },
      %{
        name: "Exit"
      },
      %{
        name: "LaunchPlugin",
        value: %{
          plugin_name: "String"
        }
      },
      %{
        name: "PluginStarted",
        value: %{
          state_id: "String"
        }
      },
      %{
        name: "PluginFinished",
        value: %{
          state_id: "String",
          value: %{
            hoge: "hoge"
          }
        }
      }
    ]

    Enum.each(builtin_events, fn event ->
      res =
        Caller.handle_event(plugin, "player_id", event)

      Logger.info("Testing event: #{inspect(event)} = #{inspect(res)}")
    end)

    :ok
  end
end
