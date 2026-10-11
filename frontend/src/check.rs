//! Live checking: about a second after the learner stops typing, the code
//! goes to `/api/check` and the diagnostics come back as squiggles and a
//! Problems list.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::prelude::*;
use leptos::task::spawn_local;
use shared::{Diagnostic, Severity};

use crate::api;

const DEBOUNCE: Duration = Duration::from_millis(1000);

#[derive(Clone, PartialEq)]
pub enum CheckStatus {
    Idle,
    Checking,
    Done,
    Failed(String),
}

/// The latest diagnostics for one editor, from live checks and runs.
#[derive(Clone, Copy)]
pub struct LiveCheck {
    pub status: RwSignal<CheckStatus>,
    pub diagnostics: RwSignal<Vec<Diagnostic>>,
    /// Bumped by every check and run, so a slower, older answer is dropped.
    latest: StoredValue<u64>,
}

impl LiveCheck {
    /// Checks `code` after each pause in typing. The code as first loaded
    /// isn't checked, so opening a lesson doesn't call the compile service.
    pub fn new(code: RwSignal<String>) -> Self {
        let check = Self {
            status: RwSignal::new(CheckStatus::Idle),
            diagnostics: RwSignal::new(Vec::new()),
            latest: StoredValue::new(0),
        };
        let timer = StoredValue::new(None::<TimeoutHandle>);
        let last_checked = StoredValue::new(code.with_untracked(|c| hash(c)));
        Effect::new(move |first: Option<()>| {
            code.track();
            if let Some(t) = timer.get_value() {
                t.clear();
            }
            if first.is_none() {
                return;
            }
            let handle = set_timeout_with_handle(
                move || {
                    let text = code.get_untracked();
                    let h = hash(&text);
                    if last_checked.get_value() == h {
                        return;
                    }
                    last_checked.set_value(h);
                    let id = check.next();
                    check.status.set(CheckStatus::Checking);
                    spawn_local(async move {
                        let result = api::check(text).await;
                        if check.is_latest(id) {
                            match result {
                                Ok(r) => check.show(r.diagnostics),
                                Err(message) => check.status.set(CheckStatus::Failed(message)),
                            }
                        }
                    });
                },
                DEBOUNCE,
            );
            timer.set_value(handle.ok());
        });
        on_cleanup(move || {
            if let Some(t) = timer.try_get_value().flatten() {
                t.clear();
            }
        });
        check
    }

    /// Starts a new request and returns its id.
    pub fn next(&self) -> u64 {
        self.latest.update_value(|n| *n += 1);
        self.latest.get_value()
    }

    pub fn is_latest(&self, id: u64) -> bool {
        self.latest.try_get_value() == Some(id)
    }

    pub fn show(&self, diagnostics: Vec<Diagnostic>) {
        self.diagnostics.set(diagnostics);
        self.status.set(CheckStatus::Done);
    }
}

fn hash(code: &str) -> u64 {
    let mut h = DefaultHasher::new();
    code.hash(&mut h);
    h.finish()
}

fn count(diagnostics: &[Diagnostic], severity: Severity) -> usize {
    diagnostics
        .iter()
        .filter(|d| d.severity == severity)
        .count()
}

fn plural(n: usize, word: &str) -> String {
    format!("{n} {word}{}", if n == 1 { "" } else { "s" })
}

/// "Checking…", "No errors", or "2 errors, 1 warning".
#[component]
pub fn CheckChip(check: LiveCheck) -> impl IntoView {
    let text = move || match check.status.get() {
        CheckStatus::Idle => None,
        CheckStatus::Checking => Some(("checking", "Checking…".to_string())),
        CheckStatus::Failed(_) => Some(("failed", "Couldn't check".to_string())),
        CheckStatus::Done => check.diagnostics.with(|d| {
            let errors = count(d, Severity::Error);
            let warnings = count(d, Severity::Warning);
            Some(match (errors, warnings) {
                (0, 0) => ("ok", "No errors".to_string()),
                (0, w) => ("warn", plural(w, "warning")),
                (e, 0) => ("error", plural(e, "error")),
                (e, w) => (
                    "error",
                    format!("{}, {}", plural(e, "error"), plural(w, "warning")),
                ),
            })
        }),
    };
    move || {
        text().map(|(kind, label)| {
            view! { <span class=format!("check-chip {kind}") role="status" title=move || match check.status.get() { CheckStatus::Failed(m) => m, _ => String::new() }>{label}</span> }
        })
    }
}

/// The current errors and warnings; clicking one moves the cursor there.
#[component]
pub fn Problems(check: LiveCheck, goto: RwSignal<Option<(u32, u32)>>) -> impl IntoView {
    move || {
        check.diagnostics.with(|d| {
            let shown: Vec<_> = d
                .iter()
                .filter(|d| d.severity != Severity::Note)
                .cloned()
                .collect();
            (!shown.is_empty()).then(|| {
                view! {
                    <ul class="problems" aria-label="Problems">
                        {shown.into_iter().map(|d| {
                            let (line, col) = (d.line, d.col);
                            let error = d.severity == Severity::Error;
                            let code = d.code.map_or(String::new(), |c| format!("[{c}]"));
                            view! {
                                <li>
                                    <button class="problem" class:error=error on:click=move |_| goto.set(Some((line, col)))>
                                        <span class="problem-kind">{format!("{}{code}", if error { "error" } else { "warning" })}</span>
                                        <span class="problem-message">{d.message}</span>
                                        <span class="problem-line">{format!("line {line}")}</span>
                                    </button>
                                </li>
                            }
                        }).collect_view()}
                    </ul>
                }
            })
        })
    }
}
