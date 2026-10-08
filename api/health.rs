use serde_json::{Value, json};
use vercel_runtime::{Error, Request, run, service_fn};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

async fn handler(_req: Request) -> Result<Value, Error> {
    let runner_url =
        std::env::var("RUNNER_URL").unwrap_or_else(|_| "https://play.rust-lang.org".into());
    Ok(json!({ "ok": true, "runner_url": runner_url }))
}
