//! `/snippets`: every snippet from the concepts the learner has opened,
//! grouped by chapter, searchable, with stars.

use std::collections::BTreeSet;

use content::{Chapter, Snippet, Track};
use leptos::prelude::*;
use leptos_router::components::A;

use crate::learner::Learner;
use crate::routes;
use crate::snippet::SnippetView;

#[component]
pub fn SnippetsPage() -> impl IntoView {
    let learner = Learner::get();
    let query = RwSignal::new(String::new());
    let starred_only = RwSignal::new(false);
    // Only the ids, so starring a snippet doesn't re-render the list (and
    // clear its output) unless it changes what's shown.
    let shown = Memo::new(move |_| {
        let terms = search_terms(&query.get());
        let starred_only = starred_only.get();
        learner.progress.with(|p| {
            content::snippets()
                .filter(|s| learner.is_open(&s.concept_id))
                .filter(|s| !starred_only || p.starred_snippets.contains(&s.id))
                .filter(|s| matches(s, &terms))
                .map(|s| s.id.clone())
                .collect::<BTreeSet<_>>()
        })
    });

    view! {
        <section class="section page-head">
            <p class="eyebrow">"rustly::snippets"</p>
            <h1>"Snippet library"</h1>
            <p class="lede">"Every code example from the lessons you've opened. Star the ones you want to find again."</p>
            <div class="library-filters">
                <input
                    type="search"
                    class="search"
                    placeholder="Search titles and code"
                    aria-label="Search snippets"
                    prop:value=move || query.get()
                    on:input=move |ev| query.set(event_target_value(&ev))
                />
                <label class="check">
                    <input
                        type="checkbox"
                        prop:checked=move || starred_only.get()
                        on:change=move |ev| starred_only.set(event_target_checked(&ev))
                    />
                    "Starred only"
                </label>
            </div>
        </section>
        {move || shown.with(|shown| {
            if shown.is_empty() {
                let message = if starred_only.get_untracked() && query.with_untracked(String::is_empty) {
                    "No starred snippets yet. Press ☆ on any snippet to keep it here."
                } else {
                    "No snippets match that search."
                };
                return view! { <p class="muted section">{message}</p> }.into_any();
            }
            [Track::Book, Track::Tokio]
                .into_iter()
                .flat_map(content::chapters)
                .filter(|chapter| chapter_snippets(chapter).any(|s| shown.contains(&s.id)))
                .map(|chapter| view! { <ChapterSnippets chapter=chapter shown=shown.clone() /> })
                .collect_view()
                .into_any()
        })}
    }
}

#[component]
fn ChapterSnippets(chapter: &'static Chapter, shown: BTreeSet<String>) -> impl IntoView {
    view! {
        <section class="section library-chapter">
            <p class="eyebrow">{format!("{} // ch.{:02}", routes::track_name(chapter.track), chapter.order)}</p>
            <h2>{chapter.title.clone()}</h2>
            {chapter
                .concepts
                .iter()
                .filter(|concept| concept.snippets.iter().any(|s| shown.contains(&s.id)))
                .map(|concept| view! {
                    <h3 class="library-concept">
                        <A href=routes::concept_path(concept)>{concept.title.clone()}</A>
                    </h3>
                    {concept
                        .snippets
                        .iter()
                        .filter(|s| shown.contains(&s.id))
                        .map(|s| view! { <SnippetView snippet=s /> })
                        .collect_view()}
                })
                .collect_view()}
        </section>
    }
}

fn chapter_snippets(chapter: &'static Chapter) -> impl Iterator<Item = &'static Snippet> {
    chapter.concepts.iter().flat_map(|c| &c.snippets)
}

fn search_terms(query: &str) -> Vec<String> {
    query.split_whitespace().map(str::to_lowercase).collect()
}

/// Every search term appears in the snippet's title, its concept or
/// chapter title, or its code.
fn matches(snippet: &Snippet, terms: &[String]) -> bool {
    if terms.is_empty() {
        return true;
    }
    let concept = content::course().concept(&snippet.concept_id);
    let mut haystack = snippet.title.clone().unwrap_or_default();
    if let Some(concept) = concept {
        haystack.push('\n');
        haystack.push_str(&concept.title);
        if let Some(chapter) = content::course()
            .all_chapters()
            .find(|c| c.id == concept.chapter_id)
        {
            haystack.push('\n');
            haystack.push_str(&chapter.title);
        }
    }
    haystack.push('\n');
    haystack.push_str(&snippet.visible_code);
    let haystack = haystack.to_lowercase();
    terms.iter().all(|t| haystack.contains(t.as_str()))
}
