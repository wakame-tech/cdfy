serve:
    just plugin-debug career_poker
    cd server && just run

release:
    just plugins
    cd server && just deploy
    cd client && npm run deploy

plugins:
    just gen
    just plugin counter
    just plugin career_poker

gen:
    cd ./crates/cdfy-binding-gen && cargo run

plugin name:
    cd ./plugins/{{name}} && cargo build --release --target=wasm32-unknown-unknown
    wasmer inspect ./plugins/{{name}}/target/wasm32-unknown-unknown/release/{{name}}.wasm
    cp ./plugins/{{name}}/target/wasm32-unknown-unknown/release/{{name}}.wasm ./server

plugin-debug name:
    cd ./plugins/{{name}} && cargo build --target=wasm32-unknown-unknown
    wasmer inspect ./plugins/{{name}}/target/wasm32-unknown-unknown/debug/{{name}}.wasm
    cp ./plugins/{{name}}/target/wasm32-unknown-unknown/debug/{{name}}.wasm ./server