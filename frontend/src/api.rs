//! Calls to the API functions in `api/`.

use gloo_net::http::Request;
use serde::Serialize;
use serde::de::DeserializeOwned;
use shared::{
    ApiError, CodeRequest, RunResponse, SolutionRequest, SolutionResponse, SubmitRequest,
    SubmitResponse,
};

/// `POST /api/run`. The error is a message ready to show the learner.
pub async fn run(code: String) -> Result<RunResponse, String> {
    post("/api/run", &CodeRequest { code }).await
}

/// `POST /api/submit`: grades the code against the exercise's hidden tests.
pub async fn submit(exercise_id: String, code: String) -> Result<SubmitResponse, String> {
    post("/api/submit", &SubmitRequest { exercise_id, code }).await
}

/// `POST /api/solution`, once the learner has failed enough submits.
pub async fn solution(exercise_id: String, failed_submits: u32) -> Result<String, String> {
    let request = SolutionRequest {
        exercise_id,
        failed_submits,
    };
    post::<_, SolutionResponse>("/api/solution", &request)
        .await
        .map(|r| r.solution)
}

async fn post<B: Serialize, R: DeserializeOwned>(url: &str, body: &B) -> Result<R, String> {
    let response = Request::post(url)
        .json(body)
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
