//! Turns rustc's human-readable output into [`Diagnostic`]s.
//!
//! A diagnostic starts with a header at column 0 such as
//! `error[E0382]: borrow of moved value: `s1`` and is followed by its primary
//! location, ` --> src/main.rs:4:16`. Child `help:` and `note:` blocks, and
//! summary lines like `error: could not compile ...`, are not diagnostics of
//! their own.

use shared::{Diagnostic, Severity};

/// The file the learner's code lives in on the compile service.
const LEARNER_FILE: &str = "src/main.rs";

/// Parses every error and warning that points into the learner's code.
pub fn parse_diagnostics(stderr: &str) -> Vec<Diagnostic> {
    let lines: Vec<&str> = stderr.lines().collect();
    let mut diagnostics = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let Some(header) = parse_header(lines[i]) else {
            i += 1;
            continue;
        };
        let end = (i + 1..lines.len())
            .find(|&j| parse_header(lines[j]).is_some())
            .unwrap_or(lines.len());
        diagnostics.extend(build(header, &lines[i + 1..end]));
        i = end;
    }
    diagnostics
}

struct Header<'a> {
    severity: Severity,
    code: Option<&'a str>,
    message: &'a str,
}

/// Matches `error: msg`, `error[E0382]: msg` and `warning: msg` at column 0.
fn parse_header(line: &str) -> Option<Header<'_>> {
    let (severity, rest) = match line.strip_prefix("error") {
        Some(rest) => (Severity::Error, rest),
        None => (Severity::Warning, line.strip_prefix("warning")?),
    };
    let (code, rest) = match rest.strip_prefix('[') {
        Some(rest) => {
            let (code, rest) = rest.split_once(']')?;
            (Some(code), rest)
        }
        None => (None, rest),
    };
    let message = rest.strip_prefix(": ")?;
    Some(Header {
        severity,
        code,
        message,
    })
}

fn build(header: Header<'_>, body: &[&str]) -> Option<Diagnostic> {
    // The primary location always comes straight after the header.
    let location = body.first()?.trim_start().strip_prefix("--> ")?;
    let mut parts = location.rsplitn(3, ':');
    let col: u32 = parts.next()?.parse().ok()?;
    let line: u32 = parts.next()?.parse().ok()?;
    if parts.next()? != LEARNER_FILE {
        return None;
    }
    let width = primary_width(body, line).unwrap_or(1);
    let code = header.code.map(str::to_string).or_else(|| lint_name(body));
    Some(Diagnostic {
        severity: header.severity,
        code,
        message: header.message.to_string(),
        line,
        col,
        end_line: line,
        end_col: col + width,
    })
}

/// Counts the `^` markers under the primary line, which is how many characters
/// the primary span covers. Spans over several lines fall back to `None`.
fn primary_width(body: &[&str], line: u32) -> Option<u32> {
    let row = body.iter().position(|l| gutter_number(l) == Some(line))?;
    let markers = body[row + 1..]
        .iter()
        .take_while(|l| gutter_number(l).is_none())
        .find(|l| l.contains('^'))?;
    let start = markers.find('^')?;
    if markers[..start].contains('_') {
        return None;
    }
    let width = markers[start..].chars().take_while(|&c| c == '^').count();
    u32::try_from(width).ok()
}

/// The line number in a snippet row such as ` 7 |     takes(total)`.
fn gutter_number(row: &str) -> Option<u32> {
    let (gutter, _) = row.split_once('|')?;
    gutter.trim().parse().ok()
}

/// Lints have no error code, so use the lint name from the
/// ``= note: `#[warn(clippy::len_zero)]` on by default`` line instead.
fn lint_name(body: &[&str]) -> Option<String> {
    body.iter().find_map(|l| {
        let note = l.trim_start().strip_prefix("= note: `#[")?;
        let (_, rest) = note.split_once('(')?;
        let (name, _) = rest.split_once(')')?;
        Some(name.to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_headers() {
        let h = parse_header("error[E0382]: borrow of moved value: `s1`").unwrap();
        assert_eq!(h.severity, Severity::Error);
        assert_eq!(h.code, Some("E0382"));
        assert_eq!(h.message, "borrow of moved value: `s1`");

        let h = parse_header("warning: unused variable: `t`").unwrap();
        assert_eq!(h.severity, Severity::Warning);
        assert_eq!(h.code, None);

        assert!(parse_header("help: consider cloning the value").is_none());
        assert!(parse_header("  = note: `#[warn(unused_variables)]` on by default").is_none());
        assert!(parse_header("warnings are fun").is_none());
    }

    #[test]
    fn ignores_diagnostics_outside_the_learners_file() {
        let stderr =
            "error[E0463]: can't find crate for `foo`\n --> /rustc/library/core/src/lib.rs:1:1\n";
        assert!(parse_diagnostics(stderr).is_empty());
    }

    #[test]
    fn empty_output_has_no_diagnostics() {
        assert!(parse_diagnostics("").is_empty());
    }
}
