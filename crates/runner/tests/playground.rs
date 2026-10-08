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
