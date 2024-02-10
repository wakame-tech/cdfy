defmodule CdfyRoomServer.Repo do
  use Ecto.Repo,
    otp_app: :cdfy_room_server,
    adapter: Ecto.Adapters.Postgres
end
