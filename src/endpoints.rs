//! What each endpoint does once its request has been read.

use runner::{PlaygroundClient, parse_diagnostics};
use shared::{CheckResponse, CodeRequest, RunResponse};

use crate::{Failure, check_size};

/// The API's handle on the compile service.
#[derive(Debug, Clone)]
pub struct Api {
    client: PlaygroundClient,
}

impl Api {
    pub fn new(client: PlaygroundClient) -> Self {
        Self { client }
    }

    /// Uses `RUNNER_URL`, falling back to the public Playground.
    pub fn from_env() -> Self {
        Self::new(PlaygroundClient::from_env())
    }

    /// `/api/check`: clippy's diagnostics for live squiggles.
    pub async fn check(&self, req: CodeRequest) -> Result<CheckResponse, Failure> {
        check_size(&req.code)?;
        let out = self.client.clippy(&req.code).await?;
        Ok(CheckResponse {
            ok: out.success,
            diagnostics: parse_diagnostics(&out.stderr),
        })
    }

    /// `/api/run`: builds and runs the program.
    pub async fn run(&self, req: CodeRequest) -> Result<RunResponse, Failure> {
        check_size(&req.code)?;
        let out = self.client.execute(&req.code, false).await?;
        Ok(RunResponse {
            success: out.success,
            diagnostics: parse_diagnostics(&out.stderr),
            stdout: out.stdout,
            stderr: out.stderr,
        })
    }
}
