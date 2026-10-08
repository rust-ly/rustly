//! The learner's progress as app-wide reactive state.

use content::Progress;
use leptos::prelude::*;

use crate::storage;

#[derive(Clone, Copy)]
pub struct Learner {
    pub progress: RwSignal<Progress>,
}

impl Learner {
    /// Loads saved progress, saves every change, and shares it with the app.
    pub fn provide() {
        let progress = RwSignal::new(storage::load_progress());
        Effect::new(move || progress.with(storage::save_progress));
        provide_context(Learner { progress });
    }

    pub fn get() -> Self {
        expect_context()
    }

    pub fn is_done(&self, concept: &str) -> bool {
        self.progress.with(|p| content::graph().is_done(concept, p))
    }
}
