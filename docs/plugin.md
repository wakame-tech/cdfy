# Plugin

- [extism](https://extism.org/docs/overview) を利用
- [Phoenix LiveView](https://hexdocs.pm/phoenix_live_view/Phoenix.LiveView.html) に依存

## interface

```rust
#[derive(Serialize)]
struct GameConstraints {
    min_players: u32,
    max_players: u32,
}

#[derive(Deserialize)]
struct GameConfig {
    player_ids: Vec<String>,
}

#[derive(Deserialize)]
struct CellValue {
    cell: Option<String>,
    value: Option<String>,
}

#[derive(Deserialize)]
struct LiveEvent {
    player_id: String,
    event_name: String,
    value: CellValue,
}

trait Plugin {
    fn get_constraints() -> GameConstraints
    fn init_game(config: GameConfig)
    fn handle_event(event: LiveEvent) -> Assigns
    fn render(assigns: Assigns) -> String
}
```