# Rustly

A browser-based Rust course written entirely in Rust: a Leptos (CSR) frontend compiled to WebAssembly, Rust API functions on Vercel, and shared crates. See [HANDOFF.md](HANDOFF.md) for the full spec and milestones.

## Local development

Prerequisites: Rust stable (the `rust-toolchain.toml` adds the `wasm32-unknown-unknown` target) and [Trunk](https://trunkrs.dev) (`brew install trunk` or `cargo install trunk --locked`).

```sh
# 1. API: each function in api/ is its own binary. vercel_runtime serves it on VERCEL_DEV_PORT.
VERCEL_DEV_PORT=3001 cargo run --bin health

# 2. Frontend: Trunk serves the app on :8080 and proxies /api/health to :3001.
cd frontend && trunk serve --port 8080
```

Open http://localhost:8080. The home page should say **API: ok**.

## Checks

```sh
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo clippy -p frontend --target wasm32-unknown-unknown -- -D warnings
cargo test
(cd frontend && trunk build --release)
```
