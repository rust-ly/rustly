#!/bin/sh
# Vercel's install step. Lives in a script because vercel.json caps
# installCommand at 256 characters.
set -eux
# Tools this script and frontend/fetch-fonts.sh need. Install any the
# build image is missing.
for tool in curl tar gzip unzip; do
  command -v "$tool" >/dev/null || dnf install -y "$tool"
done
# Vercel's build image already ships rustup in /rust/bin. Only install it
# if it's missing.
if ! command -v rustup >/dev/null; then
  curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
  . "$HOME/.cargo/env"
fi
rustup target add wasm32-unknown-unknown
# Prebuilt Trunk: compiling it with `cargo install` takes several minutes.
# Put it next to cargo so it's on PATH in the build step too.
curl -sSfL -o /tmp/trunk.tar.gz https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-musl.tar.gz
tar -xzf /tmp/trunk.tar.gz -C "$(dirname "$(command -v cargo)")"
trunk --version
