//! The course as the app sees it, after loading and resolving the files in `content/`.

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Track {
    Book,
    Tokio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExerciseKind {
    Practice,
    Checkpoint,
    Challenge,
    TestOut,
}

/// How a code fence in a lesson behaves, from its info string (`rust,runnable`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnippetKind {
    Static,
    Runnable,
    Editable,
    DoesNotCompile,
    Panics,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chapter {
    pub id: String,
    pub track: Track,
    pub title: String,
    pub order: u32,
    pub source_url: String,
    pub summary: String,
    /// As written in `chapter.toml`; usually empty.
    pub requires: Vec<String>,
    pub concepts: Vec<Concept>,
    pub challenge: Exercise,
    pub test_out: Option<Exercise>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Concept {
    pub id: String,
    pub chapter_id: String,
    pub title: String,
    pub order: u32,
    pub source_url: String,
    pub summary: String,
    /// Lesson markdown after the front matter, code fences included.
    pub body_md: String,
    /// Resolved: only the course's first concept has no requirements.
    pub requires: Vec<String>,
    /// The `rust` code fences in `body_md`, in order.
    pub snippets: Vec<Snippet>,
    pub checkpoint: Exercise,
    pub practice: Vec<Exercise>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snippet {
    pub id: String,
    pub concept_id: String,
    pub kind: SnippetKind,
    pub title: Option<String>,
    /// Full code sent to the compiler.
    pub code: String,
    /// What the page shows: `# ` setup lines removed.
    pub visible_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exercise {
    pub id: String,
    pub kind: ExerciseKind,
    pub title: String,
    pub prompt: String,
    pub starter: String,
    pub hints: Vec<String>,
    /// Appended to the learner's code when grading.
    #[cfg(feature = "server")]
    pub hidden_tests: String,
    #[cfg(feature = "server")]
    pub solution: String,
}
