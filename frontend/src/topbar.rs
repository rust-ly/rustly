use leptos::prelude::*;
use leptos_router::components::A;

use crate::learner::Learner;
use crate::storage;

#[component]
pub fn TopBar() -> impl IntoView {
    let learner = Learner::get();
    let total = content::graph().concept_ids().count();
    let done = move || {
        content::graph()
            .concept_ids()
            .filter(|id| learner.is_done(id))
            .count()
    };
    view! {
        <header class="topbar">
            <div class="container topbar-inner">
                <A href="/" attr:class="wordmark" attr:aria-label="Rustly home">
                    "Rust"<span>"ly"</span>
                </A>
                <span class="chip chip-beta">"beta"</span>
                <nav aria-label="Tracks">
                    <a href="/#track-book">"Book"</a>
                    <a href="/#track-tokio">"Tokio"</a>
                </nav>
                <div class="topbar-actions">
                    <ThemeToggle />
                    <span class="chip progress-chip nums" title="Concepts passed">
                        {done} " / " {total}
                    </span>
                </div>
            </div>
        </header>
    }
}

/// Switches between the dark (default) and light themes. The inline script in
/// `index.html` applies the saved theme before the app loads.
#[component]
fn ThemeToggle() -> impl IntoView {
    let root = document().document_element().expect("<html> exists");
    let initial = root.get_attribute("data-theme").unwrap_or_default() == "light";
    let light = RwSignal::new(initial);
    let toggle = move |_| {
        let next = !light.get_untracked();
        let theme = if next { "light" } else { "dark" };
        let _ = root.set_attribute("data-theme", theme);
        storage::save_theme(theme);
        light.set(next);
    };
    view! {
        <button
            class="icon-btn"
            on:click=toggle
            aria-label=move || if light.get() { "Switch to dark theme" } else { "Switch to light theme" }
        >
            {move || if light.get() { "☾" } else { "☀" }}
        </button>
    }
}
