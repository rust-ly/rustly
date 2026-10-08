use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::path;

use crate::chapter::ChapterPage;
use crate::course_map::LearnPage;
use crate::home::Home;
use crate::learner::Learner;
use crate::lesson::ConceptPage;
use crate::topbar::TopBar;

#[component]
pub fn App() -> impl IntoView {
    Learner::provide();
    view! {
        <Router>
            <TopBar />
            <main>
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/learn") view=|| view! { <div class="container"><LearnPage /></div> } />
                    <Route path=path!("/learn/:track/:chapter") view=|| view! { <div class="container"><ChapterPage /></div> } />
                    <Route path=path!("/learn/:track/:chapter/:concept") view=|| view! { <div class="container"><ConceptPage /></div> } />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
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
                    <span class="muted small">"A Rust capstone project"</span>
                </p>
                <div class="footer-credits small muted">
                    <p>
                        "Lessons adapted from "
                        <a href="https://doc.rust-lang.org/book/">"The Rust Programming Language"</a>
                        " (MIT / Apache-2.0) and the "
                        <a href="https://tokio.rs/tokio/tutorial">"Tokio tutorial"</a>
                        " (MIT)."
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
