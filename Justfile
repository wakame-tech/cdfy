default:
    just plugin counter  # career-poker
    cd server && just run

gen:
    cd ./crates/cdfy-binding-gen && cargo run

plugin name:
    cd ./crates/{{name}} && cargo build --target=wasm32-unknown-unknown
    wasmer inspect ./crates/{{name}}/target/wasm32-unknown-unknown/debug/*.wasm
    cp ./crates/{{name}}/target/wasm32-unknown-unknown/debug/*.wasm ./server
