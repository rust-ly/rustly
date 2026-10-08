mod common;

use common::FakePlayground;
use rustly_api::{Api, MAX_CODE_BYTES};
use shared::{CodeRequest, Severity};

const ONE_ERROR: &str = include_str!("../crates/runner/tests/fixtures/one_error.stderr");
const HELLO_STDOUT: &str = include_str!("../crates/runner/tests/fixtures/hello.stdout");

fn code(code: &str) -> CodeRequest {
    CodeRequest { code: code.into() }
}

#[tokio::test]
async fn check_asks_clippy_and_returns_diagnostics() {
    let playground = FakePlayground::answering(false, "", ONE_ERROR).await;
    let api = Api::new(playground.client());

    let res = api.check(code("fn main() {}")).await.unwrap();

    assert!(!res.ok);
    assert_eq!(res.diagnostics[0].severity, Severity::Error);
    assert_eq!(res.diagnostics[0].code.as_deref(), Some("E0382"));
    let sent = playground.received();
    assert_eq!(sent[0].0, "/clippy");
    assert_eq!(sent[0].1["code"], "fn main() {}");
}

#[tokio::test]
async fn run_executes_without_tests() {
    let playground = FakePlayground::answering(true, HELLO_STDOUT, "").await;
    let api = Api::new(playground.client());

    let res = api.run(code("fn main() {}")).await.unwrap();

    assert!(res.success);
    assert_eq!(res.stdout, HELLO_STDOUT);
    assert!(res.diagnostics.is_empty());
    let sent = playground.received();
    assert_eq!(sent[0].0, "/execute");
    assert_eq!(sent[0].1["tests"], false);
}

#[tokio::test]
async fn oversized_code_is_413_and_never_sent() {
    let playground = FakePlayground::answering(true, "", "").await;
    let api = Api::new(playground.client());

    let big = "a".repeat(MAX_CODE_BYTES + 1);
    assert_eq!(api.check(code(&big)).await.unwrap_err().status, 413);
    assert_eq!(api.run(code(&big)).await.unwrap_err().status, 413);
    assert!(playground.received().is_empty());
}

#[tokio::test]
async fn compile_service_5xx_is_502() {
    let playground = FakePlayground::with_status(503, "busy".into()).await;
    let api = Api::new(playground.client());

    let err = api.run(code("fn main() {}")).await.unwrap_err();

    assert_eq!(err.status, 502);
    assert!(err.message.contains("compile service"));
}

#[tokio::test]
async fn unreachable_compile_service_is_502() {
    // Nothing listens on port 9 (discard) on a dev machine or CI runner.
    let api = Api::new(runner::PlaygroundClient::new("http://127.0.0.1:9"));
    assert_eq!(
        api.check(code("fn main() {}")).await.unwrap_err().status,
        502
    );
}
