#!/bin/sh
# Vercel's install step. Lives in a script because vercel.json caps
# installCommand at 256 characters.
set -eux
# Tools this script and frontend/fetch-fonts.sh need. Install any the
# build image is missing.
for tool in curl tar gzip unzip; do
  command -v "$tool" >/dev/null || dnf install -y "$tool"
done
curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
. "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown
# Prebuilt Trunk: compiling it with `cargo install` takes several minutes.
curl -sSfL -o /tmp/trunk.tar.gz https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-musl.tar.gz
tar -xzf /tmp/trunk.tar.gz -C "$HOME/.cargo/bin"
trunk --version
