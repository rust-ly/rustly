//! `/api/submit` must never send back an exercise's hidden tests. (It never
//! sends the solution anywhere; rustc may still suggest the same fix itself.)

mod common;

use common::FakePlayground;
use rustly_api::Api;
use shared::{SubmitRequest, SubmitResponse};

/// Lines of `secret` long enough to be recognisable that aren't also in the
/// learner's code.
fn secret_lines<'a>(secret: &'a str, learner_code: &str) -> Vec<&'a str> {
    secret
        .lines()
        .map(str::trim)
        .filter(|l| l.len() >= 10 && !learner_code.contains(l))
        .collect()
}

/// Everything in a response that comes from the compiler: what rustc printed
/// and the diagnostics parsed from it. A failing assertion's message is
/// feedback the learner is meant to see, so test output isn't checked here.
fn compiler_text(res: &SubmitResponse) -> String {
    let mut text = res.stderr.clone();
    for d in &res.diagnostics {
        text.push('\n');
        text.push_str(&d.message);
    }
    text
}

fn assert_no_leak(id: &str, learner_code: &str, res: &SubmitResponse) {
    let exercise = content::exercise(id).unwrap();
    let text = compiler_text(res);
    for line in secret_lines(&exercise.hidden_tests, learner_code) {
        assert!(!text.contains(line), "{id}: response contains {line:?}");
    }
}

/// The worst case rustc can produce: an error pointing at every line of the
/// hidden tests, with the line quoted and a suggested fix showing it again.
fn quote_every_test_line(code: &str, hidden_tests: &str) -> String {
    let first = code.trim_end().lines().count() + 2;
    let mut stderr = String::from("   Compiling playground v0.0.1 (/playground)\n");
    for (i, line) in hidden_tests.trim().lines().enumerate() {
        let n = first + i;
        stderr.push_str(&format!(
            "error[E0308]: mismatched types\n  --> src/main.rs:{n}:5\n   |\n{n} | {line}\n   |     ^^^ expected `&str`\nhelp: try this\n   |\n{n} - {line}\n{n} + {line}\n   |\n\n"
        ));
    }
    stderr.push_str("error: could not compile `playground` (bin \"playground\" test)\n");
    stderr
}

#[tokio::test]
async fn compiler_output_never_quotes_hidden_tests() {
    for exercise in content::course().exercises() {
        let stderr = quote_every_test_line(&exercise.starter, &exercise.hidden_tests);
        let playground = FakePlayground::answering(false, "", &stderr).await;
        let api = Api::new(playground.client());

        let res = api
            .submit(SubmitRequest {
                exercise_id: exercise.id.clone(),
                code: exercise.starter.clone(),
            })
            .await
            .unwrap();

        // The fake really did quote the tests, so this checks the redaction.
        assert!(stderr.contains(exercise.hidden_tests.trim().lines().last().unwrap()));
        assert!(!res.compiled);
        assert!(res.diagnostics.is_empty(), "{}", exercise.id);
        assert_no_leak(&exercise.id, &exercise.starter, &res);
    }
}

/// The same check against the real Playground, for every starter. Ignored by
/// default to stay polite to the public service: run with
/// `cargo test -p rustly-api -- --ignored`.
#[tokio::test]
#[ignore]
async fn real_submissions_never_quote_hidden_tests() {
    let api = Api::from_env();
    for exercise in content::course().exercises() {
        let res = api
            .submit(SubmitRequest {
                exercise_id: exercise.id.clone(),
                code: exercise.starter.clone(),
            })
            .await
            .unwrap();
        assert!(!res.passed, "{}: starter passes", exercise.id);
        assert_no_leak(&exercise.id, &exercise.starter, &res);
    }
}
