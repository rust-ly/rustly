mod common;

use common::FakePlayground;
use rustly_api::{Api, MAX_CODE_BYTES};
use shared::{CodeRequest, Severity, SubmitRequest};

const ONE_ERROR: &str = include_str!("../crates/runner/tests/fixtures/one_error.stderr");
const HELLO_STDOUT: &str = include_str!("../crates/runner/tests/fixtures/hello.stdout");
const TESTS_PASS: &str = include_str!("../crates/runner/tests/fixtures/tests_pass.stdout");
const TESTS_MIXED: &str = include_str!("../crates/runner/tests/fixtures/tests_mixed.stdout");
/// Real Playground output for the ownership challenge's starter, which fails
/// to compile partly because of how the hidden tests call it.
const OWNERSHIP_STARTER: &str = include_str!("fixtures/ownership_starter.stderr");
const OWNERSHIP: &str = "book.ownership.challenge";

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

fn submission(exercise_id: &str, code: &str) -> SubmitRequest {
    SubmitRequest {
        exercise_id: exercise_id.into(),
        code: code.into(),
    }
}

#[tokio::test]
async fn submit_runs_the_hidden_tests_after_the_code() {
    let playground = FakePlayground::answering(true, TESTS_PASS, "").await;
    let api = Api::new(playground.client());
    let exercise = content::exercise(OWNERSHIP).unwrap();

    let res = api
        .submit(submission(OWNERSHIP, &exercise.solution))
        .await
        .unwrap();

    assert!(res.compiled && res.passed);
    let sent = playground.received();
    assert_eq!(sent[0].0, "/execute");
    assert_eq!(sent[0].1["tests"], true);
    let program = sent[0].1["code"].as_str().unwrap();
    assert!(program.starts_with(exercise.solution.trim_end()));
    assert!(program.trim_end().ends_with(exercise.hidden_tests.trim()));
}

#[tokio::test]
async fn submit_with_a_failing_test_is_not_passed() {
    let playground = FakePlayground::answering(false, TESTS_MIXED, "").await;
    let api = Api::new(playground.client());

    let res = api
        .submit(submission(OWNERSHIP, "fn main() {}"))
        .await
        .unwrap();

    assert!(res.compiled);
    assert!(!res.passed);
    assert_eq!(res.tests.iter().filter(|t| !t.passed).count(), 2);
}

#[tokio::test]
async fn submit_that_does_not_compile_drops_hidden_test_locations() {
    let playground = FakePlayground::answering(false, "", OWNERSHIP_STARTER).await;
    let api = Api::new(playground.client());
    let starter = &content::exercise(OWNERSHIP).unwrap().starter;
    let learner_lines = starter.trim_end().lines().count() as u32;

    let res = api.submit(submission(OWNERSHIP, starter)).await.unwrap();

    assert!(!res.compiled && !res.passed);
    // The error in `main` stays; the two inside the hidden tests are dropped.
    assert!(res.diagnostics.iter().any(|d| d.line == 12));
    assert!(res.diagnostics.iter().all(|d| d.line <= learner_lines));
    assert!(!res.stderr.contains("sign(&mut note, &name)"));
    assert!(res.stderr.contains("41 | (hidden test)"));
}

#[tokio::test]
async fn unknown_exercise_is_404_and_never_sent() {
    let playground = FakePlayground::answering(true, "", "").await;
    let api = Api::new(playground.client());

    let err = api
        .submit(submission("book.nope", "fn main() {}"))
        .await
        .unwrap_err();

    assert_eq!(err.status, 404);
    assert!(playground.received().is_empty());
}

#[tokio::test]
async fn oversized_submission_is_413() {
    let playground = FakePlayground::answering(true, "", "").await;
    let api = Api::new(playground.client());
    let big = "a".repeat(MAX_CODE_BYTES + 1);
    assert_eq!(
        api.submit(submission(OWNERSHIP, &big))
            .await
            .unwrap_err()
            .status,
        413
    );
}
