//! `/credits`: the projects Rustly adapts and builds on, with their licenses.
//! The full license texts are in `THIRD_PARTY_LICENSES.md`.

use leptos::prelude::*;

const LICENSES_URL: &str = "https://github.com/rust-ly/rustly/blob/main/THIRD_PARTY_LICENSES.md";

struct Credit {
    name: &'static str,
    url: &'static str,
    by: &'static str,
    license: &'static str,
    used_for: &'static str,
}

const COURSE: &[Credit] = &[
    Credit {
        name: "The Rust Programming Language",
        url: "https://doc.rust-lang.org/book/",
        by: "The Rust Project Developers",
        license: "MIT OR Apache-2.0",
        used_for: "Track 1. Each Book lesson, snippet and exercise is adapted from it, and links to the chapter it follows.",
    },
    Credit {
        name: "The Tokio tutorial",
        url: "https://tokio.rs/tokio/tutorial",
        by: "Carl Lerche and the Tokio website contributors",
        license: "MIT",
        used_for: "Track 2. The async lessons follow its sections, run in memory instead of over a network.",
    },
];

const TOOLS: &[Credit] = &[
    Credit {
        name: "Rust Playground",
        url: "https://play.rust-lang.org",
        by: "The Rust Project",
        license: "MIT OR Apache-2.0",
        used_for: "Compiles, runs and tests the code you write.",
    },
    Credit {
        name: "Monaco Editor",
        url: "https://microsoft.github.io/monaco-editor/",
        by: "Microsoft",
        license: "MIT",
        used_for: "The exercise editor.",
    },
    Credit {
        name: "Leptos",
        url: "https://leptos.dev",
        by: "Greg Johnston and contributors",
        license: "MIT",
        used_for: "The Rust framework this site is built with.",
    },
    Credit {
        name: "Satoshi",
        url: "https://www.fontshare.com/fonts/satoshi",
        by: "Indian Type Foundry",
        license: "ITF Free Font License",
        used_for: "Headings and body text.",
    },
    Credit {
        name: "JetBrains Mono",
        url: "https://www.jetbrains.com/lp/mono/",
        by: "The JetBrains Mono Project Authors",
        license: "SIL Open Font License 1.1",
        used_for: "Code.",
    },
];

#[component]
pub fn CreditsPage() -> impl IntoView {
    view! {
        <section class="section page-head">
            <p class="eyebrow">"rustly::credits"</p>
            <h1>"Credits"</h1>
            <p class="lede">"Rustly's lessons are written in our own words, but they follow two openly licensed projects and run on open-source tools. Thank you to everyone behind them."</p>
        </section>
        <section class="section prose">
            <CreditList title="Course material" credits=COURSE />
            <CreditList title="Tools and fonts" credits=TOOLS />
            <h2>"License texts"</h2>
            <p>
                "The full copyright and license notices are in "
                <a href=LICENSES_URL>"THIRD_PARTY_LICENSES.md"</a>
                "."
            </p>
        </section>
    }
}

#[component]
fn CreditList(title: &'static str, credits: &'static [Credit]) -> impl IntoView {
    view! {
        <h2>{title}</h2>
        <ul class="credit-list">
            {credits.iter().map(|c| view! {
                <li>
                    <a href=c.url>{c.name}</a>
                    <span class="muted">{format!(" by {} · {}", c.by, c.license)}</span>
                    <br />
                    {c.used_for}
                </li>
            }).collect_view()}
        </ul>
    }
}
