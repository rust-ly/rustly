/// Builds the program that grades an exercise: the learner's code with the
/// hidden tests appended. The tests always go after the learner's code, so
/// line numbers in diagnostics still match what the learner sees.
pub fn assemble_submission(code: &str, hidden_tests: &str) -> String {
    format!("{}\n\n{}\n", code.trim_end(), hidden_tests.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_tests_after_the_code() {
        let code = "fn one() -> i32 {\n    1\n}\n\n\n";
        let tests = "\n#[test]\nfn is_one() {\n    assert_eq!(one(), 1);\n}\n";
        assert_eq!(
            assemble_submission(code, tests),
            "fn one() -> i32 {\n    1\n}\n\n#[test]\nfn is_one() {\n    assert_eq!(one(), 1);\n}\n"
        );
    }

    #[test]
    fn keeps_learner_line_numbers() {
        let code = "\n\nfn main() {}";
        let assembled = assemble_submission(code, "#[test]\nfn t() {}");
        assert!(assembled.starts_with(code));
    }
}
