//! The learner's progress as app-wide reactive state.

use std::collections::BTreeSet;

use content::Progress;
use leptos::prelude::*;

use crate::storage;

#[derive(Clone, Copy)]
pub struct Learner {
    pub progress: RwSignal<Progress>,
    /// Open concept ids, recomputed from progress (never stored).
    pub unlocked: Memo<BTreeSet<String>>,
    /// Local dev preview: every lesson and challenge is open, nothing is
    /// marked passed. Always off in release builds.
    pub preview: bool,
}

impl Learner {
    /// Loads saved progress, saves every change, and shares it with the app.
    pub fn provide() {
        let progress = RwSignal::new(storage::load_progress());
        Effect::new(move || progress.with(storage::save_progress));
        let unlocked =
            Memo::new(move |_| progress.with(|p| content::unlocked(content::graph(), p)));
        provide_context(Learner {
            progress,
            unlocked,
            preview: preview_mode(),
        });
    }

    pub fn get() -> Self {
        expect_context()
    }

    pub fn is_open(&self, concept: &str) -> bool {
        self.preview || self.unlocked.with(|u| u.contains(concept))
    }

    pub fn is_challenge_open(&self, chapter: &str) -> bool {
        self.preview
            || self
                .progress
                .with(|p| content::graph().is_challenge_open(chapter, p))
    }

    pub fn is_done(&self, concept: &str) -> bool {
        self.progress.with(|p| content::graph().is_done(concept, p))
    }
}

/// Debug builds only: `?preview=on` turns preview on and `?preview=off` turns
/// it off; the choice is remembered in the browser until changed.
fn preview_mode() -> bool {
    if !cfg!(debug_assertions) {
        return false;
    }
    let search = window().location().search().unwrap_or_default();
    if search.contains("preview=on") {
        storage::save_preview(true);
    } else if search.contains("preview=off") {
        storage::save_preview(false);
    }
    storage::load_preview()
}
