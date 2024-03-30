defmodule Mix.Tasks.Cdfy.UploadPlugin do
  use Mix.Task
  alias Cdfy.Plugins
  alias Cdfy.PluginFile
  alias Cdfy.PluginRunner

  defp prepare_plugin(plugin_dir) do
    {:ok, cargo_toml} = File.read("#{plugin_dir}/Cargo.toml")
    {:ok, %{"package" => %{"name" => name, "version" => version}}} = Toml.decode(cargo_toml)

    # build
    0 = Mix.Shell.IO.cmd("cd #{plugin_dir} && cargo build --release")
    [wasm_path] = Path.wildcard("#{plugin_dir}/target/wasm32-wasi/release/*.wasm")

    # upload
    {:ok, _} = PluginFile.upload(name, wasm_path)

    %{title: name, version: version, url: name}
  end

  defp instantiate_plugin(%{title: name}) do
    wasm = PluginFile.download(name)
    path = "./cache/#{name}.wasm"
    File.write(path, wasm)

    {:ok, plugin} =
      PluginRunner.new(path)

    {:ok, _} = PluginRunner.init(plugin, [])
    {:ok, state} = PluginRunner.get_state(plugin)
    IO.inspect(state)
  end

  @shortdoc "Upload plugin from git repo"
  def run([plugin_dir]) do
    :ok = Mix.Task.run("app.start")
    plugin = prepare_plugin(plugin_dir)
    {:ok, _} = Plugins.create_plugin(plugin)
    IO.puts("Plugin #{plugin.title} v#{plugin.version} uploaded")

    instantiate_plugin(plugin)
  end
end
