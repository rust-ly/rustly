#!/bin/sh
# Vercel's install step. Lives in a script because vercel.json caps
# installCommand at 256 characters.
set -eu
curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
. "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown
# Prebuilt Trunk: compiling it with `cargo install` takes several minutes.
curl -sSfL https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-musl.tar.gz |
  tar -xz -C "$HOME/.cargo/bin"
# frontend/fetch-fonts.sh needs unzip.
command -v unzip >/dev/null || dnf install -y unzip
