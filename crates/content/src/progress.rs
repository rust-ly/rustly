//! A learner's progress, saved in their browser, and what it unlocks.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::graph::CourseGraph;
use crate::load::Course;

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

impl Progress {
    /// Records one graded submit: a pass completes the exercise, a fail
    /// counts towards showing the solution.
    pub fn record_submit(&mut self, exercise: &str, passed: bool) {
        if passed {
            self.completed.insert(exercise.to_string());
        } else {
            *self.failed_submits.entry(exercise.to_string()).or_default() += 1;
        }
    }
}

/// What a change in progress opened, in course order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Unlocks {
    pub concepts: Vec<String>,
    /// Chapter ids whose challenge became available.
    pub challenges: Vec<String>,
}

impl Unlocks {
    pub fn is_empty(&self) -> bool {
        self.concepts.is_empty() && self.challenges.is_empty()
    }
}

/// The concepts and challenges open in `after` that weren't in `before`.
pub fn newly_unlocked(course: &Course, before: &Progress, after: &Progress) -> Unlocks {
    let graph = course.graph();
    let concepts = course
        .concepts()
        .filter(|c| graph.is_open(&c.id, after) && !graph.is_open(&c.id, before))
        .map(|c| c.id.clone())
        .collect();
    let challenges = course
        .all_chapters()
        .filter(|c| {
            graph.is_challenge_open(&c.id, after)
                && !graph.is_challenge_open(&c.id, before)
                && !graph.is_complete(&c.id, after)
        })
        .map(|c| c.id.clone())
        .collect();
    Unlocks {
        concepts,
        challenges,
    }
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
    fn a_pass_completes_and_a_fail_is_counted() {
        let mut p = Progress::default();
        p.record_submit("book.one.a.checkpoint", false);
        p.record_submit("book.one.a.checkpoint", false);
        assert_eq!(p.failed_submits["book.one.a.checkpoint"], 2);
        assert!(p.completed.is_empty());
        p.record_submit("book.one.a.checkpoint", true);
        assert!(p.completed.contains("book.one.a.checkpoint"));
        assert_eq!(p.failed_submits["book.one.a.checkpoint"], 2);
    }

    #[test]
    fn a_checkpoint_reports_the_concept_it_unlocked() {
        let before = progress(&[]);
        let after = progress(&["book.getting-started.hello-world.checkpoint"]);
        let unlocks = newly_unlocked(crate::course(), &before, &after);
        assert_eq!(unlocks.concepts, ["book.getting-started.hello-cargo"]);
        assert!(unlocks.challenges.is_empty());
    }

    #[test]
    fn the_last_checkpoint_reports_the_challenge() {
        let before = progress(&["book.getting-started.hello-world.checkpoint"]);
        let mut after = before.clone();
        after.record_submit("book.getting-started.hello-cargo.checkpoint", true);
        let unlocks = newly_unlocked(crate::course(), &before, &after);
        assert!(unlocks.concepts.is_empty());
        assert_eq!(unlocks.challenges, ["book.getting-started"]);
    }

    #[test]
    fn a_challenge_reports_the_next_chapter() {
        let before = progress(&[
            "book.getting-started.hello-world.checkpoint",
            "book.getting-started.hello-cargo.checkpoint",
        ]);
        let mut after = before.clone();
        after.record_submit("book.getting-started.challenge", true);
        let unlocks = newly_unlocked(crate::course(), &before, &after);
        assert_eq!(unlocks.concepts, ["book.guessing-game.processing-a-guess"]);
        assert!(unlocks.challenges.is_empty());
    }

    #[test]
    fn a_test_out_reports_the_rest_of_the_chapter_but_not_its_challenge() {
        let course = cross_track();
        let unlocks = newly_unlocked(&course, &progress(&[]), &progress(&["book.one.test-out"]));
        assert_eq!(unlocks.concepts, ["book.one.b", "book.two.c"]);
        assert!(unlocks.challenges.is_empty());
    }

    #[test]
    fn a_repeat_pass_unlocks_nothing_new() {
        let p = progress(&["book.getting-started.hello-world.checkpoint"]);
        assert!(newly_unlocked(crate::course(), &p, &p).is_empty());
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
