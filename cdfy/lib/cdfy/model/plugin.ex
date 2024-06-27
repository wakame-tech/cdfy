defmodule Cdfy.Model.Plugin do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "plugins" do
    field(:version, :string)
    field(:title, :string)
    field(:url, :string)
    # https://gist.github.com/stevedomin/0ea9d9af96b565cbd0b7
    belongs_to(:user, EctoAssoc.Plugin, type: :binary_id)

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(plugin, attrs) do
    plugin
    |> cast(attrs, [:title, :version, :url])
    |> validate_required([:title, :version, :url])
  end
end
