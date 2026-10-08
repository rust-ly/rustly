//! Request and response types shared by the API functions and the frontend.
//!
//! This crate must compile for both native and `wasm32-unknown-unknown`, so it
//! only depends on `serde`.

use serde::{Deserialize, Serialize};

/// Body of `/api/check` and `/api/run`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeRequest {
    pub code: String,
}

/// Body of `/api/submit`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmitRequest {
    pub exercise_id: String,
    pub code: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Note,
}

/// One compiler message, positioned in the learner's code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: Severity,
    /// Error code or lint name, e.g. `E0382` or `clippy::len_zero`.
    pub code: Option<String>,
    pub message: String,
    /// 1-based line and column where the problem starts.
    pub line: u32,
    pub col: u32,
    /// 1-based end position, exclusive.
    pub end_line: u32,
    pub end_col: u32,
}

/// Response of `/api/check`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckResponse {
    pub ok: bool,
    pub diagnostics: Vec<Diagnostic>,
}

/// Response of `/api/run`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunResponse {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub diagnostics: Vec<Diagnostic>,
}

/// One hidden test and whether it passed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub passed: bool,
    /// Panic message and assertion details for a failing test.
    pub output: Option<String>,
}

/// Response of `/api/submit`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmitResponse {
    pub compiled: bool,
    pub passed: bool,
    pub tests: Vec<TestCase>,
    pub diagnostics: Vec<Diagnostic>,
    pub stderr: String,
}

/// Body of every non-2xx API response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_is_lowercase_in_json() {
        assert_eq!(
            serde_json::to_string(&Severity::Warning).unwrap(),
            r#""warning""#
        );
    }

    #[test]
    fn diagnostic_round_trips() {
        let d = Diagnostic {
            severity: Severity::Error,
            code: Some("E0382".into()),
            message: "borrow of moved value: `s1`".into(),
            line: 4,
            col: 16,
            end_line: 4,
            end_col: 18,
        };
        let json = serde_json::to_string(&d).unwrap();
        assert_eq!(serde_json::from_str::<Diagnostic>(&json).unwrap(), d);
    }

    #[test]
    fn submit_request_uses_snake_case_fields() {
        let req: SubmitRequest = serde_json::from_str(
            r#"{"exercise_id":"book.ownership.challenge","code":"fn main() {}"}"#,
        )
        .unwrap();
        assert_eq!(req.exercise_id, "book.ownership.challenge");
    }
}
