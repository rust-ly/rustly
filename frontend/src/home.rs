//! `/`: the landing page, built from the Rustly landing page mockup.

use content::Track;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::icons::LockIcon;
use crate::learner::Learner;
use crate::routes;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Hero />
        <HowItWorks />
        <LiveCheck />
        <Tracks />
        <StartSection />
    }
}

/// The first open lesson the learner hasn't passed, or the very first one.
pub fn next_lesson(learner: &Learner) -> String {
    let course = content::course();
    course
        .concepts()
        .find(|c| learner.is_open(&c.id) && !learner.is_done(&c.id))
        .or_else(|| course.concepts().next())
        .map_or_else(|| "/learn".to_string(), routes::concept_path)
}

#[component]
fn Hero() -> impl IntoView {
    let learner = Learner::get();
    view! {
        <section class="hero">
            <div class="hero-glyphs" aria-hidden="true">
                <span style="left:3%;top:10%">"{"</span>
                <span style="left:46%;top:8%">"&mut"</span>
                <span style="left:56%;top:30%">"⇒"</span>
                <span style="right:4%;top:12%">"}"</span>
                <span style="right:3%;top:58%">"'a"</span>
                <span style="left:2%;top:68%">"→"</span>
                <span style="left:30%;top:80%">"#[derive(Debug)]"</span>
            </div>
            <div class="container hero-inner">
                <div class="hero-text">
                    <p class="hero-chip"><span class="dot-shell"></span>"The Rust Book + the Tokio tutorial, in your browser"</p>
                    <h1 class="hero-wordmark">"Rust"<span>"ly"</span></h1>
                    <p class="hero-pitch">"Learn Rust by writing it, one passing exercise at a time."</p>
                    <p class="hero-sub">
                        "Short lessons, an editor that checks your code with the real compiler as you type, and exercises that unlock the next idea when you pass them."
                    </p>
                    <div class="hero-actions">
                        <A href=move || next_lesson(&learner) attr:class="btn btn-ink">"cargo run --learn"</A>
                        <a href="#how-it-works" class="hero-link">"See how it works"</a>
                    </div>
                </div>
                <div class="hero-visual">
                    <img
                        class="hero-photo"
                        src="/images/rustly-crab.jpg"
                        width="720"
                        height="720"
                        alt="A blue crab with red claw tips under a red scribbled circle, above the word Rustly"
                    />
                    <div class="code-card hero-code" role="img" aria-label="A snippet printing hello, rustacean with 0 errors">
                        <div class="code-card-head">
                            <span>"main.rs"</span>
                            <span class="status-ok"><span class="dot-sand"></span>"0 errors"</span>
                        </div>
                        <pre class="code-card-body">
                            <span class="tok-kw">"let"</span>" name = "<span class="tok-str">"\"rustacean\""</span>";\n"
                            <span class="tok-mac">"println!"</span>"("<span class="tok-str">"\"hello, {name}\""</span>");"
                        </pre>
                        <div class="code-card-out">"hello, rustacean"</div>
                    </div>
                </div>
            </div>
            <svg class="hero-wave" viewBox="0 0 1440 80" preserveAspectRatio="none" aria-hidden="true">
                <path class="wave-fill" d="M0 40 C 360 0, 720 80, 1080 40 S 1440 20, 1440 20 V80 H0 Z" />
                <path class="wave-line" d="M0 40 C 360 0, 720 80, 1080 40 S 1440 20, 1440 20" />
            </svg>
        </section>
    }
}

#[component]
fn HowItWorks() -> impl IntoView {
    let steps = [
        (
            "1",
            "step-tide",
            "Read a short lesson",
            "One idea at a time, adapted from the Rust Book or the Tokio tutorial, with a link to the full chapter.",
        ),
        (
            "2",
            "step-sand",
            "Run the snippets",
            "Edit and run every example. The ones that don't compile show you the real compiler error.",
        ),
        (
            "3",
            "step-amber",
            "Pass the checkpoint",
            "Write the code, get live feedback as you type, then submit it against hidden tests.",
        ),
        (
            "4",
            "step-coral",
            "Unlock the next concept",
            "Each pass opens the next idea. Finish a chapter's challenge to open the next chapter.",
        ),
    ];
    view! {
        <section class="band" id="how-it-works">
            <div class="container">
                <p class="eyebrow">"rustly::how_it_works"</p>
                <h2 class="band-title">"Read it. Run it. Pass it."<br />"Unlock the next one."</h2>
                <p class="band-lede">
                    "Every concept is a short lesson, a few code snippets you can run, and one checkpoint exercise. Pass the checkpoint and the next concept opens."
                </p>
                <ol class="steps">
                    {steps
                        .into_iter()
                        .map(|(n, class, title, text)| view! {
                            <li class="step">
                                <span class=format!("step-num {class}") aria-hidden="true">{n}</span>
                                <h3>{title}</h3>
                                <p>{text}</p>
                            </li>
                        })
                        .collect_view()}
                </ol>
            </div>
        </section>
    }
}

#[component]
fn LiveCheck() -> impl IntoView {
    view! {
        <section class="band band-alt">
            <div class="container split">
                <div>
                    <p class="eyebrow">"rustly::live_check"</p>
                    <h2 class="band-title">"The compiler talks to you while you type."</h2>
                    <p class="band-lede">
                        "About a second after you stop typing, Rustly checks your code with the real Rust compiler and underlines the problem right where it is, with the same message rustc would print."
                    </p>
                    <p class="pills">
                        <span class="pill pill-sand">"runnable"</span>
                        <span class="pill pill-amber">"editable"</span>
                        <span class="pill pill-shell">"won't compile"</span>
                        <span class="pill pill-tide">"async"</span>
                    </p>
                </div>
                <div class="code-card editor-demo" role="img" aria-label="The editor showing error E0382, borrow of moved value s1, on line 4">
                    <div class="code-card-head">
                        <span>"Ownership · Moves · checkpoint"</span>
                        <span class="status-err"><span class="dot-coral"></span>"1 error"</span>
                    </div>
                    <pre class="code-card-body numbered">
                        <span class="ln">"1"</span><span class="tok-kw">"fn"</span>" main() {\n"
                        <span class="ln">"2"</span>"    "<span class="tok-kw">"let"</span>" s1 = "<span class="tok-ty">"String"</span>"::from("<span class="tok-str">"\"hello\""</span>");\n"
                        <span class="ln">"3"</span>"    "<span class="tok-kw">"let"</span>" s2 = s1;\n"
                        <span class="ln">"4"</span>"    "<span class="tok-mac">"println!"</span>"("<span class="tok-str">"\""<span class="squiggle">"{s1}"</span>", world!\""</span>");\n"
                        <span class="ln">"5"</span>"}"
                    </pre>
                    <div class="tabs">
                        <span>"Output"</span>
                        <span class="tab-active">"Problems (1)"</span>
                        <span>"Tests"</span>
                    </div>
                    <div class="problem">
                        <p><span class="err-code">"error[E0382]"</span>": borrow of moved value: `s1`"</p>
                        <p class="muted">"--> src/main.rs:4:16"</p>
                        <p class="muted">"value moved into `s2` on line 3. "<span class="hint-link">"Show hint"</span></p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Tracks() -> impl IntoView {
    let book = content::chapters(Track::Book);
    view! {
        <section class="band">
            <div class="container">
                <p class="eyebrow">"rustly::tracks"</p>
                <h2 class="band-title">"Start with the Book. Then go async."</h2>
                <div class="track-cards">
                    <article class="track-card">
                        <div class="track-top">
                            <span class="track-label">"Track 1"</span>
                            <span class="pill pill-solid-sand">"Open"</span>
                        </div>
                        <h3>"The Rust Book"</h3>
                        <p class="muted">
                            "Ownership, borrowing, structs, enums, errors, traits, iterators and threads. Follows The Rust Programming Language, chapter by chapter."
                        </p>
                        <p class="track-meta">
                            <span>{format!("{} chapters", book.len())}</span>
                            <span class="muted">"Beginner"</span>
                        </p>
                        <A href="/learn" attr:class="btn btn-sans">"Start the Book"</A>
                    </article>
                    <article class="track-card track-card-tide">
                        <div class="track-top">
                            <span class="track-label track-label-tide">"Track 2"</span>
                            <span class="pill pill-outline"><LockIcon />"Locked"</span>
                        </div>
                        <h3 class="tide">"Async with Tokio"</h3>
                        <p class="muted">
                            "Spawning tasks, shared state, channels, select and streams. Follows the Tokio tutorial, run in memory in your browser."
                        </p>
                        <p class="track-meta">
                            <span>"Being written"</span>
                            <span class="muted">"Intermediate"</span>
                        </p>
                        <p class="muted small">"Finish Book chapters 10, 13 and 16 to unlock."</p>
                    </article>
                </div>
            </div>
        </section>
    }
}

#[component]
fn StartSection() -> impl IntoView {
    let learner = Learner::get();
    view! {
        <section class="band band-alt">
            <div class="container split split-start">
                <div>
                    <p class="eyebrow">"rustly::start"</p>
                    <h2 class="band-title">"Ready to meet the borrow checker?"</h2>
                    <p class="band-lede">"No account needed. Your progress and starred snippets stay in your browser."</p>
                    <div class="hero-actions">
                        <A href=move || next_lesson(&learner) attr:class="btn">"cargo run --learn"</A>
                        <A href="/learn" attr:class="tide-link">"or browse the course map →"</A>
                    </div>
                </div>
                <img class="start-photo" src="/images/rustly-crab.jpg" width="720" height="720" alt="" />
            </div>
        </section>
    }
}
