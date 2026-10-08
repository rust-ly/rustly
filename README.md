# Rustly

A browser-based Rust course written entirely in Rust: a Leptos (CSR) frontend compiled to WebAssembly, Rust API functions on Vercel, and shared crates. See [HANDOFF.md](HANDOFF.md) for the full spec and milestones.

## Local development

Prerequisites: Rust stable (the `rust-toolchain.toml` adds the `wasm32-unknown-unknown` target) and [Trunk](https://trunkrs.dev) (`brew install trunk` or `cargo install trunk --locked`).

```sh
# 1. API: each function in api/ is its own binary. vercel_runtime serves it on VERCEL_DEV_PORT.
#    Run each in its own terminal (or only the ones you need).
VERCEL_DEV_PORT=3001 cargo run --bin health
VERCEL_DEV_PORT=3002 cargo run --bin check
VERCEL_DEV_PORT=3003 cargo run --bin run
VERCEL_DEV_PORT=3004 cargo run --bin submit

# 2. Frontend: Trunk serves the app on :8080 and proxies each /api/* route to its port.
#    fetch-fonts.sh downloads Satoshi once (its license doesn't allow committing it).
./frontend/fetch-fonts.sh
cd frontend && trunk serve --port 8080
```

Open http://localhost:8080. The footer should say **API: ok**, and the course map is at `/learn`.

Try an endpoint directly:

```sh
curl -X POST localhost:3003 -d '{"code":"fn main() { println!(\"hi\"); }"}'
```

`RUNNER_URL` points the API at a self-hosted Playground instead of play.rust-lang.org.

## Checks

```sh
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo clippy -p frontend -p shared --target wasm32-unknown-unknown -- -D warnings
cargo test
# Optional, calls the real Playground: every exercise, snippet and starter submission.
cargo test -p content --features server -- --ignored
cargo test -p rustly-api -- --ignored
(cd frontend && trunk build --release)
```
