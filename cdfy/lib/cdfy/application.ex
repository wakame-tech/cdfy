defmodule Cdfy.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      CdfyWeb.Telemetry,
      Cdfy.Repo,
      {DNSCluster, query: Application.get_env(:cdfy, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Cdfy.PubSub},
      # Start the Finch HTTP client for sending emails
      # {Finch, name: Cdfy.Finch},
      {DynamicSupervisor, [name: Cdfy.RoomSupervisor, strategy: :one_for_one]},
      {Registry, keys: :unique, name: Cdfy.RoomRegistry},
      {DynamicSupervisor, [name: Cdfy.PluginSupervisor, strategy: :one_for_one]},
      {Registry, keys: :unique, name: Cdfy.PluginRegistry},
      CdfyWeb.Endpoint
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_all, name: Cdfy.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    CdfyWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
