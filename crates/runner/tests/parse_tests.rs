//! `parse_tests` against test reports captured from play.rust-lang.org.

use runner::parse_tests;
use shared::TestCase;

#[test]
fn all_tests_pass() {
    assert_eq!(
        parse_tests(include_str!("fixtures/tests_pass.stdout")),
        vec![
            TestCase {
                name: "greets_ada".into(),
                passed: true,
                output: None
            },
            TestCase {
                name: "greets_anyone".into(),
                passed: true,
                output: None
            },
        ]
    );
}

#[test]
fn failing_assertion_and_panic() {
    assert_eq!(
        parse_tests(include_str!("fixtures/tests_mixed.stdout")),
        vec![
            TestCase {
                name: "adds_small_numbers".into(),
                passed: true,
                output: None
            },
            TestCase {
                name: "adds_big_numbers".into(),
                passed: false,
                output: Some("assertion `left == right` failed\n  left: 43\n right: 42".into()),
            },
            TestCase {
                name: "panics_inside".into(),
                passed: false,
                output: Some("index out of bounds: the len is 0 but the index is 0".into()),
            },
        ]
    );
}

#[test]
fn a_build_that_never_ran_tests() {
    assert!(parse_tests(include_str!("fixtures/one_error.stdout")).is_empty());
}
