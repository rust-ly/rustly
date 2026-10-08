//! Lessons and exercises from `content/`, embedded at compile time and parsed
//! once on first use.

mod load;
mod model;
mod parse;

use std::sync::LazyLock;

pub use load::{Course, LoadError};
pub use model::{Chapter, Concept, Exercise, ExerciseKind, Snippet, SnippetKind, Track};

mod embedded {
    include!(concat!(env!("OUT_DIR"), "/files.rs"));
}

static COURSE: LazyLock<Course> = LazyLock::new(|| {
    Course::load(embedded::FILES.iter().copied()).unwrap_or_else(|errors| {
        let errors: Vec<_> = errors.iter().map(ToString::to_string).collect();
        panic!("content/ has errors:\n{}", errors.join("\n"))
    })
});

/// The embedded course. `cargo test -p content` checks that it loads.
pub fn course() -> &'static Course {
    &COURSE
}

/// One track's chapters, sorted by order.
pub fn chapters(track: Track) -> &'static [Chapter] {
    course().chapters(track)
}

pub fn exercise(id: &str) -> Option<&'static Exercise> {
    course().exercise(id)
}

pub fn snippets() -> impl Iterator<Item = &'static Snippet> {
    course().snippets()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_content_loads() {
        if let Err(errors) = Course::load(embedded::FILES.iter().copied()) {
            panic!("{errors:#?}");
        }
    }

    #[test]
    fn book_chapters_one_to_five_are_in_order() {
        let ids: Vec<_> = chapters(Track::Book)
            .iter()
            .map(|c| c.id.as_str())
            .collect();
        assert_eq!(
            ids[..5],
            [
                "book.getting-started",
                "book.guessing-game",
                "book.common-concepts",
                "book.ownership",
                "book.structs",
            ]
        );
    }

    #[test]
    fn every_concept_has_two_to_four_snippets() {
        for concept in course().concepts() {
            let n = concept.snippets.len();
            assert!((2..=4).contains(&n), "{} has {n} snippets", concept.id);
        }
    }

    #[test]
    fn looks_up_exercises_by_id() {
        let ex = exercise("book.ownership.challenge").unwrap();
        assert_eq!(ex.kind, ExerciseKind::Challenge);
        assert!(exercise("book.nope").is_none());
    }
}
