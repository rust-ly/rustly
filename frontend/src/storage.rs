//! Everything the app keeps in `localStorage`. Reads fall back to defaults if
//! the store is missing, blocked or holds something unreadable.

use content::Progress;
use gloo_storage::{LocalStorage, Storage};

const PROGRESS: &str = "progress";
/// Read by the inline script in `index.html` too, so it stays a plain string.
const THEME: &str = "rustly:theme";
/// Local dev only, see `Learner::provide`.
const PREVIEW: &str = "rustly:preview";

pub fn load_progress() -> Progress {
    LocalStorage::get(PROGRESS).unwrap_or_default()
}

pub fn save_progress(progress: &Progress) {
    // Best effort: a full or blocked store shouldn't break the page.
    let _ = LocalStorage::set(PROGRESS, progress);
}

pub fn load_preview() -> bool {
    LocalStorage::get(PREVIEW).unwrap_or(false)
}

pub fn save_preview(on: bool) {
    let _ = LocalStorage::set(PREVIEW, on);
}

pub fn save_theme(theme: &str) {
    let _ = LocalStorage::raw().set_item(THEME, theme);
}
