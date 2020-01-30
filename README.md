
# rust setup MAC
$ brew install rustup

$ rustup-init

# or more universal linux/mac install
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# wasm setup

needs following:

# switching to a dark side :)
$ rustup default nightly

# wasm target
$ rustup target add wasm32-unknown-unknown

# installing tool to remove unneded exports and other weight
$ cargo instal wasm-gc

# web server tool
$ cargo instal https

# creating new lib 
$ cargo new --lib utils

$ cd ./utils 


![Web Assembly Logo (WASM)] (https://webassembly.org/css/webassembly.svg)
