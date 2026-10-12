//! `./submit`: grades the learner's code, shows each hidden test's result,
//! saves the outcome and points the learner at whatever a pass unlocked.

use content::{Exercise, Unlocks};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use shared::SubmitResponse;

use crate::home::next_lesson;
use crate::icons::CheckIcon;
use crate::learner::Learner;
use crate::{api, routes};

#[derive(Clone)]
pub enum SubmitState {
    Idle,
    Grading,
    Done(SubmitResponse, Unlocks),
    Failed(String),
}

/// `./submit`. `code` is the learner's current code, e.g. the editor's
/// contents; the outcome goes to `state` for [`SubmitResults`].
#[component]
pub fn SubmitButton(
    exercise: &'static Exercise,
    #[prop(into)] code: Signal<String>,
    state: RwSignal<SubmitState>,
) -> impl IntoView {
    let learner = Learner::get();
    let grading = move || matches!(state.get(), SubmitState::Grading);

    let submit = move |_| {
        state.set(SubmitState::Grading);
        let code = code.get_untracked();
        spawn_local(async move {
            state.set(match api::submit(exercise.id.clone(), code).await {
                Ok(response) => {
                    let unlocks = learner.record_submit(&exercise.id, response.passed);
                    SubmitState::Done(response, unlocks)
                }
                Err(message) => SubmitState::Failed(message),
            });
        });
    };

    view! {
        <button class="btn btn-sm" on:click=submit disabled=grading>
            {move || if grading() { "grading…" } else { "./submit" }}
        </button>
    }
}

/// Each hidden test's result, and what a pass unlocked.
#[component]
pub fn SubmitResults(state: RwSignal<SubmitState>) -> impl IntoView {
    move || match state.get() {
        SubmitState::Idle | SubmitState::Grading => ().into_any(),
        SubmitState::Failed(message) => {
            view! { <p class="error-text submit" role="alert">{message}</p> }.into_any()
        }
        SubmitState::Done(response, unlocks) => {
            view! { <div class="submit"><Results response=response unlocks=unlocks /></div> }
                .into_any()
        }
    }
}

/// The program stopped at a `todo!()`: what running the untouched starter
/// code looks like, which is easy to mistake for a broken site.
pub fn stopped_at_todo(output: &str) -> bool {
    output.contains("not yet implemented")
}

/// Explains a `todo!()` stop in plain words.
#[component]
pub fn TodoNote() -> impl IntoView {
    view! {
        <p class="todo-note">
            "This is still the starter code. "<code>"todo!()"</code>
            " is a placeholder that stops the program: replace it with your answer."
        </p>
    }
}

#[component]
fn Results(response: SubmitResponse, unlocks: Unlocks) -> impl IntoView {
    let learner = Learner::get();
    let summary = if !response.compiled {
        "Your code didn't compile. Fix the errors below and submit again.".to_string()
    } else if response.passed {
        "Every test passed.".to_string()
    } else {
        let passed = response.tests.iter().filter(|t| t.passed).count();
        format!("{passed} of {} tests passed.", response.tests.len())
    };
    view! {
        <div class="submit-results" role="status">
            {response.passed.then(|| view! { <Unlocked unlocks=unlocks /> })}
            <p class=if response.passed { "submit-summary passed" } else { "submit-summary" }>{summary}</p>
            {response.tests.iter().any(|t| t.output.as_deref().is_some_and(stopped_at_todo)).then(|| view! { <TodoNote /> })}
            {(!response.compiled && !response.stderr.is_empty()).then(|| view! {
                <pre class="submit-stderr">{response.stderr.clone()}</pre>
            })}
            <ul class="test-list">
                {response.tests.into_iter().map(|test| view! {
                    <li class=if test.passed { "test passed" } else { "test failed" }>
                        <span class="test-mark" aria-hidden="true">{if test.passed { "✓" } else { "✗" }}</span>
                        <code>{test.name}</code>
                        <span class="visually-hidden">{if test.passed { " passed" } else { " failed" }}</span>
                        {test.output.map(|out| view! { <pre class="test-output">{out}</pre> })}
                    </li>
                }).collect_view()}
            </ul>
            {(response.passed && learner.preview).then(|| view! {
                <p class="muted small">"Preview mode: passes aren't saved."</p>
            })}
        </div>
    }
}

/// "Unlocked: Borrowing" and a Next button to the first thing that opened, or
/// to the next unfinished lesson when nothing new opened.
#[component]
fn Unlocked(unlocks: Unlocks) -> impl IntoView {
    let learner = Learner::get();
    let course = content::course();
    let concepts = unlocks.concepts.iter().filter_map(|id| course.concept(id));
    let challenges = unlocks
        .challenges
        .iter()
        .filter_map(|id| course.all_chapters().find(|c| &c.id == id));
    let mut names: Vec<String> = concepts.clone().map(|c| c.title.clone()).collect();
    names.extend(
        challenges
            .clone()
            .map(|c| format!("the {} challenge", c.title)),
    );
    let next = concepts
        .map(routes::concept_path)
        .chain(challenges.map(routes::challenge_path))
        .next()
        .unwrap_or_else(|| next_lesson(&learner));
    view! {
        <div class="unlock-toast">
            <CheckIcon />
            <span>
                {if names.is_empty() {
                    "Passed!".to_string()
                } else {
                    format!("Unlocked: {}", names.join(", "))
                }}
            </span>
            <A href=next attr:class="btn btn-sm">"next →"</A>
        </div>
    }
}
