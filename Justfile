

gen:
    cd ./crates/example-protocol && cargo run

plugin:
    cd ./crates/example-plugin && cargo build --target=wasm32-unknown-unknown