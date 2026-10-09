use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

use crate::home::next_lesson;
use crate::learner::Learner;
use crate::storage;

#[component]
pub fn TopBar() -> impl IntoView {
    let learner = Learner::get();
    let location = use_location();
    let on_learn = move || location.pathname.with(|p| p.starts_with("/learn"));
    view! {
        <header class="topbar">
            <div class="container topbar-inner">
                <A href="/" attr:class="brand" attr:aria-label="Rustly home">
                    <img class="avatar" src="/images/rustly-crab.jpg" width="36" height="36" alt="" />
                    <span class="wordmark">"Rust"<span>"ly"</span></span>
                </A>
                <nav aria-label="Main">
                    <a href="/learn#track-book" class:active=on_learn>"The Book"</a>
                    <a href="/learn#track-tokio">"Tokio"</a>
                </nav>
                <div class="topbar-actions">
                    {learner.preview.then(|| view! {
                        <a class="pill pill-outline" href="?preview=off" title="Local dev preview: everything is open. Click to turn off.">"Preview"</a>
                    })}
                    <ThemeToggle />
                    <A href=move || next_lesson(&learner) attr:class="btn btn-sans">"Start learning"</A>
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
            class="icon-btn round"
            on:click=toggle
            aria-label=move || if light.get() { "Switch to dark theme" } else { "Switch to light theme" }
        >
            {move || if light.get() { "☾" } else { "☀" }}
        </button>
    }
}
