//! URLs for chapters and concepts, and looking them up again.
//!
//! Ids look like `book.ownership.moves`, so the URL is the id with dots as
//! slashes: `/learn/book/ownership/moves`.

use content::{Chapter, Concept, Track};

pub fn chapter_path(chapter: &Chapter) -> String {
    format!("/learn/{}", chapter.id.replace('.', "/"))
}

pub fn concept_path(concept: &Concept) -> String {
    format!("/learn/{}", concept.id.replace('.', "/"))
}

pub fn challenge_path(chapter: &Chapter) -> String {
    format!("{}/challenge", chapter_path(chapter))
}

pub fn find_chapter(track: &str, chapter: &str) -> Option<&'static Chapter> {
    let id = format!("{track}.{chapter}");
    content::course().all_chapters().find(|c| c.id == id)
}

pub fn find_concept(chapter: &'static Chapter, concept: &str) -> Option<&'static Concept> {
    let id = format!("{}.{concept}", chapter.id);
    chapter.concepts.iter().find(|c| c.id == id)
}

pub fn track_name(track: Track) -> &'static str {
    match track {
        Track::Book => "book",
        Track::Tokio => "tokio",
    }
}

/// The id without its track: `book.ownership.moves` → `ownership.moves`.
pub fn short_id(id: &str) -> &str {
    id.split_once('.').map_or(id, |(_, rest)| rest)
}

/// How a locked item tells the learner what to pass, e.g. `./ownership.moves`
/// for a concept or `./ownership.challenge` for a whole chapter.
pub fn requirement_label(id: &str) -> String {
    let is_chapter = content::course().all_chapters().any(|c| c.id == id);
    if is_chapter {
        format!("./{}.challenge", short_id(id))
    } else {
        format!("./{}", short_id(id))
    }
}

/// Where to send a learner who opens a locked page.
pub fn locked_redirect(concept_id: &str) -> String {
    format!("/?locked={concept_id}")
}
