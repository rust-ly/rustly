//! What each endpoint does once its request has been read.

use runner::playground::Output;
use runner::{PlaygroundClient, assemble_submission, parse_diagnostics, parse_tests};
use shared::{CheckResponse, CodeRequest, RunResponse, SubmitRequest, SubmitResponse};

use crate::cache::Cache;
use crate::{Failure, check_size, redact};

/// The API's handle on the compile service.
#[derive(Debug)]
pub struct Api {
    client: PlaygroundClient,
    cache: Cache,
}

impl Api {
    pub fn new(client: PlaygroundClient) -> Self {
        Self {
            client,
            cache: Cache::default(),
        }
    }

    /// Uses `RUNNER_URL`, falling back to the public Playground.
    pub fn from_env() -> Self {
        Self::new(PlaygroundClient::from_env())
    }

    /// `/api/check`: clippy's diagnostics for live squiggles.
    pub async fn check(&self, req: CodeRequest) -> Result<CheckResponse, Failure> {
        check_size(&req.code)?;
        let out = self.clippy(&req.code).await?;
        Ok(CheckResponse {
            ok: out.success,
            diagnostics: parse_diagnostics(&out.stderr),
        })
    }

    /// `/api/run`: builds and runs the program.
    pub async fn run(&self, req: CodeRequest) -> Result<RunResponse, Failure> {
        check_size(&req.code)?;
        let out = self.execute(&req.code, false).await?;
        Ok(RunResponse {
            success: out.success,
            diagnostics: parse_diagnostics(&out.stderr),
            stdout: out.stdout,
            stderr: out.stderr,
        })
    }

    /// `/api/submit`: grades the code against the exercise's hidden tests.
    ///
    /// The response never includes the tests themselves: diagnostics that
    /// point into them are dropped, and lines rustc quotes from them are
    /// replaced in `stderr`.
    pub async fn submit(&self, req: SubmitRequest) -> Result<SubmitResponse, Failure> {
        check_size(&req.code)?;
        let exercise = content::exercise(&req.exercise_id).ok_or_else(|| {
            Failure::new(
                404,
                format!("There's no exercise called `{}`.", req.exercise_id),
            )
        })?;
        let program = assemble_submission(&req.code, &exercise.hidden_tests);
        let out = self.execute(&program, true).await?;

        let learner_lines = req.code.trim_end().lines().count() as u32;
        let compiled = !out.stderr.contains("error: could not compile");
        let tests = parse_tests(&out.stdout);
        let passed = compiled && !tests.is_empty() && tests.iter().all(|t| t.passed);
        let diagnostics = parse_diagnostics(&out.stderr)
            .into_iter()
            .filter(|d| d.line <= learner_lines)
            .collect();
        Ok(SubmitResponse {
            compiled,
            passed,
            tests,
            diagnostics,
            stderr: redact::hidden_test_lines(&out.stderr, learner_lines),
        })
    }

    async fn clippy(&self, code: &str) -> Result<Output, Failure> {
        if let Some(out) = self.cache.get("clippy", code) {
            return Ok(out);
        }
        let out = self.client.clippy(code).await?;
        self.cache.put("clippy", code, out.clone());
        Ok(out)
    }

    async fn execute(&self, code: &str, tests: bool) -> Result<Output, Failure> {
        let endpoint = if tests { "test" } else { "execute" };
        if let Some(out) = self.cache.get(endpoint, code) {
            return Ok(out);
        }
        let out = self.client.execute(code, tests).await?;
        self.cache.put(endpoint, code, out.clone());
        Ok(out)
    }
}
