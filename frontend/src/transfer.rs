//! Export / Import progress, so it can move between browsers.

use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, HtmlInputElement, Url};

use crate::learner::Learner;

const FILE_NAME: &str = "rustly-progress.json";

#[component]
pub fn ProgressTransfer() -> impl IntoView {
    let learner = Learner::get();
    let status = RwSignal::new(None::<Result<String, String>>);

    let export = move |_| {
        let json = learner
            .progress
            .with_untracked(serde_json::to_string_pretty)
            .unwrap_or_default();
        status.set(Some(
            download(&json)
                .map(|()| format!("Saved {FILE_NAME}."))
                .map_err(|_| "Your browser blocked the download.".to_string()),
        ));
    };

    let import = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = event_target(&ev);
        let Some(file) = input.files().and_then(|files| files.get(0)) else {
            return;
        };
        // Lets the same file be picked again later.
        input.set_value("");
        spawn_local(async move {
            let text = JsFuture::from(file.text())
                .await
                .ok()
                .and_then(|v| v.as_string());
            status.set(Some(match text {
                Some(json) => learner
                    .import(&json)
                    .map(|()| "Progress imported.".to_string()),
                None => Err("Couldn't read that file.".to_string()),
            }));
        });
    };

    view! {
        <section class="section transfer">
            <p class="eyebrow">"rustly::progress"</p>
            <h2>"Move your progress"</h2>
            <p class="muted">"Progress is saved in this browser. Export it to a file, then import that file in another browser to carry on there. Importing replaces this browser's progress."</p>
            <div class="transfer-actions">
                <button class="btn btn-secondary" on:click=export>"./export"</button>
                <label class="btn btn-secondary">
                    "./import"
                    <input class="visually-hidden" type="file" accept=".json,application/json" on:change=import />
                </label>
            </div>
            {move || status.get().map(|result| match result {
                Ok(message) => view! { <p class="small" role="status">{message}</p> }.into_any(),
                Err(message) => view! { <p class="small error-text" role="alert">{message}</p> }.into_any(),
            })}
        </section>
    }
}

/// Saves `json` as a file through a temporary download link.
fn download(json: &str) -> Result<(), JsValue> {
    let parts = js_sys::Array::of1(&JsValue::from_str(json));
    let options = BlobPropertyBag::new();
    options.set_type("application/json");
    let blob = Blob::new_with_str_sequence_and_options(&parts, &options)?;
    let url = Url::create_object_url_with_blob(&blob)?;
    let link: HtmlAnchorElement = document().create_element("a")?.unchecked_into();
    link.set_href(&url);
    link.set_download(FILE_NAME);
    link.click();
    Url::revoke_object_url(&url)
}
