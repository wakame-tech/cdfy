defmodule Cdfy.Repo.Migrations.CreatePlugins do
  use Ecto.Migration

  def change do
    drop(table(:plugins))

    create table(:plugins, primary_key: false) do
      add(:id, :binary_id, primary_key: true)
      add(:title, :string)
      add(:version, :string)
      add(:url, :string)

      timestamps(type: :utc_datetime)
    end

    create unique_index(:plugins, [:url])
  end
end
