//! Parsing for single files: chapter and exercise TOML, and lessons (front
//! matter plus snippets from the code fences).

use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use serde::Deserialize;

use crate::model::{Exercise, ExerciseKind, Snippet, SnippetKind, Track};

/// `chapter.toml`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ChapterFile {
    pub id: String,
    pub track: Track,
    pub title: String,
    pub order: u32,
    pub source: String,
    pub summary: String,
    #[serde(default)]
    pub requires: Vec<String>,
}

/// The TOML front matter between the `+++` lines of `lesson.md`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LessonMeta {
    pub id: String,
    pub chapter: String,
    #[serde(default)]
    pub requires: Vec<String>,
    pub title: String,
    pub track: Track,
    pub order: u32,
    pub source: String,
    pub summary: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExerciseFile {
    id: String,
    kind: ExerciseKind,
    title: String,
    prompt: String,
    starter: String,
    #[serde(default)]
    hints: Vec<String>,
    #[cfg(feature = "server")]
    hidden_tests: String,
    #[cfg(feature = "server")]
    solution: String,
    // Stripped from the embedded files without `server`, but accepted so
    // the same TOML parses either way.
    #[cfg(not(feature = "server"))]
    #[serde(default, rename = "hidden_tests")]
    _hidden_tests: serde::de::IgnoredAny,
    #[cfg(not(feature = "server"))]
    #[serde(default, rename = "solution")]
    _solution: serde::de::IgnoredAny,
}

pub(crate) fn chapter(text: &str) -> Result<ChapterFile, String> {
    toml::from_str(text).map_err(|e| e.to_string())
}

pub(crate) fn exercise(text: &str) -> Result<Exercise, String> {
    let file: ExerciseFile = toml::from_str(text).map_err(|e| e.to_string())?;
    Ok(Exercise {
        id: file.id,
        kind: file.kind,
        title: file.title,
        prompt: file.prompt,
        starter: file.starter,
        hints: file.hints,
        #[cfg(feature = "server")]
        hidden_tests: file.hidden_tests,
        #[cfg(feature = "server")]
        solution: file.solution,
    })
}

/// Splits a lesson into its front matter and markdown body.
pub(crate) fn lesson(text: &str) -> Result<(LessonMeta, &str), String> {
    let rest = text
        .strip_prefix("+++\n")
        .ok_or("lesson must start with a `+++` front matter line")?;
    let (meta, body) = rest
        .split_once("\n+++\n")
        .ok_or("front matter has no closing `+++` line")?;
    let meta = toml::from_str(meta).map_err(|e| format!("front matter: {e}"))?;
    Ok((meta, body))
}

/// Every `rust` code fence in `body`, in order. Fences in other languages
/// (`sh`, `toml`, ...) are display-only and aren't snippets.
pub(crate) fn snippets(concept_id: &str, body: &str) -> Result<Vec<Snippet>, String> {
    let mut snippets = Vec::new();
    let mut current: Option<(SnippetKind, Option<String>, String)> = None;
    for event in Parser::new(body) {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                if let Some((kind, title)) = fence_info(&info)? {
                    current = Some((kind, title, String::new()));
                }
            }
            Event::Text(text) => {
                if let Some((_, _, code)) = &mut current {
                    code.push_str(&text);
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((kind, title, code)) = current.take() {
                    let (code, visible_code) = split_hidden_lines(&code);
                    snippets.push(Snippet {
                        id: format!("{concept_id}.snippet-{}", snippets.len() + 1),
                        concept_id: concept_id.to_string(),
                        kind,
                        title,
                        code,
                        visible_code,
                    });
                }
            }
            _ => {}
        }
    }
    Ok(snippets)
}

/// Reads an info string such as `rust,does_not_compile title="Moving a String"`.
/// Returns `None` for fences that aren't Rust.
fn fence_info(info: &str) -> Result<Option<(SnippetKind, Option<String>)>, String> {
    let info = info.trim();
    let (head, attrs) = match info.split_once(char::is_whitespace) {
        Some((head, attrs)) => (head, attrs.trim()),
        None => (info, ""),
    };
    let mut parts = head.split(',');
    if parts.next() != Some("rust") {
        return Ok(None);
    }
    let kind = match parts.next() {
        None => SnippetKind::Static,
        Some("runnable") => SnippetKind::Runnable,
        Some("editable") => SnippetKind::Editable,
        Some("does_not_compile") => SnippetKind::DoesNotCompile,
        Some("panics") => SnippetKind::Panics,
        Some(other) => return Err(format!("unknown snippet kind `{other}` in ```{info}")),
    };
    if parts.next().is_some() {
        return Err(format!("more than one snippet kind in ```{info}"));
    }
    let title = if attrs.is_empty() {
        None
    } else {
        let title = attrs
            .strip_prefix("title=\"")
            .and_then(|t| t.strip_suffix('"'))
            .ok_or_else(|| format!("expected `title=\"...\"` in ```{info}"))?;
        Some(title.to_string())
    };
    Ok(Some((kind, title)))
}

/// Lines starting with `# ` (or a bare `#`) are compiled but not shown, like
/// rustdoc. Returns `(code, visible_code)`.
fn split_hidden_lines(code: &str) -> (String, String) {
    let mut full = String::new();
    let mut visible = String::new();
    for line in code.lines() {
        if line == "#" {
            full.push('\n');
        } else if let Some(hidden) = line.strip_prefix("# ") {
            full.push_str(hidden);
            full.push('\n');
        } else {
            full.push_str(line);
            full.push('\n');
            visible.push_str(line);
            visible.push('\n');
        }
    }
    (full, visible)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LESSON: &str = r#"+++
id = "book.ownership.moves"
chapter = "book.ownership"
title = "Moves"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html"
summary = "Who owns a value."
+++

# Moves

```rust
let s = String::from("hi");
```

```sh
cargo run
```

```rust,does_not_compile title="Moving, then using"
# #[derive(Debug)]
# struct Unit;
#
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}");
}
```
"#;

    #[test]
    fn reads_front_matter_and_body() {
        let (meta, body) = lesson(LESSON).unwrap();
        assert_eq!(meta.id, "book.ownership.moves");
        assert_eq!(meta.track, Track::Book);
        assert!(meta.requires.is_empty());
        assert!(body.starts_with("\n# Moves\n"));
    }

    #[test]
    fn rejects_lessons_without_front_matter() {
        assert!(lesson("# Moves\n").is_err());
        assert!(lesson("+++\nid = \"x\"\n# Moves\n").is_err());
    }

    #[test]
    fn rejects_missing_and_unknown_front_matter_keys() {
        let missing = LESSON.replace("title = \"Moves\"\n", "");
        assert!(lesson(&missing).unwrap_err().contains("title"));
        let unknown = LESSON.replace("order = 1", "order = 1\nlevel = 2");
        assert!(lesson(&unknown).unwrap_err().contains("level"));
    }

    #[test]
    fn finds_rust_fences_only() {
        let (meta, body) = lesson(LESSON).unwrap();
        let snippets = snippets(&meta.id, body).unwrap();
        assert_eq!(snippets.len(), 2);
        assert_eq!(snippets[0].kind, SnippetKind::Static);
        assert_eq!(snippets[0].title, None);
        assert_eq!(snippets[1].id, "book.ownership.moves.snippet-2");
        assert_eq!(snippets[1].kind, SnippetKind::DoesNotCompile);
        assert_eq!(snippets[1].title.as_deref(), Some("Moving, then using"));
    }

    #[test]
    fn hides_setup_lines_but_compiles_them() {
        let (meta, body) = lesson(LESSON).unwrap();
        let snippet = &snippets(&meta.id, body).unwrap()[1];
        assert!(
            snippet
                .code
                .starts_with("#[derive(Debug)]\nstruct Unit;\n\nfn main() {\n")
        );
        assert!(snippet.visible_code.starts_with("fn main() {\n"));
        assert!(!snippet.visible_code.contains("Unit"));
    }

    #[test]
    fn reads_every_snippet_kind() {
        let kinds = [
            ("rust", SnippetKind::Static),
            ("rust,runnable", SnippetKind::Runnable),
            ("rust,editable", SnippetKind::Editable),
            ("rust,does_not_compile", SnippetKind::DoesNotCompile),
            ("rust,panics", SnippetKind::Panics),
        ];
        for (info, kind) in kinds {
            assert_eq!(fence_info(info).unwrap(), Some((kind, None)), "{info}");
        }
        assert_eq!(fence_info("toml").unwrap(), None);
    }

    #[test]
    fn titles_may_contain_commas() {
        assert_eq!(
            fence_info(r#"rust,runnable title="Hello, world""#).unwrap(),
            Some((SnippetKind::Runnable, Some("Hello, world".into())))
        );
    }

    #[test]
    fn rejects_unknown_fence_attributes() {
        assert!(fence_info("rust,ignore").is_err());
        assert!(fence_info("rust,runnable,panics").is_err());
        assert!(fence_info("rust,runnable name=\"x\"").is_err());
    }

    const EXERCISE: &str = r##"
id = "book.ownership.moves.checkpoint"
kind = "checkpoint"
title = "Fix the move"
prompt = "Make `greet` borrow."
starter = "fn main() {}"
hidden_tests = "#[test] fn t() {}"
hints = ["Look at the parameter type."]
solution = "fn main() {}"
"##;

    #[test]
    fn reads_an_exercise() {
        let ex = exercise(EXERCISE).unwrap();
        assert_eq!(ex.kind, ExerciseKind::Checkpoint);
        assert_eq!(ex.hints.len(), 1);
    }

    #[cfg(feature = "server")]
    #[test]
    fn server_requires_tests_and_solution() {
        let ex = exercise(EXERCISE).unwrap();
        assert_eq!(ex.hidden_tests, "#[test] fn t() {}");
        let no_tests = EXERCISE.replace("hidden_tests = \"#[test] fn t() {}\"\n", "");
        assert!(exercise(&no_tests).unwrap_err().contains("hidden_tests"));
    }

    #[test]
    fn rejects_bad_exercises() {
        assert!(exercise("id = ").is_err(), "bad TOML");
        let no_title = EXERCISE.replace("title = \"Fix the move\"\n", "");
        assert!(exercise(&no_title).unwrap_err().contains("title"));
        let bad_kind = EXERCISE.replace("\"checkpoint\"", "\"quiz\"");
        assert!(exercise(&bad_kind).is_err());
        assert_eq!(
            exercise(&EXERCISE.replace("\"checkpoint\"", "\"test-out\""))
                .unwrap()
                .kind,
            ExerciseKind::TestOut
        );
    }
}
