cargo build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/utils.wasm -o utils.gc.wasm