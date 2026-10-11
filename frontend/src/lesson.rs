//! `/learn/:track/:chapter/:concept`: a concept lesson, the chapter challenge
//! at `/learn/:track/:chapter/challenge` and its test-out at `.../test-out`.

use content::{Chapter, Concept, Exercise, LessonBlock};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::{A, Redirect};
use leptos_router::hooks::use_params_map;
use shared::{RunResponse, Severity};

use crate::editor::CodeEditor;
use crate::hints::HintDrawer;
use crate::learner::Learner;
use crate::snippet::{SnippetView, program_stderr};
use crate::submit::{SubmitButton, SubmitResults, SubmitState};
use crate::{NotFound, api, routes, storage};

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
        if concept == "test-out" {
            return match &chapter.test_out {
                Some(test_out) => {
                    view! { <TestOutLesson chapter=chapter test_out=test_out /> }.into_any()
                }
                None => view! { <NotFound /> }.into_any(),
            };
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
    let open = Memo::new(move |_| learner.is_challenge_open(&chapter.id));
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

/// Open whenever the chapter itself is: passing it completes the chapter.
#[component]
fn TestOutLesson(chapter: &'static Chapter, test_out: &'static Exercise) -> impl IntoView {
    view! {
        <div class="lesson">
            <article class="lesson-prose">
                <p class="eyebrow">
                    <A href=routes::chapter_path(chapter)>{format!("ch.{:02} // {}", chapter.order, routes::short_id(&chapter.id))}</A>
                    " / test-out"
                </p>
                <h1>{test_out.title.clone()}</h1>
                <div class="prose" inner_html=content::markdown_html(&test_out.prompt)></div>
            </article>
            <aside class="lesson-side">
                <ExercisePanel label="test-out //" exercise=test_out />
            </aside>
        </div>
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

/// The exercise for this page: the task, the editor, Run and Submit. Drafts
/// are saved as the learner types, so a reload keeps their code.
#[component]
fn ExercisePanel(label: &'static str, exercise: &'static Exercise) -> impl IntoView {
    let starter = format!("{}\n", exercise.starter.trim());
    let code = RwSignal::new(storage::load_draft(&exercise.id).unwrap_or_else(|| starter.clone()));
    let state = RwSignal::new(RunState::Idle);
    let submit = RwSignal::new(SubmitState::Idle);
    {
        let starter = starter.clone();
        Effect::new(move || {
            code.with(|code| {
                if *code == starter {
                    storage::clear_draft(&exercise.id);
                } else {
                    storage::save_draft(&exercise.id, code);
                }
            });
        });
    }

    let running = move || matches!(state.get(), RunState::Running);
    let run = move |_| {
        state.set(RunState::Running);
        spawn_local(async move {
            state.set(match api::run(code.get_untracked()).await {
                Ok(response) => RunState::Done(response),
                Err(message) => RunState::Failed(message),
            });
        });
    };
    let reset = move |_| {
        code.set(starter.clone());
        state.set(RunState::Idle);
        submit.set(SubmitState::Idle);
    };
    // Compile errors from the last run, shown as squiggles until the next one.
    let diagnostics = Signal::derive(move || match state.get() {
        RunState::Done(r) => r.diagnostics,
        _ => Vec::new(),
    });

    view! {
        <section class="exercise card">
            <p class="eyebrow">{label}</p>
            <h2 class="exercise-title">{exercise.title.clone()}</h2>
            <div class="prose small-prose" inner_html=content::markdown_html(&exercise.prompt)></div>
            <CodeEditor code=code diagnostics=diagnostics />
            <div class="exercise-actions">
                <button class="btn btn-secondary btn-sm" on:click=reset disabled=running>"./reset"</button>
                <button class="btn btn-sm" on:click=run disabled=running>
                    {move || if running() { "running…" } else { "./run" }}
                </button>
                <SubmitButton exercise=exercise code=code state=submit />
            </div>
            <RunOutput state=state />
            <SubmitResults state=submit />
            <HintDrawer exercise=exercise code=code />
        </section>
    }
}

#[derive(Clone)]
enum RunState {
    Idle,
    Running,
    Done(RunResponse),
    Failed(String),
}

/// What the last Run printed, or why it didn't compile.
#[component]
fn RunOutput(state: RwSignal<RunState>) -> impl IntoView {
    move || match state.get() {
        RunState::Idle | RunState::Running => None,
        RunState::Failed(message) => Some(
            view! { <div class="snippet-output output-error" role="status">{message}</div> }
                .into_any(),
        ),
        RunState::Done(r) if r.success => {
            let text = if r.stdout.is_empty() {
                "(no output)".to_string()
            } else {
                r.stdout
            };
            Some(
                view! { <pre class="snippet-output output-ok" role="status">{text}</pre> }
                    .into_any(),
            )
        }
        RunState::Done(r) => {
            let errors: Vec<_> = r
                .diagnostics
                .into_iter()
                .filter(|d| d.severity == Severity::Error)
                .collect();
            Some(if errors.is_empty() {
                view! {
                    <pre class="snippet-output output-error" role="status">{r.stdout}{program_stderr(&r.stderr)}</pre>
                }
                .into_any()
            } else {
                view! {
                    <div class="snippet-output output-error" role="status">
                        {errors.into_iter().map(|d| {
                            let code = d.code.map_or(String::new(), |c| format!("[{c}]"));
                            view! { <p><strong>{format!("error{code}")}</strong>": "{d.message}{format!(" (line {})", d.line)}</p> }
                        }).collect_view()}
                    </div>
                }
                .into_any()
            })
        }
    }
}
