# wasm setup

needs following:

# switching to a dark side :)
rustup default nightly

# wasm target
rustup target add wasm32-unknown-unknown

# installing tool to remove unneded exports and other weight
cargo instal wasm-gc

# web erver tool
cargo instal https

# creating new lib 
cargo new --lib utils