//! Reading JSON requests and writing JSON responses.

use http_body_util::{BodyExt, Limited};
use serde::Serialize;
use serde::de::DeserializeOwned;
use shared::ApiError;
use vercel_runtime::{Request, Response, ResponseBody};

use crate::Failure;

/// Largest request body read, in bytes. JSON escaping can make a body bigger
/// than the code inside it, so this sits well above [`crate::MAX_CODE_BYTES`].
const MAX_BODY_BYTES: usize = 64 * 1024;

/// Reads a POST request's JSON body.
pub async fn read_json<T: DeserializeOwned>(req: Request) -> Result<T, Failure> {
    if req.method() != "POST" {
        return Err(Failure::new(405, "Use POST for this endpoint."));
    }
    let bytes = Limited::new(req.into_body(), MAX_BODY_BYTES)
        .collect()
        .await
        .map_err(|_| Failure::too_large())?
        .to_bytes();
    parse_json(&bytes)
}

fn parse_json<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Failure> {
    serde_json::from_slice(bytes)
        .map_err(|e| Failure::new(400, format!("The request body isn't valid: {e}")))
}

/// Writes `result` as JSON: the value with 200, or an [`ApiError`] with the
/// failure's status.
pub fn respond<T: Serialize>(result: Result<T, Failure>) -> Response<ResponseBody> {
    let (status, body) = match result {
        Ok(value) => (200, serde_json::to_string(&value)),
        Err(failure) => (
            failure.status,
            serde_json::to_string(&ApiError {
                error: failure.message,
            }),
        ),
    };
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body.expect("API types always serialize").into())
        .expect("status and header are valid")
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::CodeRequest;

    #[test]
    fn parses_a_valid_body() {
        let req: CodeRequest = parse_json(br#"{"code":"fn main() {}"}"#).unwrap();
        assert_eq!(req.code, "fn main() {}");
    }

    #[test]
    fn rejects_a_malformed_body_with_400() {
        let err = parse_json::<CodeRequest>(br#"{"source":1}"#).unwrap_err();
        assert_eq!(err.status, 400);
    }

    #[tokio::test]
    async fn failures_become_api_error_json() {
        let response = respond::<()>(Err(Failure::new(404, "No such exercise.")));
        assert_eq!(response.status(), 404);
        assert_eq!(response.headers()["content-type"], "application/json");
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let error: ApiError = serde_json::from_slice(&body).unwrap();
        assert_eq!(error.error, "No such exercise.");
    }
}
