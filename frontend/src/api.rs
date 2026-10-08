//! Calls to the API functions in `api/`.

use gloo_net::http::Request;
use shared::{ApiError, CodeRequest, RunResponse};

/// `POST /api/run`. The error is a message ready to show the learner.
pub async fn run(code: String) -> Result<RunResponse, String> {
    let response = Request::post("/api/run")
        .json(&CodeRequest { code })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|_| {
            "Couldn't reach the Rustly API. Check your connection and try again.".to_string()
        })?;
    if response.ok() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        let status = response.status();
        Err(response
            .json::<ApiError>()
            .await
            .map(|e| e.error)
            .unwrap_or_else(|_| format!("The API answered with HTTP {status}.")))
    }
}
