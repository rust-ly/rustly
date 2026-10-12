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

impl Diagnostic {
    /// The official page explaining this diagnostic's code, if it has one.
    pub fn explain_url(&self) -> Option<String> {
        explain_url(self.code.as_deref()?)
    }
}

/// Where the Rust docs explain `code`: error codes like `E0382` have a page in
/// the error index, and Clippy lints have an entry in Clippy's lint list.
pub fn explain_url(code: &str) -> Option<String> {
    if let Some(digits) = code.strip_prefix('E')
        && digits.len() == 4
        && digits.bytes().all(|b| b.is_ascii_digit())
    {
        return Some(format!("https://doc.rust-lang.org/error_codes/{code}.html"));
    }
    let lint = code.strip_prefix("clippy::")?;
    (!lint.is_empty()
        && lint
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_'))
    .then(|| format!("https://rust-lang.github.io/rust-clippy/stable/index.html#{lint}"))
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

/// Response of `/api/format`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormatResponse {
    /// False when rustfmt couldn't parse the code; `code` is then unchanged.
    pub formatted: bool,
    pub code: String,
    /// Why rustfmt couldn't parse the code.
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

/// Failed submits after which an exercise's solution can be shown.
pub const SOLUTION_AFTER_FAILS: u32 = 3;

/// Body of `/api/solution`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionRequest {
    pub exercise_id: String,
    /// The learner's failed submits for this exercise, from their progress.
    pub failed_submits: u32,
}

/// Response of `/api/solution`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionResponse {
    pub solution: String,
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
    fn error_codes_link_to_the_error_index() {
        assert_eq!(
            explain_url("E0382").as_deref(),
            Some("https://doc.rust-lang.org/error_codes/E0382.html")
        );
    }

    #[test]
    fn clippy_lints_link_to_the_lint_list() {
        assert_eq!(
            explain_url("clippy::len_zero").as_deref(),
            Some("https://rust-lang.github.io/rust-clippy/stable/index.html#len_zero")
        );
    }

    #[test]
    fn other_codes_have_no_link() {
        for code in [
            "unused_variables",
            "E038",
            "E03822",
            "Eabcd",
            "clippy::",
            "clippy::a#b",
        ] {
            assert_eq!(explain_url(code), None, "{code}");
        }
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
