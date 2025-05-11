#!/bin/bash

# Install rustup
curl https://sh.rustup.rs -sSf | sh -s -- -y

source $HOME/.cargo/env

# Add wasm target
rustup target add wasm32-unknown-unknown

# Install trunk
cargo install trunk

# Build the project
trunk build --release

