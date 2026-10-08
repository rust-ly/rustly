//! Keeps hidden tests out of compiler output.
//!
//! When a submission doesn't compile because of how the hidden tests call the
//! learner's code, rustc quotes the test lines in its output:
//!
//! ```text
//! 41 |         sign(&mut note, &name);
//! 41 -         sign(&mut note, &name);
//! ```
//!
//! The tests are appended after the learner's code, so any quoted line past
//! the learner's last line is a test line.

/// Shown in place of a quoted hidden-test line.
pub const PLACEHOLDER: &str = "(hidden test)";

/// Replaces every source line rustc quotes from past `learner_lines`.
pub fn hidden_test_lines(stderr: &str, learner_lines: u32) -> String {
    let mut out = String::with_capacity(stderr.len());
    for line in stderr.split_inclusive('\n') {
        match quoted_line_number(line) {
            Some((n, gutter)) if n > learner_lines => {
                out.push_str(gutter);
                out.push(' ');
                out.push_str(PLACEHOLDER);
                out.push('\n');
            }
            _ => out.push_str(line),
        }
    }
    out
}

/// For a quoted source line like ` 41 |  code`, `41 -  code` or `41 +  code`,
/// returns the line number and the gutter (`" 41 |"`).
fn quoted_line_number(line: &str) -> Option<(u32, &str)> {
    let digits_start = line.len() - line.trim_start().len();
    let rest = &line[digits_start..];
    let digits = rest.len() - rest.trim_start_matches(|c: char| c.is_ascii_digit()).len();
    if digits == 0 {
        return None;
    }
    let n = rest[..digits].parse().ok()?;
    let after = &rest[digits..];
    let marker_at = after.len() - after.trim_start_matches(' ').len();
    match after[marker_at..].chars().next() {
        Some('|' | '-' | '+') => Some((n, &line[..digits_start + digits + marker_at + 1])),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_quoted_lines_past_the_learner_code() {
        let stderr = "  --> src/main.rs:41:9\n   |\n41 |         sign(&mut note, &name);\n   |         ^^^^\n";
        assert_eq!(
            hidden_test_lines(stderr, 20),
            "  --> src/main.rs:41:9\n   |\n41 | (hidden test)\n   |         ^^^^\n"
        );
    }

    #[test]
    fn replaces_suggestion_diffs_too() {
        let stderr = "41 -         sign(&mut note, &name);\n41 +         sign(note, &name);\n";
        assert_eq!(
            hidden_test_lines(stderr, 20),
            "41 - (hidden test)\n41 + (hidden test)\n"
        );
    }

    #[test]
    fn keeps_the_learner_lines() {
        let stderr = "12 |     sign(&mut note, \"Ada\");\n 5 | fn sign(note: String) {\n";
        assert_eq!(hidden_test_lines(stderr, 20), stderr);
    }

    #[test]
    fn ignores_lines_that_only_start_with_a_number() {
        let stderr = "42 tests ran\n";
        assert_eq!(hidden_test_lines(stderr, 1), stderr);
    }
}
