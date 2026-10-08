//! `parse_diagnostics` against stderr captured from play.rust-lang.org.
//! The source that produced each fixture sits next to it as `<name>.rs`.

use runner::parse_diagnostics;
use shared::{Diagnostic, Severity};

fn diag(
    severity: Severity,
    code: Option<&str>,
    message: &str,
    line: u32,
    col: u32,
    width: u32,
) -> Diagnostic {
    Diagnostic {
        severity,
        code: code.map(str::to_string),
        message: message.to_string(),
        line,
        col,
        end_line: line,
        end_col: col + width,
    }
}

#[test]
fn clean_build() {
    assert!(parse_diagnostics(include_str!("fixtures/hello.stderr")).is_empty());
}

#[test]
fn one_error_and_a_warning() {
    assert_eq!(
        parse_diagnostics(include_str!("fixtures/one_error.stderr")),
        vec![
            diag(
                Severity::Error,
                Some("E0382"),
                "borrow of moved value: `s1`",
                4,
                16,
                2
            ),
            diag(
                Severity::Warning,
                Some("unused_variables"),
                "unused variable: `s2`",
                3,
                9,
                2
            ),
        ]
    );
}

#[test]
fn several_errors_with_notes_and_suggestions() {
    assert_eq!(
        parse_diagnostics(include_str!("fixtures/many_errors.stderr")),
        vec![
            diag(
                Severity::Error,
                Some("E0425"),
                "cannot find value `y` in this scope",
                6,
                20,
                1
            ),
            diag(Severity::Error, Some("E0308"), "mismatched types", 3, 18, 6),
            diag(Severity::Error, Some("E0308"), "mismatched types", 7, 11, 5),
        ]
    );
}

#[test]
fn errors_and_warnings_together() {
    assert_eq!(
        parse_diagnostics(include_str!("fixtures/errors_warnings.stderr")),
        vec![
            diag(
                Severity::Error,
                Some("E0502"),
                "cannot borrow `v` as mutable because it is also borrowed as immutable",
                7,
                5,
                9
            ),
            diag(
                Severity::Error,
                Some("E0382"),
                "borrow of moved value: `s`",
                8,
                16,
                1
            ),
            diag(
                Severity::Warning,
                Some("unused_variables"),
                "unused variable: `unused`",
                2,
                9,
                6
            ),
            diag(Severity::Warning, None, "unused variable: `t`", 4, 9, 1),
        ]
    );
}

#[test]
fn clippy_lints() {
    assert_eq!(
        parse_diagnostics(include_str!("fixtures/clippy_lint.stderr")),
        vec![
            diag(
                Severity::Warning,
                Some("clippy::len_zero"),
                "length comparison to zero",
                3,
                8,
                12
            ),
            diag(
                Severity::Warning,
                Some("clippy::useless_vec"),
                "useless use of `vec!`",
                2,
                13,
                13
            ),
        ]
    );
}

#[test]
fn a_panic_is_not_a_compile_diagnostic() {
    assert!(parse_diagnostics(include_str!("fixtures/panic.stderr")).is_empty());
}
