//! Lessons and exercises embedded at compile time.

mod load;
mod model;
mod parse;

pub use load::{Course, LoadError};

pub use model::{Chapter, Concept, Exercise, ExerciseKind, Snippet, SnippetKind, Track};
