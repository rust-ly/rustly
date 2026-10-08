use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::path;

use crate::chapter::ChapterPage;
use crate::course_map::Home;
use crate::learner::Learner;
use crate::lesson::ConceptPage;
use crate::topbar::TopBar;

#[component]
pub fn App() -> impl IntoView {
    Learner::provide();
    view! {
        <Router>
            <TopBar />
            <main class="container">
                <Routes fallback=NotFound>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/learn/:track/:chapter") view=ChapterPage />
                    <Route path=path!("/learn/:track/:chapter/:concept") view=ConceptPage />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <section class="section page-head">
            <p class="eyebrow">"error[404] // not found"</p>
            <h1>"There's nothing here."</h1>
            <p><A href="/">"Back to the course map"</A></p>
        </section>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container footer-inner small muted">
                <p>
                    "Lessons adapted from "
                    <a href="https://doc.rust-lang.org/book/">"The Rust Programming Language"</a>
                    " (MIT / Apache-2.0) and the "
                    <a href="https://tokio.rs/tokio/tutorial">"Tokio tutorial"</a>
                    " (MIT)."
                </p>
                <ApiStatus />
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
