serve:
    just plugin-debug counter
    cd server && just run

release:
    just plugins
    cd server && just deploy
    cd client && npm run build && npm run deploy

plugins:
    just gen
    just plugin counter-server
    just plugin career_poker

gen:
    cd ./crates/cdfy-binding-gen && cargo run

plugin name:
    cd ./plugins/{{name}} && cargo build --release --target=wasm32-unknown-unknown
    wasmer inspect ./plugins/{{name}}/target/wasm32-unknown-unknown/release/*.wasm
    cp ./plugins/{{name}}/target/wasm32-unknown-unknown/release/*.wasm .cache/

plugin-debug name:
    cd ./plugins/{{name}} && cargo build --target=wasm32-unknown-unknown
    wasmer inspect ./plugins/{{name}}/target/wasm32-unknown-unknown/debug/*.wasm
    cp ./plugins/{{name}}/target/wasm32-unknown-unknown/debug/*.wasm .cache/