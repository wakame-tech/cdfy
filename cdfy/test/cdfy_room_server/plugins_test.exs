defmodule Cdfy.PluginsTest do
  use Cdfy.DataCase

  alias Cdfy.Plugins

  describe "plugins" do
    alias Cdfy.Plugins.Plugin

    import Cdfy.PluginsFixtures

    @invalid_attrs %{version: nil, title: nil, url: nil}

    test "list_plugins/0 returns all plugins" do
      plugin = plugin_fixture()
      assert Plugins.list_plugins() == [plugin]
    end

    test "get_plugin!/1 returns the plugin with given id" do
      plugin = plugin_fixture()
      assert Plugins.get_plugin!(plugin.id) == plugin
    end

    test "create_plugin/1 with valid data creates a plugin" do
      valid_attrs = %{version: "some version", title: "some title", url: "some url"}

      assert {:ok, %Plugin{} = plugin} = Plugins.create_plugin(valid_attrs)
      assert plugin.version == "some version"
      assert plugin.title == "some title"
      assert plugin.url == "some url"
    end

    test "create_plugin/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Plugins.create_plugin(@invalid_attrs)
    end

    test "update_plugin/2 with valid data updates the plugin" do
      plugin = plugin_fixture()

      update_attrs = %{
        version: "some updated version",
        title: "some updated title",
        url: "some updated url"
      }

      assert {:ok, %Plugin{} = plugin} = Plugins.update_plugin(plugin, update_attrs)
      assert plugin.version == "some updated version"
      assert plugin.title == "some updated title"
      assert plugin.url == "some updated url"
    end

    test "update_plugin/2 with invalid data returns error changeset" do
      plugin = plugin_fixture()
      assert {:error, %Ecto.Changeset{}} = Plugins.update_plugin(plugin, @invalid_attrs)
      assert plugin == Plugins.get_plugin!(plugin.id)
    end

    test "delete_plugin/1 deletes the plugin" do
      plugin = plugin_fixture()
      assert {:ok, %Plugin{}} = Plugins.delete_plugin(plugin)
      assert_raise Ecto.NoResultsError, fn -> Plugins.get_plugin!(plugin.id) end
    end

    test "change_plugin/1 returns a plugin changeset" do
      plugin = plugin_fixture()
      assert %Ecto.Changeset{} = Plugins.change_plugin(plugin)
    end
  end
end
