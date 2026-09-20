#!/bin/bash

# Install rustup
curl https://sh.rustup.rs -sSf | sh -s -- -y

source $HOME/.cargo/env

# Add wasm target
rustup target add wasm32-unknown-unknown

# Install Trunk from the prebuilt release binary
curl -L \
  https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz \
  -o /tmp/trunk.tar.gz

tar -xzf /tmp/trunk.tar.gz -C /tmp

chmod +x /tmp/trunk
mv /tmp/trunk "$HOME/.cargo/bin/trunk"

# Build the project
trunk build --release

