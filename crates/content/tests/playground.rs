//! Runs every exercise and snippet through the real compile service. Ignored
//! by default to stay polite to the public Playground: run with
//! `cargo test -p content --features server -- --ignored`.
#![cfg(feature = "server")]

use content::{Exercise, SnippetKind};
use runner::{PlaygroundClient, assemble_submission, parse_tests};

/// Grades `code` against the exercise's hidden tests: `Some(all passed)`, or
/// `None` if nothing ran.
async fn grade(client: &PlaygroundClient, ex: &Exercise, code: &str) -> Option<bool> {
    let out = client
        .execute(&assemble_submission(code, &ex.hidden_tests), true)
        .await
        .unwrap();
    let tests = parse_tests(&out.stdout);
    (!tests.is_empty()).then(|| tests.iter().all(|t| t.passed))
}

#[tokio::test]
#[ignore]
async fn every_solution_passes_and_every_starter_fails() {
    let client = PlaygroundClient::from_env();
    let mut failures = Vec::new();
    for ex in content::course().exercises() {
        if grade(&client, ex, &ex.solution).await != Some(true) {
            failures.push(format!("{}: solution doesn't pass", ex.id));
        }
        if grade(&client, ex, &ex.starter).await == Some(true) {
            failures.push(format!("{}: starter passes", ex.id));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[tokio::test]
#[ignore]
async fn every_snippet_behaves_as_its_fence_says() {
    let client = PlaygroundClient::from_env();
    let mut failures = Vec::new();
    for snippet in content::snippets() {
        if snippet.kind == SnippetKind::Static {
            continue;
        }
        let out = client.execute(&snippet.code, false).await.unwrap();
        let compiled = !out.stderr.contains("error: could not compile");
        let ok = match snippet.kind {
            SnippetKind::Static => true,
            SnippetKind::Runnable | SnippetKind::Editable => out.success,
            SnippetKind::DoesNotCompile => !compiled,
            SnippetKind::Panics => compiled && out.stderr.contains("panicked at"),
        };
        if !ok {
            failures.push(format!(
                "{} ({:?}):\n{}",
                snippet.id, snippet.kind, out.stderr
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}
