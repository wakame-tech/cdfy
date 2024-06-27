defmodule Cdfy.Repo.Migrations.PluginBelongsToUser do
  use Ecto.Migration

  def change do
    alter table(:plugins) do
      add(:user_id, references(:users, type: :binary_id))
    end
  end
end
