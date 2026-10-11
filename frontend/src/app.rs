use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::path;
use wasm_bindgen::JsCast;

use crate::chapter::ChapterPage;
use crate::course_map::LearnPage;
use crate::credits::CreditsPage;
use crate::home::Home;
use crate::learner::Learner;
use crate::lesson::ConceptPage;
use crate::snippets::SnippetsPage;
use crate::topbar::TopBar;

#[component]
pub fn App() -> impl IntoView {
    Learner::provide();
    view! {
        <Router>
            <SkipLink />
            <TopBar />
            <main id="main" tabindex="-1">
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/learn") view=|| view! { <div class="container"><LearnPage /></div> } />
                    <Route path=path!("/learn/:track/:chapter") view=|| view! { <div class="container"><ChapterPage /></div> } />
                    <Route path=path!("/credits") view=|| view! { <div class="container"><CreditsPage /></div> } />
                    <Route path=path!("/snippets") view=|| view! { <div class="container"><SnippetsPage /></div> } />
                    <Route path=path!("/learn/:track/:chapter/:concept") view=|| view! { <div class="container"><ConceptPage /></div> } />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}

/// The first thing Tab reaches: jumps past the top bar to the page content.
/// Focuses `<main>` directly, because the router would treat `#main` as a
/// navigation.
#[component]
fn SkipLink() -> impl IntoView {
    let skip = |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        if let Some(main) = document().get_element_by_id("main") {
            let _ = main.unchecked_into::<web_sys::HtmlElement>().focus();
        }
    };
    view! { <a class="skip-link" href="#main" on:click=skip>"Skip to content"</a> }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <section class="container section page-head">
            <p class="eyebrow">"error[404]::not_found"</p>
            <h1>"There's nothing here."</h1>
            <p><A href="/">"Back to the course map"</A></p>
        </section>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container footer-inner">
                <p class="footer-brand">
                    <img class="avatar" src="/images/rustly-crab.jpg" width="28" height="28" alt="" />
                    <span class="wordmark-sm">"Rust"<span>"ly"</span></span>
                </p>
                <div class="footer-credits small muted">
                    <p>
                        "Lessons adapted from "
                        <a href="https://doc.rust-lang.org/book/">"The Rust Programming Language"</a>
                        " (MIT / Apache-2.0) and the "
                        <a href="https://tokio.rs/tokio/tutorial">"Tokio tutorial"</a>
                        " (MIT). "
                        <A href="/credits">"Credits and licenses"</A>
                    </p>
                    <ApiStatus />
                </div>
            </div>
        </footer>
    }
}

/// Calls `/api/health` so it's easy to see whether the API is reachable.
#[component]
fn ApiStatus() -> impl IntoView {
    let health = LocalResource::new(|| async {
        let resp = gloo_net::http::Request::get("/api/health")
            .send()
            .await
            .map_err(|e| e.to_string())?;
        resp.json::<serde_json::Value>()
            .await
            .map_err(|e| e.to_string())
    });
    view! {
        <p>
            "API: "
            {move || match health.get() {
                None => "checking…".to_string(),
                Some(Ok(v)) if v["ok"] == true => "ok".to_string(),
                Some(Ok(v)) => format!("unexpected response {v}"),
                Some(Err(e)) => format!("unreachable ({e})"),
            }}
        </p>
    }
}
