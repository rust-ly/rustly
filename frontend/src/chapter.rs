//! `/learn/:track/:chapter`: the chapter intro.

use leptos::prelude::*;
use leptos_router::components::{A, Redirect};
use leptos_router::hooks::use_params_map;

use crate::course_map::{ChapterState, ConceptList, StateChip, chapter_state};
use crate::icons::LockIcon;
use crate::learner::Learner;
use crate::{NotFound, routes};

#[component]
pub fn ChapterPage() -> impl IntoView {
    let params = use_params_map();
    let learner = Learner::get();
    move || {
        let (track, slug) = params.with(|p| (p.get("track"), p.get("chapter")));
        let Some(chapter) =
            routes::find_chapter(&track.unwrap_or_default(), &slug.unwrap_or_default())
        else {
            return view! { <NotFound /> }.into_any();
        };
        let state = chapter_state(&learner, chapter);
        if state == ChapterState::Locked {
            let first = &chapter.concepts[0].id;
            return view! { <Redirect path=routes::locked_redirect(first) /> }.into_any();
        }
        let challenge_open = learner
            .progress
            .with(|p| content::graph().is_challenge_open(&chapter.id, p));
        view! {
            <section class="section page-head">
                <div class="card-top">
                    <p class="eyebrow">{format!("ch.{:02} // {}", chapter.order, routes::short_id(&chapter.id))}</p>
                    <StateChip state=state />
                </div>
                <h1>{chapter.title.clone()}</h1>
                <p class="lede">{chapter.summary.clone()}</p>
                <a href=chapter.source_url.clone() target="_blank" rel="noopener">"Read the full chapter ↗"</a>
            </section>
            <section class="section">
                <p class="eyebrow">"concepts //"</p>
                <ConceptList chapter=chapter />
            </section>
            <section class="section">
                <p class="eyebrow">"challenge //"</p>
                {if challenge_open {
                    view! {
                        <A href=routes::challenge_path(chapter) attr:class="btn">"./start-challenge"</A>
                    }
                    .into_any()
                } else {
                    view! {
                        <p class="lock-line"><LockIcon />" pass every concept above to unlock the challenge"</p>
                    }
                    .into_any()
                }}
            </section>
        }
        .into_any()
    }
}
