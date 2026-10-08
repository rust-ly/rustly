//! A learner's progress, saved in their browser, and what it unlocks.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::graph::CourseGraph;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Progress {
    /// Bump and migrate if the shape changes.
    pub version: u32,
    /// Passed exercise ids: checkpoints, challenges, practice and test-outs.
    pub completed: BTreeSet<String>,
    pub concepts_read: BTreeSet<String>,
    /// Failed submits per exercise; the solution unlocks after 3.
    pub failed_submits: BTreeMap<String, u32>,
    pub starred_snippets: BTreeSet<String>,
    /// Route to resume from.
    pub last_visited: Option<String>,
}

/// The concepts whose lessons and checkpoints are open: every requirement is
/// met, or their chapter is already complete. Never stored, so changing the
/// course can't leave stale unlocks behind.
pub fn unlocked(graph: &CourseGraph, progress: &Progress) -> BTreeSet<String> {
    graph
        .concept_ids()
        .filter(|id| graph.is_open(id, progress))
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load::tests::{chapter, concept, fixture, load};

    fn progress(completed: &[&str]) -> Progress {
        Progress {
            completed: completed.iter().map(|s| s.to_string()).collect(),
            ..Progress::default()
        }
    }

    fn unlocked_in_book(completed: &[&str]) -> Vec<String> {
        unlocked(crate::graph(), &progress(completed))
            .into_iter()
            .collect()
    }

    #[test]
    fn a_new_learner_can_only_open_hello_world() {
        assert_eq!(unlocked_in_book(&[]), ["book.getting-started.hello-world"]);
    }

    #[test]
    fn a_checkpoint_unlocks_the_next_concept() {
        assert_eq!(
            unlocked_in_book(&["book.getting-started.hello-world.checkpoint"]),
            [
                "book.getting-started.hello-cargo",
                "book.getting-started.hello-world",
            ]
        );
    }

    #[test]
    fn the_challenge_opens_once_every_concept_is_done() {
        let graph = crate::graph();
        let chapter = "book.getting-started";
        let one = progress(&["book.getting-started.hello-world.checkpoint"]);
        assert!(!graph.is_challenge_open(chapter, &one));
        let both = progress(&[
            "book.getting-started.hello-world.checkpoint",
            "book.getting-started.hello-cargo.checkpoint",
        ]);
        assert!(graph.is_challenge_open(chapter, &both));
    }

    #[test]
    fn a_challenge_unlocks_the_next_chapter() {
        let open = unlocked_in_book(&[
            "book.getting-started.hello-world.checkpoint",
            "book.getting-started.hello-cargo.checkpoint",
        ]);
        assert!(!open.contains(&"book.guessing-game.processing-a-guess".to_string()));

        let open = unlocked_in_book(&[
            "book.getting-started.hello-world.checkpoint",
            "book.getting-started.hello-cargo.checkpoint",
            "book.getting-started.challenge",
        ]);
        assert!(open.contains(&"book.guessing-game.processing-a-guess".to_string()));
        assert!(!open.contains(&"book.guessing-game.comparing-guesses".to_string()));
    }

    #[test]
    fn practice_unlocks_nothing() {
        let mut p = progress(&[]);
        p.completed
            .insert("book.getting-started.hello-world.practice-1".into());
        assert_eq!(unlocked(crate::graph(), &p).len(), 1);
    }

    #[test]
    fn mid_chapter() {
        // Chapters 1 and 2 done, first concept of chapter 3 passed.
        let course = crate::course();
        let mut done: Vec<&str> = Vec::new();
        for chapter in &course.chapters(crate::Track::Book)[..2] {
            done.extend(chapter.concepts.iter().map(|c| c.checkpoint.id.as_str()));
            done.push(&chapter.challenge.id);
        }
        done.push("book.common-concepts.variables.checkpoint");
        let open = unlocked_in_book(&done);
        assert!(open.contains(&"book.common-concepts.types-and-functions".to_string()));
        assert!(!open.contains(&"book.common-concepts.control-flow".to_string()));
        assert!(!open.iter().any(|id| id.starts_with("book.ownership")));
    }

    /// A course with a test-out on chapter one and a Tokio chapter that
    /// needs both Book chapters.
    fn cross_track() -> crate::Course {
        let mut files = fixture();
        files.push((
            "book/01-one/test-out.toml".into(),
            crate::load::tests::exercise("book.one.test-out", "test-out"),
        ));
        chapter(
            &mut files,
            "tokio",
            "01-hello",
            "tokio.hello",
            1,
            "\"book.one\", \"book.two\"",
        );
        concept(
            &mut files,
            "tokio/01-hello/01-main",
            "tokio.hello.main",
            "tokio.hello",
            1,
            "",
        );
        load(&files).unwrap()
    }

    #[test]
    fn a_test_out_completes_the_whole_chapter() {
        let course = cross_track();
        let open = unlocked(course.graph(), &progress(&["book.one.test-out"]));
        assert_eq!(
            open.into_iter().collect::<Vec<_>>(),
            ["book.one.a", "book.one.b", "book.two.c"]
        );
        assert!(
            course
                .graph()
                .is_complete("book.one", &progress(&["book.one.test-out"]))
        );
    }

    #[test]
    fn tokio_waits_for_every_required_book_chapter() {
        let course = cross_track();
        let graph = course.graph();
        assert_eq!(
            graph.concept_requires("tokio.hello.main"),
            ["book.one", "book.two"]
        );
        let one_done = progress(&["book.one.challenge"]);
        assert!(!unlocked(graph, &one_done).contains("tokio.hello.main"));
        let both_done = progress(&["book.one.challenge", "book.two.challenge"]);
        assert!(unlocked(graph, &both_done).contains("tokio.hello.main"));
    }

    #[test]
    fn progress_round_trips_and_tolerates_missing_fields() {
        let mut p = progress(&["book.one.a.checkpoint"]);
        p.failed_submits.insert("book.one.challenge".into(), 2);
        let json = serde_json::to_string(&p).unwrap();
        assert_eq!(serde_json::from_str::<Progress>(&json).unwrap(), p);
        assert_eq!(
            serde_json::from_str::<Progress>("{}").unwrap(),
            Progress::default()
        );
    }
}
