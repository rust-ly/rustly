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
}

impl Learner {
    /// Loads saved progress, saves every change, and shares it with the app.
    pub fn provide() {
        let progress = RwSignal::new(storage::load_progress());
        Effect::new(move || progress.with(storage::save_progress));
        let unlocked =
            Memo::new(move |_| progress.with(|p| content::unlocked(content::graph(), p)));
        provide_context(Learner { progress, unlocked });
    }

    pub fn get() -> Self {
        expect_context()
    }

    pub fn is_open(&self, concept: &str) -> bool {
        self.unlocked.with(|u| u.contains(concept))
    }

    pub fn is_done(&self, concept: &str) -> bool {
        self.progress.with(|p| content::graph().is_done(concept, p))
    }
}
