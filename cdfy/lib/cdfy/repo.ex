defmodule Cdfy.Repo do
  use Ecto.Repo,
    otp_app: :cdfy,
    adapter: Ecto.Adapters.Postgres
end
