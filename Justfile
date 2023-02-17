serve:
    just plugins
    cd server && just run

release:
    just plugins
    cd server && just deploy
    cd client && npm run deploy

plugins:
    just gen
    just plugin counter
    just plugin career-poker

gen:
    cd ./crates/cdfy-binding-gen && cargo run

plugin name:
    cd ./crates/{{name}} && cargo build --release --target=wasm32-unknown-unknown
    wasmer inspect ./crates/{{name}}/target/wasm32-unknown-unknown/release/*.wasm
    cp ./crates/{{name}}/target/wasm32-unknown-unknown/release/*.wasm ./server

plugin-debug name:
    cd ./crates/{{name}} && cargo build --target=wasm32-unknown-unknown
    wasmer inspect ./crates/{{name}}/target/wasm32-unknown-unknown/debug/*.wasm
    cp ./crates/{{name}}/target/wasm32-unknown-unknown/debug/*.wasm ./server