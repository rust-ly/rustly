//! Builds a [`Course`] from the files in `content/`, keyed by their path
//! relative to that folder:
//!
//! ```text
//! <track>/<NN-chapter>/chapter.toml
//! <track>/<NN-chapter>/challenge.toml
//! <track>/<NN-chapter>/test-out.toml           (optional)
//! <track>/<NN-chapter>/<NN-concept>/lesson.md
//! <track>/<NN-chapter>/<NN-concept>/checkpoint.toml
//! <track>/<NN-chapter>/<NN-concept>/practice-*.toml (optional)
//! ```

use std::collections::{BTreeMap, HashMap};
use std::fmt;

use crate::graph::CourseGraph;
use crate::model::{Chapter, Concept, Exercise, ExerciseKind, Snippet, Track};
use crate::parse;

/// A problem with one file, or with how files fit together.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadError {
    pub path: String,
    pub message: String,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}

impl std::error::Error for LoadError {}

/// Every chapter of every track, sorted by order.
#[derive(Debug, Clone)]
pub struct Course {
    tracks: BTreeMap<Track, Vec<Chapter>>,
    graph: CourseGraph,
}

impl Course {
    /// Parses and checks every file, and reports all problems at once.
    pub fn load<'a>(
        files: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Result<Course, Vec<LoadError>> {
        let mut errors = Errors::default();
        let mut dirs: BTreeMap<&str, ChapterDir> = BTreeMap::new();
        for (path, text) in files {
            match path.split('/').collect::<Vec<_>>()[..] {
                [_, _, name] => {
                    let dir = dirs
                        .entry(&path[..path.len() - name.len() - 1])
                        .or_default();
                    match name {
                        "chapter.toml" => dir.chapter = Some((path, text)),
                        "challenge.toml" => dir.challenge = Some((path, text)),
                        "test-out.toml" => dir.test_out = Some((path, text)),
                        _ => errors.push(path, "unexpected file in a chapter folder"),
                    }
                }
                [_, _, concept, name] => {
                    let chapter_dir = &path[..path.len() - concept.len() - name.len() - 2];
                    let dir = dirs.entry(chapter_dir).or_default();
                    let concept = dir.concepts.entry(concept).or_default();
                    match name {
                        "lesson.md" => concept.lesson = Some((path, text)),
                        "checkpoint.toml" => concept.checkpoint = Some((path, text)),
                        _ if name.starts_with("practice-") && name.ends_with(".toml") => {
                            concept.practice.push((path, text))
                        }
                        _ => errors.push(path, "unexpected file in a concept folder"),
                    }
                }
                _ => errors.push(path, "unexpected file outside a chapter folder"),
            }
        }

        let mut tracks: BTreeMap<Track, Vec<Chapter>> = BTreeMap::new();
        for (path, dir) in dirs {
            if let Some(chapter) = dir.load(path, &mut errors) {
                tracks.entry(chapter.track).or_default().push(chapter);
            }
        }
        for chapters in tracks.values_mut() {
            chapters.sort_by_key(|c| c.order);
            for pair in chapters.windows(2) {
                if pair[0].order == pair[1].order {
                    errors.push(&pair[1].id, format!("same order as {}", pair[0].id));
                }
            }
            resolve_requires(chapters);
        }

        let graph = CourseGraph::new(tracks.values().flatten());
        let course = Course { tracks, graph };
        course.check_unique_ids(&mut errors);
        if errors.0.is_empty() {
            for (id, message) in course.graph.validate() {
                errors.push(&id, message);
            }
        }
        errors.finish(course)
    }

    /// One track's chapters, sorted by order.
    pub fn chapters(&self, track: Track) -> &[Chapter] {
        self.tracks.get(&track).map_or(&[], Vec::as_slice)
    }

    pub fn graph(&self) -> &CourseGraph {
        &self.graph
    }

    /// Every chapter, track by track.
    pub fn all_chapters(&self) -> impl Iterator<Item = &Chapter> {
        self.tracks.values().flatten()
    }

    pub fn concepts(&self) -> impl Iterator<Item = &Concept> {
        self.all_chapters().flat_map(|c| &c.concepts)
    }

    pub fn concept(&self, id: &str) -> Option<&Concept> {
        self.concepts().find(|c| c.id == id)
    }

    /// Every checkpoint, practice, challenge and test-out.
    pub fn exercises(&self) -> impl Iterator<Item = &Exercise> {
        self.all_chapters().flat_map(|chapter| {
            chapter
                .concepts
                .iter()
                .flat_map(|c| std::iter::once(&c.checkpoint).chain(&c.practice))
                .chain([&chapter.challenge])
                .chain(&chapter.test_out)
        })
    }

    pub fn exercise(&self, id: &str) -> Option<&Exercise> {
        self.exercises().find(|e| e.id == id)
    }

    pub fn snippets(&self) -> impl Iterator<Item = &Snippet> {
        self.concepts().flat_map(|c| &c.snippets)
    }

    fn check_unique_ids(&self, errors: &mut Errors) {
        let ids = self
            .all_chapters()
            .map(|c| &c.id)
            .chain(self.concepts().map(|c| &c.id))
            .chain(self.exercises().map(|e| &e.id))
            .chain(self.snippets().map(|s| &s.id));
        let mut seen = HashMap::new();
        for id in ids {
            *seen.entry(id).or_insert(0) += 1;
        }
        for (id, count) in seen {
            if count > 1 {
                errors.push(id, format!("id is used {count} times"));
            }
        }
    }
}

/// An empty `requires` means the previous concept in the chapter. For a
/// chapter's first concept it means the previous chapter in the track plus
/// whatever `chapter.toml` lists.
fn resolve_requires(chapters: &mut [Chapter]) {
    let mut previous_chapter: Option<String> = None;
    for chapter in chapters {
        let mut previous_concept: Option<String> = None;
        for concept in &mut chapter.concepts {
            if concept.requires.is_empty() {
                concept.requires = match previous_concept {
                    Some(id) => vec![id],
                    None => previous_chapter
                        .iter()
                        .chain(&chapter.requires)
                        .cloned()
                        .collect(),
                };
            }
            previous_concept = Some(concept.id.clone());
        }
        previous_chapter = Some(chapter.id.clone());
    }
}

type File<'a> = (&'a str, &'a str);

#[derive(Default)]
struct ChapterDir<'a> {
    chapter: Option<File<'a>>,
    challenge: Option<File<'a>>,
    test_out: Option<File<'a>>,
    concepts: BTreeMap<&'a str, ConceptDir<'a>>,
}

#[derive(Default)]
struct ConceptDir<'a> {
    lesson: Option<File<'a>>,
    checkpoint: Option<File<'a>>,
    practice: Vec<File<'a>>,
}

impl ChapterDir<'_> {
    fn load(self, path: &str, errors: &mut Errors) -> Option<Chapter> {
        let Some((chapter_path, text)) = self.chapter else {
            errors.push(path, "missing chapter.toml");
            return None;
        };
        let meta = errors.check(chapter_path, parse::chapter(text))?;
        let track = path.split('/').next().unwrap_or_default();
        if !track.eq_ignore_ascii_case(&format!("{:?}", meta.track)) {
            errors.push(
                chapter_path,
                format!("track is {:?} but the folder is {track}/", meta.track),
            );
        }

        let challenge = match self.challenge {
            Some(file) => load_exercise(file, ExerciseKind::Challenge, errors),
            None => {
                errors.push(path, "missing challenge.toml");
                None
            }
        };
        let test_out = self
            .test_out
            .and_then(|file| load_exercise(file, ExerciseKind::TestOut, errors));

        let mut concepts: Vec<Concept> = self
            .concepts
            .into_iter()
            .filter_map(|(name, dir)| dir.load(&format!("{path}/{name}"), &meta, errors))
            .collect();
        if concepts.is_empty() {
            errors.push(path, "chapter has no concepts");
        }
        concepts.sort_by_key(|c| c.order);
        for pair in concepts.windows(2) {
            if pair[0].order == pair[1].order {
                errors.push(&pair[1].id, format!("same order as {}", pair[0].id));
            }
        }

        Some(Chapter {
            id: meta.id,
            track: meta.track,
            title: meta.title,
            order: meta.order,
            source_url: meta.source,
            summary: meta.summary,
            requires: meta.requires,
            concepts,
            challenge: challenge?,
            test_out,
        })
    }
}

impl ConceptDir<'_> {
    fn load(
        self,
        path: &str,
        chapter: &parse::ChapterFile,
        errors: &mut Errors,
    ) -> Option<Concept> {
        let Some((lesson_path, text)) = self.lesson else {
            errors.push(path, "missing lesson.md");
            return None;
        };
        let (meta, body) = errors.check(lesson_path, parse::lesson(text))?;
        if meta.chapter != chapter.id {
            errors.push(
                lesson_path,
                format!(
                    "chapter is {} but the folder is {}",
                    meta.chapter, chapter.id
                ),
            );
        }
        if meta.track != chapter.track {
            errors.push(lesson_path, "track doesn't match chapter.toml");
        }
        let snippets = errors.check(lesson_path, parse::snippets(&meta.id, body))?;

        let checkpoint = match self.checkpoint {
            Some(file) => load_exercise(file, ExerciseKind::Checkpoint, errors),
            None => {
                errors.push(
                    path,
                    "missing checkpoint.toml: every concept needs exactly one",
                );
                None
            }
        };
        let practice = self
            .practice
            .into_iter()
            .filter_map(|file| load_exercise(file, ExerciseKind::Practice, errors))
            .collect();

        Some(Concept {
            id: meta.id,
            chapter_id: meta.chapter,
            title: meta.title,
            order: meta.order,
            source_url: meta.source,
            summary: meta.summary,
            body_md: body.to_string(),
            requires: meta.requires,
            snippets,
            checkpoint: checkpoint?,
            practice,
        })
    }
}

fn load_exercise((path, text): File, kind: ExerciseKind, errors: &mut Errors) -> Option<Exercise> {
    let exercise = errors.check(path, parse::exercise(text))?;
    if exercise.kind != kind {
        errors.push(
            path,
            format!(
                "kind is {:?}, but this file must be {kind:?}",
                exercise.kind
            ),
        );
    }
    Some(exercise)
}

#[derive(Default)]
struct Errors(Vec<LoadError>);

impl Errors {
    fn push(&mut self, path: &str, message: impl Into<String>) {
        self.0.push(LoadError {
            path: path.to_string(),
            message: message.into(),
        });
    }

    fn check<T>(&mut self, path: &str, result: Result<T, String>) -> Option<T> {
        result.map_err(|message| self.push(path, message)).ok()
    }

    fn finish(mut self, course: Course) -> Result<Course, Vec<LoadError>> {
        if self.0.is_empty() {
            Ok(course)
        } else {
            self.0.sort_by(|a, b| a.path.cmp(&b.path));
            Err(self.0)
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A small but complete course: `book.one` (concepts a, b) and `book.two` (c).
    pub(crate) fn fixture() -> Vec<(String, String)> {
        let mut files = Vec::new();
        chapter(&mut files, "book", "01-one", "book.one", 1, "");
        concept(
            &mut files,
            "book/01-one/01-a",
            "book.one.a",
            "book.one",
            1,
            "",
        );
        concept(
            &mut files,
            "book/01-one/02-b",
            "book.one.b",
            "book.one",
            2,
            "",
        );
        chapter(&mut files, "book", "02-two", "book.two", 2, "");
        concept(
            &mut files,
            "book/02-two/01-c",
            "book.two.c",
            "book.two",
            1,
            "",
        );
        files
    }

    pub(crate) fn chapter(
        files: &mut Vec<(String, String)>,
        track: &str,
        dir: &str,
        id: &str,
        order: u32,
        requires: &str,
    ) {
        files.push((
            format!("{track}/{dir}/chapter.toml"),
            format!(
                "id = \"{id}\"\ntrack = \"{track}\"\ntitle = \"T\"\norder = {order}\n\
                 source = \"https://example.com\"\nsummary = \"S\"\nrequires = [{requires}]\n"
            ),
        ));
        files.push((
            format!("{track}/{dir}/challenge.toml"),
            exercise(&format!("{id}.challenge"), "challenge"),
        ));
    }

    pub(crate) fn concept(
        files: &mut Vec<(String, String)>,
        dir: &str,
        id: &str,
        chapter: &str,
        order: u32,
        requires: &str,
    ) {
        let track = &dir[..dir.find('/').unwrap()];
        files.push((
            format!("{dir}/lesson.md"),
            format!(
                "+++\nid = \"{id}\"\nchapter = \"{chapter}\"\nrequires = [{requires}]\n\
                 title = \"T\"\ntrack = \"{track}\"\norder = {order}\n\
                 source = \"https://example.com\"\nsummary = \"S\"\n+++\n\n\
                 ```rust,runnable\nfn main() {{}}\n```\n"
            ),
        ));
        files.push((
            format!("{dir}/checkpoint.toml"),
            exercise(&format!("{id}.checkpoint"), "checkpoint"),
        ));
    }

    pub(crate) fn exercise(id: &str, kind: &str) -> String {
        format!(
            "id = \"{id}\"\nkind = \"{kind}\"\ntitle = \"T\"\nprompt = \"P\"\n\
             starter = \"fn main() {{}}\"\nhidden_tests = \"\"\nsolution = \"fn main() {{}}\"\n"
        )
    }

    pub(crate) fn load(files: &[(String, String)]) -> Result<Course, Vec<LoadError>> {
        Course::load(files.iter().map(|(p, t)| (p.as_str(), t.as_str())))
    }

    fn messages(files: &[(String, String)]) -> Vec<String> {
        load(files)
            .unwrap_err()
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    #[test]
    fn loads_a_course() {
        let course = load(&fixture()).unwrap();
        let chapters = course.chapters(Track::Book);
        assert_eq!(chapters.len(), 2);
        assert_eq!(chapters[0].concepts.len(), 2);
        assert!(course.chapters(Track::Tokio).is_empty());
        assert_eq!(course.exercises().count(), 5);
        assert_eq!(course.snippets().count(), 3);
        assert_eq!(
            course.exercise("book.two.challenge").unwrap().kind,
            ExerciseKind::Challenge
        );
    }

    #[test]
    fn resolves_empty_requires() {
        let course = load(&fixture()).unwrap();
        let requires = |id| course.concept(id).unwrap().requires.clone();
        assert!(
            requires("book.one.a").is_empty(),
            "the course's first concept"
        );
        assert_eq!(requires("book.one.b"), ["book.one.a"]);
        assert_eq!(requires("book.two.c"), ["book.one"]);
    }

    #[test]
    fn keeps_explicit_requires() {
        let mut files = fixture();
        files.retain(|(p, _)| !p.starts_with("book/02-two/01-c/"));
        concept(
            &mut files,
            "book/02-two/01-c",
            "book.two.c",
            "book.two",
            1,
            "\"book.one.a\"",
        );
        let course = load(&files).unwrap();
        assert_eq!(
            course.concept("book.two.c").unwrap().requires,
            ["book.one.a"]
        );
    }

    #[test]
    fn sorts_by_order_not_folder_name() {
        let mut files = fixture();
        for (path, text) in &mut files {
            if path.starts_with("book/01-one/01-a/lesson.md") {
                *text = text.replace("order = 1", "order = 3");
            }
        }
        let course = load(&files).unwrap();
        let ids: Vec<_> = course.chapters(Track::Book)[0]
            .concepts
            .iter()
            .map(|c| &c.id)
            .collect();
        assert_eq!(ids, ["book.one.b", "book.one.a"]);
    }

    #[test]
    fn rejects_duplicate_ids() {
        let mut files = fixture();
        concept(
            &mut files,
            "book/02-two/02-d",
            "book.one.a",
            "book.two",
            2,
            "",
        );
        let errors = messages(&files);
        assert!(
            errors.contains(&"book.one.a: id is used 2 times".to_string()),
            "{errors:?}"
        );
    }

    #[test]
    fn rejects_bad_toml_and_missing_fields() {
        let mut files = fixture();
        let mut set = |suffix: &str, text: &str| {
            files
                .iter_mut()
                .find(|(p, _)| p.ends_with(suffix))
                .unwrap()
                .1 = text.into();
        };
        set("02-two/chapter.toml", "id = ");
        set(
            "02-b/checkpoint.toml",
            "id = \"x\"\nkind = \"checkpoint\"\n",
        );
        let errors = messages(&files);
        assert_eq!(errors.len(), 2, "{errors:?}");
        assert!(
            errors[0].starts_with("book/01-one/02-b/checkpoint.toml: TOML parse error"),
            "{errors:?}"
        );
        assert!(errors[0].contains("missing field `title`"));
        assert!(errors[1].starts_with("book/02-two/chapter.toml: TOML parse error"));
    }

    #[test]
    fn rejects_missing_and_misplaced_files() {
        let mut files = fixture();
        files.retain(|(p, _)| p != "book/01-one/02-b/checkpoint.toml");
        files.retain(|(p, _)| p != "book/02-two/challenge.toml");
        files.push(("book/02-two/01-c/notes.md".into(), String::new()));
        files.push(("README.md".into(), String::new()));
        let errors = messages(&files);
        assert_eq!(
            errors,
            [
                "README.md: unexpected file outside a chapter folder",
                "book/01-one/02-b: missing checkpoint.toml: every concept needs exactly one",
                "book/02-two: missing challenge.toml",
                "book/02-two/01-c/notes.md: unexpected file in a concept folder",
            ]
        );
    }

    #[test]
    fn rejects_exercises_of_the_wrong_kind() {
        let mut files = fixture();
        files.push((
            "book/01-one/01-a/practice-extra.toml".into(),
            exercise("book.one.a.extra", "checkpoint"),
        ));
        assert_eq!(
            messages(&files),
            [
                "book/01-one/01-a/practice-extra.toml: kind is Checkpoint, but this file must be Practice"
            ]
        );
    }

    #[test]
    fn rejects_lessons_filed_under_the_wrong_chapter() {
        let mut files = fixture();
        concept(
            &mut files,
            "book/02-two/02-d",
            "book.two.d",
            "book.one",
            2,
            "",
        );
        assert_eq!(
            messages(&files),
            ["book/02-two/02-d/lesson.md: chapter is book.one but the folder is book.two"]
        );
    }
}
