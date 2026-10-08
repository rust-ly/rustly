//! A small client for the Rust Playground's `/execute` and `/clippy` endpoints.
//!
//! The Playground has no formal API contract. The field names here were checked
//! against play.rust-lang.org; note that `/execute` answers with `exitDetail` and
//! `/clippy` with `exit_detail`.

use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};

pub const DEFAULT_URL: &str = "https://play.rust-lang.org";
const TIMEOUT: Duration = Duration::from_secs(15);
const USER_AGENT: &str = concat!(
    "rustly/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/rust-ly/rustly)"
);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExecuteRequest<'a> {
    channel: &'static str,
    mode: &'static str,
    edition: &'static str,
    crate_type: &'static str,
    tests: bool,
    backtrace: bool,
    code: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClippyRequest<'a> {
    channel: &'static str,
    edition: &'static str,
    crate_type: &'static str,
    code: &'a str,
}

/// What the Playground sends back from a build or run.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Output {
    /// False for compile errors, panics and failing tests.
    pub success: bool,
    #[serde(rename = "exitDetail", alias = "exit_detail", default)]
    pub exit_detail: String,
    /// Program output, or the test report when tests are run.
    pub stdout: String,
    /// Cargo and rustc output, plus anything the program writes to stderr.
    pub stderr: String,
}

#[derive(Debug)]
pub enum Error {
    /// The Playground didn't answer within the timeout.
    Timeout,
    /// The Playground answered with a non-success HTTP status.
    Status(u16),
    /// The request couldn't be sent or the response couldn't be read.
    Request(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Timeout => write!(f, "the compile service timed out"),
            Error::Status(code) => write!(f, "the compile service returned HTTP {code}"),
            Error::Request(e) => write!(f, "couldn't reach the compile service: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            Error::Timeout
        } else if let Some(status) = e.status() {
            Error::Status(status.as_u16())
        } else {
            Error::Request(e)
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlaygroundClient {
    http: reqwest::Client,
    base_url: String,
}

impl PlaygroundClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let http = reqwest::Client::builder()
            .timeout(TIMEOUT)
            .user_agent(USER_AGENT)
            .build()
            .expect("static reqwest config is valid");
        let base_url = base_url.into().trim_end_matches('/').to_string();
        Self { http, base_url }
    }

    /// Uses `RUNNER_URL` if it's set, so a self-hosted runner can replace the public one.
    pub fn from_env() -> Self {
        Self::new(std::env::var("RUNNER_URL").unwrap_or_else(|_| DEFAULT_URL.to_string()))
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Builds and runs `code` as a binary. With `tests`, runs its `#[test]`s instead of `main`.
    pub async fn execute(&self, code: &str, tests: bool) -> Result<Output, Error> {
        self.post("execute", &execute_request(code, tests)).await
    }

    /// Runs clippy on `code`. Compile errors come back here too.
    pub async fn clippy(&self, code: &str) -> Result<Output, Error> {
        self.post("clippy", &clippy_request(code)).await
    }

    async fn post(&self, endpoint: &str, body: &impl Serialize) -> Result<Output, Error> {
        let url = format!("{}/{endpoint}", self.base_url);
        let response = self
            .http
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;
        Ok(response.json().await?)
    }
}

fn execute_request(code: &str, tests: bool) -> ExecuteRequest<'_> {
    ExecuteRequest {
        channel: "stable",
        mode: "debug",
        edition: "2024",
        crate_type: "bin",
        tests,
        backtrace: false,
        code,
    }
}

fn clippy_request(code: &str) -> ClippyRequest<'_> {
    ClippyRequest {
        channel: "stable",
        edition: "2024",
        crate_type: "bin",
        code,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn execute_request_matches_playground_fields() {
        let body = serde_json::to_value(execute_request("fn main() {}", true)).unwrap();
        assert_eq!(
            body,
            json!({
                "channel": "stable",
                "mode": "debug",
                "edition": "2024",
                "crateType": "bin",
                "tests": true,
                "backtrace": false,
                "code": "fn main() {}",
            })
        );
    }

    #[test]
    fn clippy_request_matches_playground_fields() {
        let body = serde_json::to_value(clippy_request("fn main() {}")).unwrap();
        assert_eq!(
            body,
            json!({"channel": "stable", "edition": "2024", "crateType": "bin", "code": "fn main() {}"})
        );
    }

    #[test]
    fn reads_both_exit_detail_spellings() {
        let execute: Output = serde_json::from_str(
            r#"{"success":true,"exitDetail":"Exited with status 0","stdout":"hi\n","stderr":""}"#,
        )
        .unwrap();
        let clippy: Output = serde_json::from_str(
            r#"{"success":false,"exit_detail":"Exited with status 101","stdout":"","stderr":"e"}"#,
        )
        .unwrap();
        assert_eq!(execute.exit_detail, "Exited with status 0");
        assert_eq!(clippy.exit_detail, "Exited with status 101");
    }

    #[test]
    fn trims_trailing_slash_from_base_url() {
        assert_eq!(
            PlaygroundClient::new("http://localhost:5000/").base_url(),
            "http://localhost:5000"
        );
    }
}
