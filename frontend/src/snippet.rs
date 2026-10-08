//! One component for all five snippet kinds.

use std::collections::BTreeSet;

use content::{Snippet, SnippetKind};
use leptos::prelude::*;
use leptos::task::spawn_local;
use shared::{RunResponse, Severity};

use crate::{api, highlight};

#[derive(Clone)]
enum RunState {
    Idle,
    Running,
    Done(RunResponse),
    Failed(String),
}

#[component]
pub fn SnippetView(snippet: &'static Snippet) -> impl IntoView {
    let kind = snippet.kind;
    let edited = RwSignal::new(snippet.visible_code.clone());
    let state = RwSignal::new(RunState::Idle);

    let run = move |_| {
        let code = match kind {
            SnippetKind::Editable => with_hidden_lines(snippet, &edited.get_untracked()),
            _ => snippet.code.clone(),
        };
        state.set(RunState::Running);
        spawn_local(async move {
            state.set(match api::run(code).await {
                Ok(response) => RunState::Done(response),
                Err(message) => RunState::Failed(message),
            });
        });
    };
    let reset = move |_| {
        edited.set(snippet.visible_code.clone());
        state.set(RunState::Idle);
    };
    let copy = move |_| {
        let text = match kind {
            SnippetKind::Editable => edited.get_untracked(),
            _ => snippet.visible_code.clone(),
        };
        let _ = window().navigator().clipboard().write_text(&text);
    };

    // Visible lines with a compiler error, for highlighting.
    let error_lines = move || match state.get() {
        RunState::Done(r) => visible_error_lines(snippet, &r),
        _ => BTreeSet::new(),
    };
    let running = move || matches!(state.get(), RunState::Running);

    view! {
        <figure class="snippet">
            <figcaption class="snippet-head">
                <span class="snippet-title">{snippet.title.clone().unwrap_or_default()}</span>
                <span class="snippet-actions">
                    {badge(kind)}
                    {(kind == SnippetKind::Editable).then(|| view! {
                        <button class="btn btn-secondary btn-sm" on:click=reset>"./reset"</button>
                    })}
                    <button class="btn btn-secondary btn-sm" on:click=copy aria-label="Copy code">"copy"</button>
                    {(kind != SnippetKind::Static).then(|| view! {
                        <button class="btn btn-sm" on:click=run disabled=running>
                            {move || if running() { "running…" } else { "./run" }}
                        </button>
                    })}
                </span>
            </figcaption>
            {if kind == SnippetKind::Editable {
                view! {
                    <textarea
                        class="snippet-code snippet-edit"
                        spellcheck="false"
                        aria-label=format!("Editable code: {}", snippet.title.clone().unwrap_or_default())
                        rows=snippet.visible_code.lines().count()
                        prop:value=move || edited.get()
                        on:input=move |ev| edited.set(event_target_value(&ev))
                    ></textarea>
                }
                .into_any()
            } else {
                view! { <Highlighted code=&snippet.visible_code error_lines=Signal::derive(error_lines) /> }.into_any()
            }}
            <Output snippet=snippet state=state />
        </figure>
    }
}

fn badge(kind: SnippetKind) -> Option<AnyView> {
    let (class, label) = match kind {
        SnippetKind::Static => return None,
        SnippetKind::Runnable => ("badge badge-runnable", "runnable"),
        SnippetKind::Editable => ("badge badge-editable", "editable"),
        SnippetKind::DoesNotCompile => ("badge badge-error", "won't compile"),
        SnippetKind::Panics => ("badge badge-panics", "panics"),
    };
    Some(view! { <span class=class>{label}</span> }.into_any())
}

#[component]
fn Highlighted(code: &'static str, error_lines: Signal<BTreeSet<u32>>) -> impl IntoView {
    view! {
        <pre class="snippet-code"><code>
            {code
                .lines()
                .enumerate()
                .map(|(i, text)| {
                    let n = i as u32 + 1;
                    let class = move || if error_lines.with(|e| e.contains(&n)) { "line line-error" } else { "line" };
                    view! {
                        <span class=class>
                            {highlight::line(text)
                                .into_iter()
                                .map(|(token, piece)| view! { <span class=token.class()>{piece}</span> })
                                .collect_view()}
                            "\n"
                        </span>
                    }
                })
                .collect_view()}
        </code></pre>
    }
}

#[component]
fn Output(snippet: &'static Snippet, state: RwSignal<RunState>) -> impl IntoView {
    move || match state.get() {
        RunState::Idle | RunState::Running => None,
        RunState::Failed(message) => Some(
            view! {
                <div class="snippet-output output-error" role="status">{message}</div>
            }
            .into_any(),
        ),
        RunState::Done(r) => {
            let errors: Vec<_> = r
                .diagnostics
                .iter()
                .filter(|d| d.severity == Severity::Error)
                .collect();
            Some(if r.success {
                let text = if r.stdout.is_empty() {
                    "(no output)".to_string()
                } else {
                    r.stdout.clone()
                };
                view! { <pre class="snippet-output output-ok" role="status">{text}</pre> }
                    .into_any()
            } else if !errors.is_empty() {
                view! {
                    <div class="snippet-output output-error" role="status">
                        {errors.into_iter().map(|d| {
                            let line = visible_line(snippet, d.line).map_or(String::new(), |n| format!(" (line {n})"));
                            let code = d.code.as_ref().map_or(String::new(), |c| format!("[{c}]"));
                            view! { <p><strong>{format!("error{code}")}</strong>": "{d.message.clone()}{line}</p> }
                        }).collect_view()}
                    </div>
                }
                .into_any()
            } else {
                view! {
                    <pre class="snippet-output output-error" role="status">{r.stdout.clone()}{program_stderr(&r.stderr)}</pre>
                }
                .into_any()
            })
        }
    }
}

/// An edited snippet with its hidden setup lines put back around it.
fn with_hidden_lines(snippet: &Snippet, edited: &str) -> String {
    let lines: Vec<&str> = snippet.code.lines().collect();
    let first = snippet
        .visible_lines
        .first()
        .map_or(lines.len(), |&n| n as usize - 1);
    let last = snippet
        .visible_lines
        .last()
        .map_or(lines.len(), |&n| n as usize);
    let mut code = String::new();
    for line in &lines[..first] {
        code.push_str(line);
        code.push('\n');
    }
    code.push_str(edited.trim_end());
    code.push('\n');
    for line in &lines[last..] {
        code.push_str(line);
        code.push('\n');
    }
    code
}

/// The visible line (1-based) for a line of the compiled code.
fn visible_line(snippet: &Snippet, line: u32) -> Option<u32> {
    snippet
        .visible_lines
        .iter()
        .position(|&n| n == line)
        .map(|i| i as u32 + 1)
}

fn visible_error_lines(snippet: &Snippet, response: &RunResponse) -> BTreeSet<u32> {
    response
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .filter_map(|d| visible_line(snippet, d.line))
        .collect()
}

/// What the program itself printed to stderr, without Cargo's build lines.
fn program_stderr(stderr: &str) -> String {
    match stderr.find("Running `") {
        Some(i) => stderr[i..]
            .split_once('\n')
            .map_or("", |(_, rest)| rest.trim_start())
            .to_string(),
        None => stderr.to_string(),
    }
}
