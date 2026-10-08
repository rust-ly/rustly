use std::sync::LazyLock;

use rustly_api::{Api, read_json, respond};
use vercel_runtime::{Error, Request, Response, ResponseBody, run, service_fn};

static API: LazyLock<Api> = LazyLock::new(Api::from_env);

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

async fn handler(req: Request) -> Result<Response<ResponseBody>, Error> {
    Ok(respond(
        async { API.run(read_json(req).await?).await }.await,
    ))
}
