//! Real calls to the compile service. Ignored by default to stay polite to the
//! public Playground: run with `cargo test -p runner -- --ignored`.

use runner::PlaygroundClient;

#[tokio::test]
#[ignore]
async fn runs_hello_world() {
    let out = PlaygroundClient::from_env()
        .execute("fn main() {\n    println!(\"Hello, world!\");\n}\n", false)
        .await
        .unwrap();
    assert!(out.success, "{}", out.stderr);
    assert_eq!(out.stdout, "Hello, world!\n");
}

#[tokio::test]
#[ignore]
async fn clippy_reports_lints() {
    let out = PlaygroundClient::from_env()
        .clippy("fn main() {\n    let v = vec![1];\n    if v.len() == 0 {}\n}\n")
        .await
        .unwrap();
    assert!(out.stderr.contains("clippy::len_zero"), "{}", out.stderr);
}

#[tokio::test]
#[ignore]
async fn grades_a_submission() {
    let code =
        "fn greet(name: &str) -> String {\n    format!(\"Hello, {name}!\")\n}\n\nfn main() {}\n";
    let hidden_tests = "#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn greets_ada() {\n        assert_eq!(greet(\"Ada\"), \"Hello, Ada!\");\n    }\n}\n";
    let out = PlaygroundClient::from_env()
        .execute(&runner::assemble_submission(code, hidden_tests), true)
        .await
        .unwrap();
    let tests = runner::parse_tests(&out.stdout);
    assert_eq!(tests.len(), 1, "{}", out.stdout);
    assert!(tests[0].passed);
}
