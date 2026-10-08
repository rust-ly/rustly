use leptos::prelude::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::path;

use crate::learner::Learner;
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
                    <Route path=path!("/learn/*rest") view=Learn />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <h1>"Hello Rustly"</h1>
        <p>"Learn Rust in the browser. The course map arrives in M4."</p>
        <ApiStatus />
        <A href="/learn/book/ch01">"Deep link test"</A>
    }
}

/// Placeholder so deep links can be checked against the SPA rewrite in M0.
#[component]
fn Learn() -> impl IntoView {
    let location = use_location();
    view! {
        <h1>"Rustly"</h1>
        <p>"You are at " <code>{move || location.pathname.get()}</code></p>
        <A href="/">"Home"</A>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <h1>"Not found"</h1>
        <A href="/">"Back to the course map"</A>
    }
}

/// Calls `/api/health` so M0 proves the frontend can reach the API.
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
