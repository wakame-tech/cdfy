defmodule CdfyRoomServerWeb.PluginControllerTest do
  use CdfyRoomServerWeb.ConnCase

  import CdfyRoomServer.PluginsFixtures

  alias CdfyRoomServer.Plugins.Plugin

  @create_attrs %{
    version: "some version",
    title: "some title",
    url: "some url"
  }
  @update_attrs %{
    version: "some updated version",
    title: "some updated title",
    url: "some updated url"
  }
  @invalid_attrs %{version: nil, title: nil, url: nil}

  setup %{conn: conn} do
    {:ok, conn: put_req_header(conn, "accept", "application/json")}
  end

  describe "index" do
    test "lists all plugins", %{conn: conn} do
      conn = get(conn, ~p"/api/plugins")
      assert json_response(conn, 200)["data"] == []
    end
  end

  describe "create plugin" do
    test "renders plugin when data is valid", %{conn: conn} do
      conn = post(conn, ~p"/api/plugins", plugin: @create_attrs)
      assert %{"id" => id} = json_response(conn, 201)["data"]

      conn = get(conn, ~p"/api/plugins/#{id}")

      assert %{
               "id" => ^id,
               "title" => "some title",
               "url" => "some url",
               "version" => "some version"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn} do
      conn = post(conn, ~p"/api/plugins", plugin: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "update plugin" do
    setup [:create_plugin]

    test "renders plugin when data is valid", %{conn: conn, plugin: %Plugin{id: id} = plugin} do
      conn = put(conn, ~p"/api/plugins/#{plugin}", plugin: @update_attrs)
      assert %{"id" => ^id} = json_response(conn, 200)["data"]

      conn = get(conn, ~p"/api/plugins/#{id}")

      assert %{
               "id" => ^id,
               "title" => "some updated title",
               "url" => "some updated url",
               "version" => "some updated version"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn, plugin: plugin} do
      conn = put(conn, ~p"/api/plugins/#{plugin}", plugin: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "delete plugin" do
    setup [:create_plugin]

    test "deletes chosen plugin", %{conn: conn, plugin: plugin} do
      conn = delete(conn, ~p"/api/plugins/#{plugin}")
      assert response(conn, 204)

      assert_error_sent 404, fn ->
        get(conn, ~p"/api/plugins/#{plugin}")
      end
    end
  end

  defp create_plugin(_) do
    plugin = plugin_fixture()
    %{plugin: plugin}
  end
end
