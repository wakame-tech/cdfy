default:
    just plugin counter
    just plugin career-poker
    cd server && just run

release:
    just gen
    just plugin counter
    just plugin career-poker
    cd server && just deploy
    cd client && npm run deploy

gen:
    cd ./crates/cdfy-binding-gen && cargo run

plugin name:
    cd ./crates/{{name}} && cargo build --target=wasm32-unknown-unknown
    wasmer inspect ./crates/{{name}}/target/wasm32-unknown-unknown/debug/*.wasm
    cp ./crates/{{name}}/target/wasm32-unknown-unknown/debug/*.wasm ./server
