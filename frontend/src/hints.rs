//! `HintDrawer`: an exercise's hints, one at a time, then its solution once
//! the learner has failed enough submits.

use content::Exercise;
use leptos::prelude::*;
use leptos::task::spawn_local;
use shared::SOLUTION_AFTER_FAILS;

use crate::api;
use crate::learner::Learner;

#[derive(Clone)]
enum SolutionState {
    Hidden,
    Loading,
    Shown(String),
    Failed(String),
}

#[component]
pub fn HintDrawer(exercise: &'static Exercise, code: RwSignal<String>) -> impl IntoView {
    let learner = Learner::get();
    let total = exercise.hints.len();
    let revealed = RwSignal::new(0usize);
    let solution = RwSignal::new(SolutionState::Hidden);
    let failed = Memo::new(move |_| {
        learner
            .progress
            .with(|p| p.failed_submits.get(&exercise.id).copied().unwrap_or(0))
    });
    let solution_open = move || failed.get() >= SOLUTION_AFTER_FAILS;

    let show_solution = move |_| {
        solution.set(SolutionState::Loading);
        spawn_local(async move {
            solution.set(
                match api::solution(exercise.id.clone(), failed.get_untracked()).await {
                    Ok(code) => SolutionState::Shown(code),
                    Err(message) => SolutionState::Failed(message),
                },
            );
        });
    };

    view! {
        <div class="hints">
            <ol class="hint-list">
                {move || exercise.hints[..revealed.get()]
                    .iter()
                    .map(|hint| view! { <li class="prose small-prose" inner_html=content::markdown_html(hint)></li> })
                    .collect_view()}
            </ol>
            <div class="hint-actions">
                {move || (revealed.get() < total).then(|| view! {
                    <button class="btn btn-secondary btn-sm" on:click=move |_| revealed.update(|n| *n += 1)>
                        {format!("Show hint {} of {total}", revealed.get() + 1)}
                    </button>
                })}
                {move || match solution.get() {
                    SolutionState::Hidden | SolutionState::Failed(_) => {
                        let label = if solution_open() {
                            "Show solution".to_string()
                        } else {
                            format!(
                                "Solution opens after {SOLUTION_AFTER_FAILS} failed submits ({}/{SOLUTION_AFTER_FAILS})",
                                failed.get()
                            )
                        };
                        view! {
                            <button class="btn btn-secondary btn-sm" on:click=show_solution disabled=move || !solution_open()>
                                {label}
                            </button>
                        }.into_any()
                    }
                    SolutionState::Loading => view! {
                        <button class="btn btn-secondary btn-sm" disabled=true>"loading…"</button>
                    }.into_any(),
                    SolutionState::Shown(_) => ().into_any(),
                }}
            </div>
            {move || match solution.get() {
                SolutionState::Shown(answer) => {
                    let markdown = format!("```rust\n{}\n```", answer.trim_end());
                    let fill = move |_| code.set(format!("{}\n", answer.trim()));
                    Some(view! {
                        <div class="solution">
                            <p class="eyebrow">"solution //"</p>
                            <div class="prose" inner_html=content::markdown_html(&markdown)></div>
                            <button class="btn btn-secondary btn-sm" on:click=fill>"Put it in the editor"</button>
                        </div>
                    }.into_any())
                }
                SolutionState::Failed(message) => Some(view! { <p class="muted small">{message}</p> }.into_any()),
                _ => None,
            }}
        </div>
    }
}
