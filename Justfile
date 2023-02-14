gen:
    cd ./crates/cdfy-binding-gen && cargo run

plugin name:
    cd ./crates/{{name}} && cargo build --target=wasm32-unknown-unknown
    wasmer inspect ./crates/{{name}}/target/wasm32-unknown-unknown/debug/{{name}}.wasm
    cp ./crates/{{name}}/target/wasm32-unknown-unknown/debug/{{name}}.wasm ./server