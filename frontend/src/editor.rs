//! The exercise code editor: Monaco, driven through the thin binding in
//! `editor.js`.

use leptos::html::Div;
use leptos::prelude::*;
use serde_json::json;
use shared::{Diagnostic, Severity};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/src/editor.js")]
extern "C" {
    fn create_editor(el: &HtmlElement, code: &str, on_change: &Closure<dyn Fn(String)>) -> JsValue;
    fn get_value(editor: &JsValue) -> String;
    fn set_value(editor: &JsValue, code: &str);
    fn set_markers(editor: &JsValue, markers_json: &str);
    fn dispose_editor(editor: &JsValue);
}

struct Editor {
    handle: JsValue,
    // Monaco calls this on every edit, so it lives as long as the editor.
    _on_change: Closure<dyn Fn(String)>,
}

/// Edits `code` in both directions: typing updates the signal, and setting the
/// signal (e.g. Reset) updates the editor. `diagnostics` become squiggles.
#[component]
pub fn CodeEditor(
    code: RwSignal<String>,
    #[prop(into)] diagnostics: Signal<Vec<Diagnostic>>,
) -> impl IntoView {
    let host = NodeRef::<Div>::new();
    let editor = StoredValue::new_local(None::<Editor>);

    Effect::new(move || {
        let Some(el) = host.get() else { return };
        if editor.with_value(Option::is_some) {
            return;
        }
        let on_change = Closure::<dyn Fn(String)>::new(move |text: String| code.set(text));
        let handle = create_editor(&el, &code.get_untracked(), &on_change);
        editor.set_value(Some(Editor {
            handle,
            _on_change: on_change,
        }));
    });
    Effect::new(move || {
        let text = code.get();
        editor.with_value(|e| {
            if let Some(e) = e
                && get_value(&e.handle) != text
            {
                set_value(&e.handle, &text);
            }
        });
    });
    Effect::new(move || {
        let markers = diagnostics.with(|d| markers_json(d));
        editor.with_value(|e| {
            if let Some(e) = e {
                set_markers(&e.handle, &markers);
            }
        });
    });
    on_cleanup(move || {
        editor.try_with_value(|e| {
            if let Some(e) = e {
                dispose_editor(&e.handle);
            }
        });
    });

    view! { <div class="code-editor" node_ref=host></div> }
}

/// Monaco's `IMarkerData`, as JSON.
fn markers_json(diagnostics: &[Diagnostic]) -> String {
    let markers: Vec<_> = diagnostics
        .iter()
        .map(|d| {
            json!({
                // monaco.MarkerSeverity
                "severity": match d.severity {
                    Severity::Error => 8,
                    Severity::Warning => 4,
                    Severity::Note => 2,
                },
                "message": d.message,
                "code": d.code,
                "startLineNumber": d.line,
                "startColumn": d.col,
                "endLineNumber": d.end_line,
                "endColumn": d.end_col,
            })
        })
        .collect();
    serde_json::Value::Array(markers).to_string()
}
