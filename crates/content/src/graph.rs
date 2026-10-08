//! Which concepts and chapters depend on which, built once at load time.

use std::collections::{BTreeMap, BTreeSet};

use crate::model::{Chapter, Track};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseGraph {
    /// The first concept of the first Book chapter: open to everyone.
    root: Option<String>,
    concepts: BTreeMap<String, ConceptNode>,
    chapters: BTreeMap<String, ChapterNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConceptNode {
    chapter: String,
    /// Concept or chapter ids.
    requires: Vec<String>,
    checkpoint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChapterNode {
    concepts: Vec<String>,
    /// Passing any of these (the challenge or the test-out) completes the chapter.
    completed_by: Vec<String>,
}

impl CourseGraph {
    pub(crate) fn new<'a>(chapters: impl IntoIterator<Item = &'a Chapter>) -> Self {
        let mut graph = CourseGraph {
            root: None,
            concepts: BTreeMap::new(),
            chapters: BTreeMap::new(),
        };
        for chapter in chapters {
            if chapter.track == Track::Book && graph.root.is_none() {
                graph.root = chapter.concepts.first().map(|c| c.id.clone());
            }
            for concept in &chapter.concepts {
                graph.concepts.insert(
                    concept.id.clone(),
                    ConceptNode {
                        chapter: chapter.id.clone(),
                        requires: concept.requires.clone(),
                        checkpoint: concept.checkpoint.id.clone(),
                    },
                );
            }
            graph.chapters.insert(
                chapter.id.clone(),
                ChapterNode {
                    concepts: chapter.concepts.iter().map(|c| c.id.clone()).collect(),
                    completed_by: std::iter::once(&chapter.challenge)
                        .chain(&chapter.test_out)
                        .map(|e| e.id.clone())
                        .collect(),
                },
            );
        }
        graph
    }

    /// The concept a new learner starts on.
    pub fn root(&self) -> Option<&str> {
        self.root.as_deref()
    }

    /// Checks that every `requires` id exists, there are no cycles, and every
    /// concept can be reached from the root. Returns `(id, message)` pairs.
    pub(crate) fn validate(&self) -> Vec<(String, String)> {
        let mut errors = Vec::new();
        let Some(root) = &self.root else {
            return vec![("book".into(), "the Book track has no concepts".into())];
        };
        for (id, node) in &self.concepts {
            for required in &node.requires {
                if required == id || required == &node.chapter {
                    errors.push((id.clone(), format!("requires its own {required}")));
                } else if !self.concepts.contains_key(required)
                    && !self.chapters.contains_key(required)
                {
                    errors.push((id.clone(), format!("requires unknown id {required}")));
                }
            }
            if node.requires.is_empty() && id != root {
                errors.push((id.clone(), format!("has no requirements; only {root} may")));
            }
        }
        if !errors.is_empty() {
            return errors;
        }

        // Unlock everything a learner could, in rounds, starting from the
        // root. Whatever stays locked is in, or behind, a cycle.
        let mut open: BTreeSet<&str> = BTreeSet::new();
        loop {
            let before = open.len();
            for (id, node) in &self.concepts {
                if node.requires.iter().all(|r| open.contains(r.as_str())) {
                    open.insert(id);
                }
            }
            for (id, node) in &self.chapters {
                if node.concepts.iter().all(|c| open.contains(c.as_str())) {
                    open.insert(id);
                }
            }
            if open.len() == before {
                break;
            }
        }
        for id in self.concepts.keys() {
            if !open.contains(id.as_str()) {
                errors.push((
                    id.clone(),
                    format!("can't be reached from {root}: its requires form a cycle"),
                ));
            }
        }
        errors
    }
}

#[cfg(test)]
mod tests {
    use crate::load::tests::{concept, fixture, load};

    fn messages(files: &[(String, String)]) -> Vec<String> {
        load(files)
            .unwrap_err()
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    fn set_requires(files: &mut [(String, String)], concept_dir: &str, requires: &str) {
        let (_, text) = files
            .iter_mut()
            .find(|(p, _)| p == &format!("{concept_dir}/lesson.md"))
            .unwrap();
        *text = text.replace("requires = []", &format!("requires = [{requires}]"));
    }

    #[test]
    fn root_is_the_first_book_concept() {
        let course = load(&fixture()).unwrap();
        assert_eq!(course.graph().root(), Some("book.one.a"));
    }

    #[test]
    fn rejects_unknown_requires() {
        let mut files = fixture();
        set_requires(&mut files, "book/01-one/02-b", "\"book.nope\"");
        assert_eq!(
            messages(&files),
            ["book.one.b: requires unknown id book.nope"]
        );
    }

    #[test]
    fn rejects_requiring_itself() {
        let mut files = fixture();
        set_requires(&mut files, "book/01-one/02-b", "\"book.one.b\"");
        set_requires(&mut files, "book/02-two/01-c", "\"book.two\"");
        assert_eq!(
            messages(&files),
            [
                "book.one.b: requires its own book.one.b",
                "book.two.c: requires its own book.two",
            ]
        );
    }

    #[test]
    fn rejects_cycles() {
        let mut files = fixture();
        concept(
            &mut files,
            "book/02-two/02-d",
            "book.two.d",
            "book.two",
            2,
            "",
        );
        set_requires(&mut files, "book/02-two/01-c", "\"book.two.d\"");
        let errors = messages(&files);
        assert_eq!(errors.len(), 2, "{errors:?}");
        assert!(errors[0].starts_with("book.two.c: can't be reached from book.one.a"));
        assert!(errors[1].starts_with("book.two.d: can't be reached"));
    }

    #[test]
    fn rejects_a_second_starting_point() {
        let mut files = fixture();
        crate::load::tests::chapter(&mut files, "tokio", "01-hello", "tokio.hello", 1, "");
        concept(
            &mut files,
            "tokio/01-hello/01-main",
            "tokio.hello.main",
            "tokio.hello",
            1,
            "",
        );
        assert_eq!(
            messages(&files),
            ["tokio.hello.main: has no requirements; only book.one.a may"]
        );
    }
}
