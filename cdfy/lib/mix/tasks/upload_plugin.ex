defmodule Mix.Tasks.Cdfy.UploadPlugin do
  use Mix.Task
  alias Cdfy.Repo.Plugins
  alias Cdfy.Storage
  alias Cdfy.Plugin.Caller

  defp prepare_plugin(plugin_dir) do
    {:ok, cargo_toml} = File.read("#{plugin_dir}/Cargo.toml")
    {:ok, %{"package" => %{"name" => name, "version" => version}}} = Toml.decode(cargo_toml)

    # build
    0 = Mix.Shell.IO.cmd("cd #{plugin_dir} && cargo build --release")
    [wasm_path] = Path.wildcard("#{plugin_dir}/target/wasm32-wasi/release/*.wasm")

    # upload
    {:ok, _} = Storage.upload(name, wasm_path)

    %{title: name, version: version, url: name}
  end

  defp instantiate_plugin(%{title: name}) do
    wasm = Storage.download(name)
    path = "./cache/#{name}.wasm"
    File.write(path, wasm)

    {:ok, plugin} =
      Caller.new(path)

    :ok = Caller.init(plugin, [])
    {:ok, state} = Caller.get_state(plugin)
    IO.inspect(state)
  end

  @shortdoc "Upload plugin from git repo"
  def run([plugin_dir]) do
    :ok = Mix.Task.run("app.start")
    plugin = prepare_plugin(plugin_dir)

    if Plugins.exists_plugin?(plugin.title) do
      p = Plugins.get_plugin_by_title(plugin.title)
      {:ok, _} = Plugins.update_plugin(p, plugin)
    else
      {:ok, _} = Plugins.create_plugin(plugin)
    end

    IO.puts("Plugin #{plugin.title} v#{plugin.version} uploaded")

    instantiate_plugin(plugin)
  end
end
