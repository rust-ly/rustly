//! The home page: both tracks, their chapters and concepts, each marked
//! locked, open or done.

use content::{Chapter, Concept, Track};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_query_map;

use crate::icons::{CheckIcon, LockIcon};
use crate::learner::Learner;
use crate::routes;

/// `/learn`: both tracks with every chapter and concept.
#[component]
pub fn LearnPage() -> impl IntoView {
    view! {
        <LockedNotice />
        <section class="section page-head">
            <p class="eyebrow">"rustly::course_map"</p>
            <h1>"Your course"</h1>
            <p class="lede">"Pass a concept's checkpoint to open the next one. Finish a chapter's challenge to open the next chapter."</p>
        </section>
        <TrackSection track=Track::Book title="The Rust Book" />
        <TrackSection track=Track::Tokio title="Async with Tokio" />
    }
}

/// Shown when a locked page redirected here.
#[component]
fn LockedNotice() -> impl IntoView {
    let query = use_query_map();
    let learner = Learner::get();
    move || {
        let id = query.with(|q| q.get("locked"))?;
        let concept = content::course().concept(&id)?;
        let needs = unmet_requirements(&learner, concept);
        Some(view! {
            <div class="notice" role="status">
                <LockIcon />
                <span>
                    <strong>{concept.title.clone()}</strong>
                    " is locked. "
                    <code>{needs.join(", ")}</code>
                    " to unlock it."
                </span>
            </div>
        })
    }
}

#[component]
fn TrackSection(track: Track, title: &'static str) -> impl IntoView {
    let name = routes::track_name(track);
    let chapters = content::chapters(track);
    view! {
        <section class="section" id=format!("track-{name}")>
            <p class="eyebrow">{format!("rustly::track.{name}")}</p>
            <h2>{title}</h2>
            {if chapters.is_empty() {
                view! {
                    <div class="card card-locked track-soon">
                        <p class="lock-line"><LockIcon />" finish book chapters 10, 13 and 16 to unlock"</p>
                        <p class="muted">"The Tokio track is being written."</p>
                    </div>
                }
                .into_any()
            } else {
                view! {
                    <div class="card-grid">
                        {chapters.iter().map(|c| view! { <ChapterCard chapter=c /> }).collect_view()}
                    </div>
                }
                .into_any()
            }}
        </section>
    }
}

/// Where a learner stands in a chapter.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChapterState {
    Locked,
    Open,
    InProgress,
    Done,
}

pub fn chapter_state(learner: &Learner, chapter: &Chapter) -> ChapterState {
    let complete = learner
        .progress
        .with(|p| content::graph().is_complete(&chapter.id, p));
    let done = chapter
        .concepts
        .iter()
        .filter(|c| learner.is_done(&c.id))
        .count();
    if complete {
        ChapterState::Done
    } else if done > 0 {
        ChapterState::InProgress
    } else if chapter.concepts.iter().any(|c| learner.is_open(&c.id)) {
        ChapterState::Open
    } else {
        ChapterState::Locked
    }
}

#[component]
pub fn StateChip(state: ChapterState) -> impl IntoView {
    match state {
        ChapterState::Locked => {
            view! { <span class="chip chip-locked"><LockIcon />"locked"</span> }.into_any()
        }
        ChapterState::Open => view! { <span class="chip chip-open">"open"</span> }.into_any(),
        ChapterState::InProgress => {
            view! { <span class="chip chip-progress">"in progress"</span> }.into_any()
        }
        ChapterState::Done => {
            view! { <span class="chip chip-done"><CheckIcon />"passed"</span> }.into_any()
        }
    }
}

#[component]
fn ChapterCard(chapter: &'static Chapter) -> impl IntoView {
    let learner = Learner::get();
    let state = Memo::new(move |_| chapter_state(&learner, chapter));
    let class = move || match state.get() {
        ChapterState::Locked => "card card-locked",
        ChapterState::InProgress => "card card-progress",
        _ => "card",
    };
    view! {
        <article class=class>
            <div class="card-top">
                <p class="eyebrow">{format!("ch.{:02} // {}", chapter.order, routes::short_id(&chapter.id))}</p>
                {move || view! { <StateChip state=state.get() /> }}
            </div>
            <h3>
                {move || if state.get() == ChapterState::Locked {
                    chapter.title.clone().into_any()
                } else {
                    view! { <A href=routes::chapter_path(chapter)>{chapter.title.clone()}</A> }.into_any()
                }}
            </h3>
            <p class="muted small">{chapter.summary.clone()}</p>
            <ConceptList chapter=chapter />
        </article>
    }
}

/// A chapter's concepts with their state, linked when open.
#[component]
pub fn ConceptList(chapter: &'static Chapter) -> impl IntoView {
    view! {
        <ol class="concepts">
            {chapter.concepts.iter().map(|c| view! { <ConceptRow concept=c /> }).collect_view()}
        </ol>
    }
}

#[component]
fn ConceptRow(concept: &'static Concept) -> impl IntoView {
    let learner = Learner::get();
    move || {
        if learner.is_done(&concept.id) {
            view! {
                <li class="concept concept-done">
                    <CheckIcon /><A href=routes::concept_path(concept)>{concept.title.clone()}</A>
                    <span class="visually-hidden">" (passed)"</span>
                </li>
            }
            .into_any()
        } else if learner.is_open(&concept.id) {
            view! {
                <li class="concept concept-open">
                    <span class="dot" aria-hidden="true"></span>
                    <A href=routes::concept_path(concept)>{concept.title.clone()}</A>
                </li>
            }
            .into_any()
        } else {
            let needs = unmet_requirements(&learner, concept).join(", ");
            view! {
                <li class="concept concept-locked">
                    <LockIcon /><span>{concept.title.clone()}</span>
                    <span class="lock-line">{format!("{needs} to unlock")}</span>
                </li>
            }
            .into_any()
        }
    }
}

/// `pass ./ownership.moves`-style labels for what a concept still needs.
pub fn unmet_requirements(learner: &Learner, concept: &Concept) -> Vec<String> {
    let graph = content::graph();
    learner.progress.with(|p| {
        graph
            .concept_requires(&concept.id)
            .iter()
            .filter(|r| !(graph.is_done(r, p) || graph.is_complete(r, p)))
            .map(|r| format!("pass {}", routes::requirement_label(r)))
            .collect()
    })
}
