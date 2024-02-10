defmodule CdfyRoomServer.Plugin do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "plugins" do
    field :version, :string
    field :title, :string
    field :url, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(plugin, attrs) do
    plugin
    |> cast(attrs, [:title, :version, :url])
    |> validate_required([:title, :version, :url])
  end
end
