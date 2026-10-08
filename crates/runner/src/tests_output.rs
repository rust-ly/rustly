//! Turns the libtest report from a `tests: true` run into [`TestCase`]s.

use shared::TestCase;

/// Parses `test tests::name ... ok|FAILED` lines, and attaches each failing
/// test's output block (`---- tests::name stdout ----`) to it.
///
/// Names lose their `tests::` prefix. Ignored tests are left out.
pub fn parse_tests(stdout: &str) -> Vec<TestCase> {
    stdout
        .lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("test ")?;
            let (name, result) = rest.rsplit_once(" ... ")?;
            let passed = match result {
                "ok" => true,
                "FAILED" => false,
                _ => return None,
            };
            Some(TestCase {
                name: display_name(name).to_string(),
                passed,
                output: if passed {
                    None
                } else {
                    failure_output(stdout, name)
                },
            })
        })
        .collect()
}

fn display_name(name: &str) -> &str {
    name.strip_prefix("tests::").unwrap_or(name)
}

/// The panic message for one failing test, without the `thread ... panicked at`
/// line (it points into the hidden tests) or the backtrace hint.
fn failure_output(stdout: &str, name: &str) -> Option<String> {
    let header = format!("---- {name} stdout ----");
    let start = stdout.find(&header)? + header.len();
    let block = &stdout[start..];
    let end = block
        .find("\n---- ")
        .or_else(|| block.find("\nfailures:\n"))
        .unwrap_or(block.len());
    let output = block[..end]
        .lines()
        .filter(|l| !(l.starts_with("thread '") && l.contains(" panicked at ")))
        .filter(|l| !l.starts_with("note: run with `RUST_BACKTRACE=1`"))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();
    (!output.is_empty()).then_some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_ignored_tests_and_other_lines() {
        let stdout = "running 2 tests\ntest tests::slow ... ignored\ntest tests::fast ... ok\n\ntest result: ok.";
        let tests = parse_tests(stdout);
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].name, "fast");
    }

    #[test]
    fn keeps_names_outside_the_tests_module() {
        assert_eq!(parse_tests("test check_it ... ok")[0].name, "check_it");
    }

    #[test]
    fn no_report_means_no_tests() {
        assert!(parse_tests("").is_empty());
    }
}
