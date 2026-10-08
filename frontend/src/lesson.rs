//! `/learn/:track/:chapter/:concept`: a concept lesson, and the chapter
//! challenge at `/learn/:track/:chapter/challenge`.

use leptos::prelude::*;
use leptos_router::components::Redirect;
use leptos_router::hooks::use_params_map;

use crate::learner::Learner;
use crate::{NotFound, routes};

#[component]
pub fn ConceptPage() -> impl IntoView {
    let params = use_params_map();
    let learner = Learner::get();
    move || {
        let (track, chapter, concept) =
            params.with(|p| (p.get("track"), p.get("chapter"), p.get("concept")));
        let Some(chapter) =
            routes::find_chapter(&track.unwrap_or_default(), &chapter.unwrap_or_default())
        else {
            return view! { <NotFound /> }.into_any();
        };
        let Some(concept) = routes::find_concept(chapter, &concept.unwrap_or_default()) else {
            return view! { <NotFound /> }.into_any();
        };
        if !learner.is_open(&concept.id) {
            return view! { <Redirect path=routes::locked_redirect(&concept.id) /> }.into_any();
        }
        view! {
            <section class="section page-head">
                <p class="eyebrow">{routes::short_id(&concept.id).to_string()}</p>
                <h1>{concept.title.clone()}</h1>
                <p class="lede">{concept.summary.clone()}</p>
            </section>
        }
        .into_any()
    }
}
