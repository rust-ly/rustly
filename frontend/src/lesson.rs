//! `/learn/:track/:chapter/:concept`: a concept lesson, and the chapter
//! challenge at `/learn/:track/:chapter/challenge`.

use content::{Chapter, Concept, Exercise, LessonBlock};
use leptos::prelude::*;
use leptos_router::components::{A, Redirect};
use leptos_router::hooks::use_params_map;

use crate::learner::Learner;
use crate::snippet::SnippetView;
use crate::{NotFound, highlight, routes};

#[component]
pub fn ConceptPage() -> impl IntoView {
    let params = use_params_map();
    move || {
        let (track, chapter, concept) =
            params.with(|p| (p.get("track"), p.get("chapter"), p.get("concept")));
        let Some(chapter) =
            routes::find_chapter(&track.unwrap_or_default(), &chapter.unwrap_or_default())
        else {
            return view! { <NotFound /> }.into_any();
        };
        let concept = concept.unwrap_or_default();
        if concept == "challenge" {
            return view! { <ChallengeLesson chapter=chapter /> }.into_any();
        }
        match routes::find_concept(chapter, &concept) {
            Some(concept) => view! { <ConceptLesson chapter=chapter concept=concept /> }.into_any(),
            None => view! { <NotFound /> }.into_any(),
        }
    }
}

#[component]
fn ConceptLesson(chapter: &'static Chapter, concept: &'static Concept) -> impl IntoView {
    let learner = Learner::get();
    // A memo, so progress changes only re-render the page if the lock changes.
    let open = Memo::new(move |_| learner.is_open(&concept.id));
    Effect::new(move || {
        if open.get()
            && !learner
                .progress
                .with_untracked(|p| p.concepts_read.contains(&concept.id))
        {
            learner.progress.update(|p| {
                p.concepts_read.insert(concept.id.clone());
            });
        }
    });
    move || {
        if !open.get() {
            return view! { <Redirect path=routes::locked_redirect(&concept.id) /> }.into_any();
        }
        view! {
            <div class="lesson">
                <article class="lesson-prose">
                    <p class="eyebrow">
                        <A href=routes::chapter_path(chapter)>{format!("ch.{:02} // {}", chapter.order, routes::short_id(&chapter.id))}</A>
                        {format!(" / {}", concept.order)}
                    </p>
                    <LessonBody concept=concept />
                    <p class="source-link">
                        <a href=concept.source_url.clone() target="_blank" rel="noopener">"Read this section in the original ↗"</a>
                    </p>
                </article>
                <aside class="lesson-side">
                    <ExercisePanel label="checkpoint //" exercise=&concept.checkpoint />
                </aside>
            </div>
        }
        .into_any()
    }
}

#[component]
fn ChallengeLesson(chapter: &'static Chapter) -> impl IntoView {
    let learner = Learner::get();
    let open = Memo::new(move |_| {
        learner
            .progress
            .with(|p| content::graph().is_challenge_open(&chapter.id, p))
    });
    move || {
        if !open.get() {
            // The chapter page explains what's left before the challenge.
            return view! { <Redirect path=routes::chapter_path(chapter) /> }.into_any();
        }
        view! {
            <div class="lesson">
                <article class="lesson-prose">
                    <p class="eyebrow">
                        <A href=routes::chapter_path(chapter)>{format!("ch.{:02} // {}", chapter.order, routes::short_id(&chapter.id))}</A>
                        " / challenge"
                    </p>
                    <h1>{chapter.challenge.title.clone()}</h1>
                    <div class="prose" inner_html=content::markdown_html(&chapter.challenge.prompt)></div>
                </article>
                <aside class="lesson-side">
                    <ExercisePanel label="challenge //" exercise=&chapter.challenge />
                </aside>
            </div>
        }
        .into_any()
    }
}

/// The lesson markdown with each snippet rendered as a live component.
#[component]
fn LessonBody(concept: &'static Concept) -> impl IntoView {
    content::lesson_blocks(&concept.body_md)
        .into_iter()
        .map(|block| match block {
            LessonBlock::Html(html) => {
                view! { <div class="prose" inner_html=html></div> }.into_any()
            }
            LessonBlock::Snippet(i) => {
                view! { <SnippetView snippet=&concept.snippets[i] /> }.into_any()
            }
        })
        .collect_view()
}

/// The exercise for this page. The editor and grading arrive in M5–M7; until
/// then the panel shows the task and the starter code.
#[component]
fn ExercisePanel(label: &'static str, exercise: &'static Exercise) -> impl IntoView {
    view! {
        <section class="exercise card">
            <p class="eyebrow">{label}</p>
            <h2 class="exercise-title">{exercise.title.clone()}</h2>
            <div class="prose small-prose" inner_html=content::markdown_html(&exercise.prompt)></div>
            <pre class="snippet-code starter"><code>
                {exercise.starter.trim().lines().map(|text| view! {
                    <span class="line">
                        {highlight::line(text).into_iter().map(|(t, piece)| view! { <span class=t.class()>{piece}</span> }).collect_view()}
                        "\n"
                    </span>
                }).collect_view()}
            </code></pre>
            <p class="muted small">"The code editor and ./submit arrive in the next milestone."</p>
        </section>
    }
}
