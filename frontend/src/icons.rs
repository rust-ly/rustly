//! Small inline SVG icons. They inherit `currentColor` and are hidden from
//! screen readers; every use sits next to a text label.

use leptos::prelude::*;

#[component]
pub fn LockIcon() -> impl IntoView {
    view! {
        <svg class="icon" viewBox="0 0 16 16" width="14" height="14" aria-hidden="true">
            <rect x="3" y="7" width="10" height="7" fill="none" stroke="currentColor" stroke-width="1.5" />
            <path d="M5 7V5a3 3 0 0 1 6 0v2" fill="none" stroke="currentColor" stroke-width="1.5" />
        </svg>
    }
}

#[component]
pub fn CheckIcon() -> impl IntoView {
    view! {
        <svg class="icon" viewBox="0 0 16 16" width="14" height="14" aria-hidden="true">
            <path d="M3 8.5l3 3 7-7" fill="none" stroke="currentColor" stroke-width="2" />
        </svg>
    }
}
