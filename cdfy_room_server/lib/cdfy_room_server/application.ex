defmodule CdfyRoomServer.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      CdfyRoomServerWeb.Telemetry,
      CdfyRoomServer.Repo,
      {DNSCluster, query: Application.get_env(:cdfy_room_server, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: CdfyRoomServer.PubSub},
      # Start the Finch HTTP client for sending emails
      # {Finch, name: CdfyRoomServer.Finch},
      {DynamicSupervisor, [name: CdfyRoomServer.RoomSupervisor, strategy: :one_for_one]},
      {Registry, keys: :unique, name: CdfyRoomServer.RoomRegistry},
      CdfyRoomServerWeb.Endpoint
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_all, name: CdfyRoomServer.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    CdfyRoomServerWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
