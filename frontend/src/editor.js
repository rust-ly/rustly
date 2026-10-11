// Thin binding to Monaco (loaded from a CDN in index.html), called from
// `editor.rs`. If Monaco can't load, a plain textarea stands in.

// Monaco's built-in theme for the page theme, on the editor's own background.
function theme(el) {
  const light = document.documentElement.dataset.theme === "light";
  const bg = getComputedStyle(el).backgroundColor.match(/\d+/g).slice(0, 3);
  const hex = "#" + bg.map((n) => (+n).toString(16).padStart(2, "0")).join("");
  // Comments in the page's muted colour: Monaco's own green is too faint on our backgrounds.
  const muted = getComputedStyle(el).getPropertyValue("--muted").trim().replace("#", "");
  const rules = [{ token: "comment", foreground: muted, fontStyle: "italic" }];
  monaco.editor.defineTheme("rustly", { base: light ? "vs" : "vs-dark", inherit: true, rules, colors: { "editor.background": hex } });
  return "rustly";
}

// Focuses the first focusable element after `el`, outside it.
function focus_after(el) {
  const focusable = "a[href], button:not([disabled]), input, select, textarea, [tabindex]:not([tabindex='-1'])";
  const next = [...document.querySelectorAll(focusable)]
    .find((n) => !el.contains(n) && el.compareDocumentPosition(n) & Node.DOCUMENT_POSITION_FOLLOWING);
  if (next) next.focus();
}

export function create_editor(el, code, on_change) {
  const handle = { code, ed: null, area: null };
  const fallback = () => {
    const area = document.createElement("textarea");
    area.className = "editor-fallback";
    area.spellcheck = false;
    area.setAttribute("aria-label", "Code editor");
    area.value = handle.code;
    area.oninput = () => { handle.code = area.value; on_change(area.value); };
    el.replaceChildren(area);
    handle.area = area;
  };
  if (!window.require) return fallback(), handle;
  window.require(["vs/editor/editor.main"], () => {
    if (handle.disposed) return;
    const ed = monaco.editor.create(el, {
      value: handle.code, language: "rust", theme: theme(el), automaticLayout: true,
      minimap: { enabled: false }, scrollBeyondLastLine: false, tabSize: 4,
      fontFamily: "'JetBrains Mono', monospace", fontSize: 14, fixedOverflowWidgets: true,
      ariaLabel: "Code editor. Press Escape to move to the buttons below.",
    });
    // Tab indents inside the editor, so Escape moves focus to the first control
    // after it (./reset). Not while a popup such as autocomplete is open.
    ed.addCommand(monaco.KeyCode.Escape, () => focus_after(el),
      "!suggestWidgetVisible && !findWidgetVisible && !parameterHintsVisible && !inSnippetMode");
    ed.onDidChangeModelContent(() => { handle.code = ed.getValue(); on_change(handle.code); });
    handle.observer = new MutationObserver(() => monaco.editor.setTheme(theme(el)));
    handle.observer.observe(document.documentElement, { attributeFilter: ["data-theme"] });
    handle.ed = ed;
    if (handle.markers) set_markers(handle, handle.markers);
  }, fallback);
  return handle;
}

export function get_value(handle) { return handle.code; }

export function set_value(handle, code) {
  handle.code = code;
  if (handle.ed) handle.ed.setValue(code);
  if (handle.area) handle.area.value = code;
}

export function set_markers(handle, markers_json) {
  handle.markers = markers_json;
  if (handle.ed) monaco.editor.setModelMarkers(handle.ed.getModel(), "rustc", JSON.parse(markers_json));
}

export function dispose_editor(handle) {
  handle.disposed = true;
  if (handle.observer) handle.observer.disconnect();
  if (handle.ed) handle.ed.dispose();
}
