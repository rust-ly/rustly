#!/bin/sh
# Vercel's build step: builds the frontend into frontend/dist.
set -eux
# Only present if vercel-install.sh had to install rustup itself.
if [ -f "$HOME/.cargo/env" ]; then . "$HOME/.cargo/env"; fi
cd frontend
./fetch-fonts.sh
trunk build --release
