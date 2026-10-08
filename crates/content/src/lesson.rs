//! Splits a lesson into HTML and the snippets placed between it, so the
//! frontend can render each snippet as a live component.

use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd, html};

use crate::parse::fence_info;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LessonBlock {
    /// Rendered markdown between snippets.
    Html(String),
    /// Index into the concept's `snippets`.
    Snippet(usize),
}

/// The lesson as HTML with each `rust` code fence replaced by a
/// [`LessonBlock::Snippet`]. Snippets are numbered in the same order as
/// [`crate::Concept::snippets`]. Other fences (`sh`, `toml`, ...) stay as HTML.
pub fn lesson_blocks(body_md: &str) -> Vec<LessonBlock> {
    let mut blocks = Vec::new();
    let mut pending: Vec<Event> = Vec::new();
    let mut in_snippet = false;
    let mut snippets = 0;
    for event in Parser::new_ext(body_md, Options::ENABLE_TABLES) {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref info)))
                if matches!(fence_info(info), Ok(Some(_))) =>
            {
                flush(&mut pending, &mut blocks);
                blocks.push(LessonBlock::Snippet(snippets));
                snippets += 1;
                in_snippet = true;
            }
            Event::End(TagEnd::CodeBlock) if in_snippet => in_snippet = false,
            _ if in_snippet => {}
            event => pending.push(event),
        }
    }
    flush(&mut pending, &mut blocks);
    blocks
}

fn flush(pending: &mut Vec<Event>, blocks: &mut Vec<LessonBlock>) {
    if pending.is_empty() {
        return;
    }
    let mut out = String::new();
    html::push_html(&mut out, pending.drain(..));
    blocks.push(LessonBlock::Html(out));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn places_snippets_between_html() {
        let body = "# Moves\n\nIntro.\n\n```rust,runnable\nfn main() {}\n```\n\nMiddle.\n\n```rust\nlet x = 1;\n```\n";
        assert_eq!(
            lesson_blocks(body),
            [
                LessonBlock::Html("<h1>Moves</h1>\n<p>Intro.</p>\n".into()),
                LessonBlock::Snippet(0),
                LessonBlock::Html("<p>Middle.</p>\n".into()),
                LessonBlock::Snippet(1),
            ]
        );
    }

    #[test]
    fn keeps_other_languages_as_html() {
        let blocks = lesson_blocks("```sh\ncargo run\n```\n");
        assert_eq!(
            blocks,
            [LessonBlock::Html(
                "<pre><code class=\"language-sh\">cargo run\n</code></pre>\n".into()
            )]
        );
    }

    #[test]
    fn numbers_snippets_like_the_concept_does() {
        for concept in crate::course().concepts() {
            let count = lesson_blocks(&concept.body_md)
                .iter()
                .filter(|b| matches!(b, LessonBlock::Snippet(_)))
                .count();
            assert_eq!(count, concept.snippets.len(), "{}", concept.id);
        }
    }
}
