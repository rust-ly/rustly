# Rustly: Capstone Handoff for Claude Code

Oct 8, 2026 · @Coder Girlies

## Overview

Rustly is a browser-based Rust course: each chapter is a lesson page next to a code editor, and learners get compiler feedback as they type and graded exercises when they press Run. Everything we write is Rust — the frontend compiles to WebAssembly with Leptos, the API runs as Rust functions on Vercel, and lessons, types and grading logic live in shared Rust crates.

**The one constraint that shapes the design:** Vercel functions cannot run `rustc`. There is no Rust toolchain in the function environment and the size limits rule out bundling one. So the API forwards code to a compile service — by default the public [Rust Playground](https://play.rust-lang.org), with a config switch to point at a self-hosted copy later. Our Rust code owns everything around that call: building the program, appending hidden tests, parsing compiler errors into editor markers, and grading results.

**Capstone scope (MVP)**

- 15 chapters in two tracks, 10 from the Rust Book and 5 from the Tokio tutorial, each with 2–3 exercises (see Curriculum)
- Lesson page: rendered markdown with runnable, editable and "won't compile" code snippets on the left, editor on the right
- Live diagnostics: red/yellow squiggles from the real compiler, about 1 second after the learner stops typing
- Run button: executes code and shows stdout/stderr
- Submit button: runs hidden tests and shows pass/fail per test; passing a checkpoint unlocks the next concept, and passing a chapter challenge unlocks the next chapter
- Progress and starred snippets saved in the browser, plus a searchable snippet library
- Deployed on Vercel from GitHub, with CI

**Out of scope for the MVP:** user accounts, a database, an in-browser compiler, and authoring tools. They are listed under stretch goals.

**How to use this doc with Claude Code:** export it as Markdown, save it in the repo root as `HANDOFF.md`, and paste the kickoff prompt at the end into Claude Code. Work one milestone per session and have Claude Code tick off acceptance criteria before moving on.

## Architecture

The browser runs a Leptos app compiled to WASM; Vercel serves those static files plus four Rust API functions; the API is the only part that talks to the compile service. Shared crates keep types, lessons and grading logic in one place.

&#91;embedded content: RustLearn architecture · browser, Vercel, compile service\]

The frontend never calls the Playground directly: going through our API keeps hidden tests secret, lets us parse errors in Rust, and lets us swap the compile service with one env var.

**Request flow for live checking:** learner types → 1 s debounce → `POST /api/check` with the code → function calls the Playground's Clippy endpoint → `runner` parses stderr into `Vec<Diagnostic>` → JSON back → Monaco shows squiggles. Submit is the same path, except the API appends the exercise's hidden tests and runs them.

## Repo layout and Cargo workspace

One Git repo, one Cargo workspace. The root `Cargo.toml` is both the workspace and the package Vercel builds API functions from, because Vercel's Rust runtime expects each function as a `[[bin]]` whose source is in `api/`.

```text
rustly/
├── Cargo.toml            # [workspace] + [package] with one [[bin]] per API route
├── vercel.json
├── rust-toolchain.toml   # pin stable, add wasm32-unknown-unknown target
├── THIRD_PARTY_LICENSES.md  # Rust Book (MIT/Apache-2.0) + Tokio website (MIT)
├── api/                  # Vercel functions (thin: parse request → call crates → respond)
│   ├── run.rs            # POST /api/run
│   ├── check.rs          # POST /api/check
│   ├── submit.rs         # POST /api/submit
│   └── health.rs         # GET  /api/health
├── crates/
│   ├── shared/           # serde types used by BOTH frontend and api
│   ├── content/          # lessons + exercises, embedded at compile time
│   └── runner/           # playground client, code assembly, output parsers
├── frontend/             # Leptos CSR app, built with Trunk
│   ├── Cargo.toml
│   ├── Trunk.toml
│   ├── index.html
│   ├── style/main.css
│   └── src/{main.rs, app.rs, pages/, components/, api.rs, storage.rs}
├── content/              # the course itself (markdown + TOML)
│   ├── book/             # Track 1: follows The Rust Programming Language
│   │   └── 04-ownership/
│   │       ├── chapter.toml          # title, order, source link, requires
│   │       ├── test-out.toml         # optional: skip the chapter
│   │       ├── 01-moves/
│   │       │   ├── lesson.md         # prose + snippets (code fences)
│   │       │   ├── checkpoint.toml   # passing it unlocks 02-borrowing
│   │       │   └── practice-*.toml   # optional extra exercises
│   │       ├── 02-borrowing/ ...
│   │       ├── 03-slices/ ...
│   │       └── challenge.toml        # passing it unlocks chapter 5
│   └── tokio/            # Track 2: follows the Tokio tutorial (same layout)
│       └── 02-spawning/ ...
└── .github/workflows/ci.yml
```

**Workspace rules for Claude Code**

- `members = ["crates/*", "frontend"]`, but set `default-members` to exclude `frontend` so `cargo build` at the root (what Vercel runs for the API) never compiles the WASM app.
- `shared` must compile for both `wasm32-unknown-unknown` and Linux: only `serde` and plain types, no `tokio` or `reqwest`.
- `content` is used by both sides: the frontend shows lesson text and starter code; the API uses it to look up hidden tests by exercise id.
- `runner` is server-only (uses `reqwest`) and is where most unit tests live.
- Keep `api/*.rs` files under \~60 lines each — real logic belongs in `runner` so it can be tested with plain `cargo test`.

**Suggested crates** (Claude Code should confirm current versions on crates.io):

| Purpose | Crate |
| --- | --- |
| Frontend framework | `leptos` (CSR feature), `leptos_router` |
| WASM build tool | `trunk` (CLI) |
| HTTP from browser | `gloo-net` |
| Browser storage | `gloo-storage` |
| JS interop for editor | `wasm-bindgen`, `web-sys`, `js-sys` |
| Markdown → HTML | `pulldown-cmark` (+ `ammonia` if any HTML is user-supplied) |
| Vercel functions | `vercel_runtime`, `tokio` |
| HTTP from server | `reqwest` (rustls, json) |
| Serialization | `serde`, `serde_json`, `toml` |
| Embedding content | `include_dir` (or a `build.rs`) |
| Hashing for cache keys | `sha2` or `blake3` |

## Content model

The course is plain files in `content/`, embedded into the binaries at compile time, so there is no database and no CMS. Adding a chapter means adding a folder and redeploying.

**Lesson file** — `content/book/04-ownership/01-moves/lesson.md`, markdown with TOML front matter:

````markdown
+++
id = "book.ownership.moves"
chapter = "book.ownership"
requires = []   # empty = the previous concept (or previous chapter's challenge)
title = "Moves"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html"
summary = "Who owns a value, and what happens when it moves."
+++

# Ownership

Every value in Rust has a single owner...

```rust,runnable
fn main() {
    let s = String::from("hi");
    let t = s;
    println!("{t}");
}
```
````

Code fences tagged `rust,runnable` get a small Run button inline; plain `rust` fences are display-only.

**Exercise file** — `content/book/04-ownership/01-moves/checkpoint.toml`:

```toml
id = "book.ownership.moves.checkpoint"
kind = "checkpoint"   # practice | checkpoint | challenge | test-out
title = "Fix the move"
prompt = """
This code doesn't compile. Make `greet` borrow the string instead of taking ownership.
"""
starter = """
fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

fn main() {
    let name = String::from("Ada");
    let msg = greet(name);
    println!("{msg} ({name})");
}
"""
# Appended to the learner's code server-side. Never sent to the browser.
hidden_tests = """
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greets_by_reference() {
        let n = String::from("Ada");
        assert_eq!(greet(&n), "Hello, Ada!");
        assert_eq!(n, "Ada");
    }
}
"""
hints = [
  "Look at the type of the `name` parameter.",
  "`&str` lets you borrow without taking ownership.",
]
solution = """..."""
```

**Rust types in `crates/content`**

```rust
pub enum Track { Book, Tokio }
pub enum ExerciseKind { Practice, Checkpoint, Challenge, TestOut }
pub enum SnippetKind { Static, Runnable, Editable, DoesNotCompile, Panics }

pub struct Chapter { pub id: String, pub track: Track, pub title: String, pub order: u32,
                     pub source_url: String, pub requires: Vec<String>,
                     pub concepts: Vec<Concept>, pub challenge: Exercise,
                     pub test_out: Option<Exercise> }

pub struct Concept { pub id: String, pub title: String, pub body_md: String,
                     pub requires: Vec<String>,        // resolved: never empty after loading
                     pub snippets: Vec<Snippet>,
                     pub checkpoint: Exercise, pub practice: Vec<Exercise> }

pub struct Snippet { pub id: String, pub concept_id: String, pub kind: SnippetKind,
                     pub title: Option<String>,
                     pub code: String,          // full code sent to the compiler
                     pub visible_code: String } // `# ` lines removed

pub struct Exercise { pub id: String, pub kind: ExerciseKind, pub title: String,
                      pub prompt: String, pub starter: String, pub hints: Vec<String>,
                      #[cfg(feature = "server")] pub hidden_tests: String,
                      #[cfg(feature = "server")] pub solution: String }

pub struct CourseGraph { /* concept/chapter ids → requires, built at load time */ }

pub fn chapters(track: Track) -> &'static [Chapter];   // sorted by order
pub fn exercise(id: &str) -> Option<&'static Exercise>;
pub fn snippets() -> impl Iterator<Item = &'static Snippet>;
pub fn graph() -> &'static CourseGraph;
pub fn unlocked(graph: &CourseGraph, progress: &Progress) -> BTreeSet<String>;
```

- Parse once into a `OnceLock` / `LazyLock` static.
- Gate `hidden_tests` and `solution` behind a Cargo feature (`server`) so the frontend build physically cannot include them. The API enables `server`; the frontend doesn't.
- A unit test in `content` must load every file, fail on bad TOML, duplicate ids or missing fields, and compile-check that every `solution` + `hidden_tests` passes (via the runner, marked `#[ignore]` so it only runs in CI on demand).

**How grading works:** the API takes the learner's code, appends `hidden_tests`, calls the compile service with tests enabled, and parses lines like `test tests::greets_by_reference ... ok` and the final `test result:` line into a per-test pass/fail list.

## Curriculum: the Rust Book + the Tokio tutorial

Rustly has two tracks that follow existing, openly licensed material: **Track 1** follows [The Rust Programming Language](https://doc.rust-lang.org/book/) ("the Book"), and **Track 2** follows the [Tokio tutorial](https://tokio.rs/tokio/tutorial). Each Rustly chapter is a short lesson in our own words, runnable snippets, 2–3 exercises, and a "Read the full chapter" link to the original page. The MVP covers 10 Book chapters and 5 Tokio sections; the rest come after.

**Licensing and attribution**

- The Book's [source repo](https://github.com/rust-lang/book) is dual-licensed MIT and Apache-2.0. The Tokio [website repo](https://github.com/tokio-rs/website), which holds the tutorial, is MIT.
- Both licenses allow adapting text and code examples, as long as we keep their copyright and license notices. Add a `/credits` page and a `THIRD_PARTY_LICENSES.md` file listing both projects with their license texts, and show "Adapted from …" with a link on every lesson.
- Even though copying is allowed, write concise lessons (about 300–600 words) and link out for depth. Copying whole chapters makes lessons long and goes stale when upstream updates.
- The current Book targets Rust 1.97 and edition 2024, which matches our Playground settings (`edition: 2024`, stable channel).

**Playground limits that shape the exercises**

- Everything is one `main.rs` file, so multi-file module layouts and Cargo workflows are explained, not exercised.
- No stdin, no command-line args, no file system, and no network. Interactive or I/O programs become pure functions graded by hidden tests.
- The Playground ships a fixed set of popular crates. Claude Code must confirm that `tokio` (with `macros`, `rt-multi-thread`, `sync`, `time`), `tokio-stream`, `tokio-util` and `bytes` are available before writing Track 2 exercises. The Book's own `trpl` helper crate for Chapter 17 is unlikely to be there, so use Tokio for async instead.
- Tokio exercises use in-memory stand-ins: channels and `tokio::io::duplex()` instead of TCP sockets, and `#[tokio::test(start_paused = true)]` so timers don't burn the run time limit.

**Track 1: The Rust Book**

| Book chapter | Rustly exercises | Phase |
| --- | --- | --- |
| [1. Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html) | Print a greeting; read-only notes on installing Rust locally | MVP |
| [2. Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) | `compare_guess(guess, secret) -> Ordering` with tests, no stdin | Later |
| [3. Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html) | Fix a mutability error; Fahrenheit/Celsius; nth Fibonacci | MVP |
| [4. Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) | Fix a move; borrow instead of clone; `first_word` with slices | MVP |
| [5. Using Structs](https://doc.rust-lang.org/book/ch05-00-structs.html) | `Rectangle` with `area` and `can_hold` methods | MVP |
| [6. Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html) | Coin values with `match`; `Option` plus `if let` / `let...else` | MVP |
| [7. Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) | Inline `mod` blocks and `pub` visibility fixes | Later |
| [8. Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html) | Median and mode of a `Vec`; word counts with `HashMap`; Pig Latin | MVP |
| [9. Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) | Parse input into `Result` and propagate with `?` | MVP |
| [10. Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html) | Generic `largest`; a `Summary` trait; `longest<'a>` | MVP |
| [11. Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html) | Learner writes the tests; hidden tests check a planted bug is caught | Later |
| [12. An I/O Project (minigrep)](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) | `search` and `search_case_insensitive` over a string, no files | Later |
| [13. Iterators and Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html) | Shoe-size filter; rewrite loops as `map`/`filter`/`sum` | MVP |
| [14. More about Cargo and Crates.io](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html) | Read-only lesson | Later |
| [15. Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) | `Box` cons list; `Rc<RefCell<T>>` mock messenger | Later |
| [16. Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) | Threads + `mpsc`; counter with `Arc<Mutex<T>>` | MVP |
| [17. Async, Await, Futures, Streams](https://doc.rust-lang.org/book/ch17-00-async-await.html) | Bridge lesson that hands off to Track 2 (Tokio) | Later |
| [18. Object-Oriented Features](https://doc.rust-lang.org/book/ch18-00-oop.html) | Trait objects; state pattern for a blog `Post` | Later |
| [19. Patterns and Matching](https://doc.rust-lang.org/book/ch19-00-patterns.html) | Destructuring, guards, `@` bindings | Later |
| [20. Advanced Features](https://doc.rust-lang.org/book/ch20-00-advanced-features.html) | Newtype pattern; a small `macro_rules!` | Later |
| [21. Final Project: Web Server](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html) | `ThreadPool` that runs jobs, tested without sockets | Later |

**Track 2: Tokio**

| Tokio section | Rustly exercises | Phase |
| --- | --- | --- |
| [Hello Tokio](https://tokio.rs/tokio/tutorial/hello-tokio) | `#[tokio::main]`, writing and awaiting an `async fn` | MVP |
| [Spawning](https://tokio.rs/tokio/tutorial/spawning) | `tokio::spawn` + `JoinHandle`; fix `'static` and `Send` errors (great for live diagnostics) | MVP |
| [Shared state](https://tokio.rs/tokio/tutorial/shared-state) | In-memory key-value store with `Arc<Mutex<HashMap>>` | MVP |
| [Channels](https://tokio.rs/tokio/tutorial/channels) | Manager task pattern with `mpsc` + `oneshot` replies | MVP |
| [I/O](https://tokio.rs/tokio/tutorial/io) | Echo with `AsyncReadExt`/`AsyncWriteExt` over `tokio::io::duplex` | Later |
| [Framing](https://tokio.rs/tokio/tutorial/framing) | Parse length-prefixed frames from a `BytesMut` buffer | Later |
| [Async in depth](https://tokio.rs/tokio/tutorial/async) | Implement a `Delay` future by hand | Later |
| [Select](https://tokio.rs/tokio/tutorial/select) | `tokio::select!` with a timeout and cancellation | MVP |
| [Streams](https://tokio.rs/tokio/tutorial/streams) | `StreamExt` adapters over a stream of numbers | Later |
| [Graceful Shutdown](https://tokio.rs/tokio/topics/shutdown) | Stop workers with a `CancellationToken` | Later |
| [Unit Testing](https://tokio.rs/tokio/topics/testing) | `#[tokio::test]` with paused time | Later |

The Tokio tutorial builds a Redis-style server over TCP. Rustly keeps the same ideas (a key-value store, a manager task, framing) but runs them in memory, because the Playground has no network.

**Content workflow with Claude Code:** for each chapter, have Claude Code read the original page, draft `lesson.md` and the exercise TOML files, then prove every `solution` passes its `hidden_tests` and every `starter` fails them. You review the wording and difficulty before committing.

## Progression: exercises unlock the next concept

Learners can't skip ahead by scrolling: each concept stays locked until the exercise before it is passed. A chapter is split into 2–4 **concepts** (e.g. Ownership → Moves, Borrowing, Slices). Each concept is a short lesson, its code snippets, and one **checkpoint** exercise. Passing the checkpoint unlocks the next concept; passing the chapter's **challenge** unlocks the next chapter.

&#91;embedded content: unlock flow · Book chapter 4 as the example\]

**Exercise kinds**

| Kind | Where | What passing it does |
| --- | --- | --- |
| `practice` | Inside a concept, optional | Marks it done; unlocks nothing |
| `checkpoint` | End of each concept, exactly one | Unlocks every concept whose `requires` are now all met |
| `challenge` | End of each chapter, one bigger problem | Completes the chapter; unlocks the next chapter's first concept |
| `test-out` | Top of each chapter, optional | Lets someone who already knows the material complete the whole chapter at once |

**Rules**

- Every concept declares `requires = ["<concept or chapter id>", ...]`. If it's left out, it requires the previous concept in its chapter (or the previous chapter's challenge for the first concept).
- Cross-track prerequisites are allowed. Track 2 (Tokio) starts locked until Book chapters 10 (traits and generics), 13 (closures) and 16 (concurrency) are complete.
- Book chapter 1, concept 1 and its snippets are always open, so a new visitor can try the editor immediately.
- Locked concepts show their title, a lock icon and "Pass *X* to unlock". The lesson body and exercises stay hidden until unlocked.
- When a checkpoint passes, show a short "Unlocked: *Borrowing*" toast and move the learner straight to it.

**Where the logic lives:** a pure function in `crates/content`, `fn unlocked(graph: &CourseGraph, progress: &Progress) -> BTreeSet<ConceptId>`, used by the frontend to draw lock states. It's unit-tested with fixtures (fresh learner, mid-chapter, test-out, cross-track). Unlocking is enforced in the browser only, which is fine for a learning tool: it guides the order, it isn't a security boundary.

**Content validation (a `cargo test` in `content`):** every `requires` id exists, the graph has no cycles, every concept is reachable from Book chapter 1, and every concept has exactly one checkpoint.

## Code snippets

Every concept lesson includes 2–4 code snippets that show the idea before the learner has to use it. Snippets are code fences in `lesson.md`; their info string sets how they behave. The attribute names match the ones the Rust Book itself uses, so adapted examples keep their meaning.

| Fence | Learner sees | Learner can |
| --- | --- | --- |
| ```` ```rust ```` | Highlighted code | Copy |
| ```` ```rust,runnable ```` | Code + Run button + output | Run, copy |
| ```` ```rust,editable ```` | A small inline editor | Edit, run, reset, copy |
| ```` ```rust,does_not_compile ```` | Code with a "won't compile" badge | Run it to see the real compiler error, highlighted at the right line |
| ```` ```rust,panics ```` | Code with a "panics" badge | Run it to see the panic message |

- **Hidden setup lines:** lines starting with ` #  ` are sent to the compiler but hidden in the page (the same convention as rustdoc), so snippets can skip boilerplate like `use` lines or a `#[tokio::main]` wrapper.
- **Titles:** an optional `title="Moving a String"` after the fence label shows above the snippet and in the library.
- **"Try it in the editor":** every runnable or editable snippet has a button that copies it into the main editor.
- **Snippet library (`/snippets`):** every snippet from the concepts the learner has unlocked, grouped by track and chapter, searchable by title and keyword, with a star button to save favourites. Starred snippets are kept in `Progress`.
- **Snippets are tested in CI:** an ignored-by-default test runs every snippet through the runner and checks that `runnable`/`editable` ones compile and exit 0, `does_not_compile` ones fail to compile, and `panics` ones panic. A broken snippet fails the build.

Example in a lesson:

````markdown
```rust,does_not_compile title="Using a value after a move"
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}, world!");
}
```

```rust,editable title="Fix it with a clone"
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{s1}, world! ({s2})");
}
```
````

## Frontend (Leptos → WebAssembly)

The frontend is a client-side-rendered Leptos app compiled to WASM by Trunk and served by Vercel as static files. CSR (not SSR) is deliberate: it keeps hosting to a static folder plus a few functions, which is the simplest thing that works on Vercel.

**Routes**

| Path | Page |
| --- | --- |
| `/` | Home: course map with both tracks (Rust Book, Tokio), chapters and concepts marked locked, unlocked or done |
| `/learn/:track/:chapter` | Chapter intro: summary, source link, concept list, test-out |
| `/learn/:track/:chapter/:concept` | Concept lesson: markdown and snippets on the left, editor with the checkpoint on the right |
| `/learn/:track/:chapter/challenge` | Chapter challenge |
| `/snippets` | Snippet library: every unlocked snippet, searchable, with stars |
| `/credits` | Rust Book and Tokio attribution and licenses |
| `*` | 404; a locked route redirects to the course map with "Pass *X* to unlock" |

**Components**

- `CourseMap` — tracks → chapters → concepts, each with a locked / unlocked / done state from `content::unlocked()`
- `LessonView` — renders `body_md` with `pulldown-cmark` and swaps each code fence for a `Snippet`
- `Snippet` — one component for all five snippet kinds: badge, Run, Reset, Copy, "Try it in the editor", star
- `CodeEditor` — wraps the editor (below); props: initial code, `on_change`, a `diagnostics` signal
- `OutputPanel` — tabs for Output / Problems / Tests
- `HintDrawer` — reveals hints one at a time; last button shows the solution only after 3 failed submits
- `UnlockToast` — "Unlocked: *Borrowing*" with a Go button
- `TopBar` — Rustly logo, track switcher, link to the snippet library, "reset code" button

**The editor**

A good code editor (cursor, undo, syntax colours, error squiggles) doesn't exist as a pure-Rust WASM crate yet, so use **Monaco** loaded from a CDN in `index.html`, controlled from Rust through a thin `wasm-bindgen` binding:

```rust
#[wasm_bindgen(module = "/src/editor.js")]
extern "C" {
    fn create_editor(el: &HtmlElement, code: &str, on_change: &Closure<dyn Fn(String)>) -> JsValue;
    fn set_markers(editor: &JsValue, markers_json: &str);
    fn get_value(editor: &JsValue) -> String;
    fn set_value(editor: &JsValue, code: &str);
}
```

`editor.js` should stay under \~50 lines and contain no app logic — only calls into Monaco. If the course requires literally zero JavaScript, fall back to a `<textarea>` with a line-number gutter and render diagnostics as a list below it; everything else stays the same.

**Live checking flow (the "code as you learn" feature)**

1. Every edit updates a `code` signal and saves a draft to `localStorage` (key: `draft:<exercise id>`).
2. A debounce of \~1 s after the last keystroke fires `POST /api/check`.
3. An in-flight request is dropped if a newer edit arrives (keep a request counter and ignore stale responses).
4. The response's `Vec<Diagnostic>` becomes Monaco markers (red for errors, yellow for warnings) and the Problems tab.
5. Show a small status chip: "Checking…", "No errors", or "2 errors".
6. Skip the call if the code hash is the same as the last check.

**Run and Submit**

- **Run** → `POST /api/run`: shows stdout/stderr in the Output tab.
- **Submit** (exercises only) → `POST /api/submit`: shows each test as pass/fail; all green marks the exercise done, recomputes unlocked concepts, shows an "Unlocked" toast if anything opened, and a "Next" button.
- Disable both buttons while a request is in flight; show the playground's error text if the service is down.

**Styling:** plain CSS in `style/main.css` (Trunk bundles it), a dark theme by default, CSS grid for the split view, stacking vertically under 900 px wide.

## Backend API (Rust on Vercel Functions)

Four small Rust functions, each a `[[bin]]` using `vercel_runtime`. They validate input, call the `runner` crate, and return JSON. All request/response types come from `crates/shared`, so the frontend and backend can't drift apart.

| Route | Method | Purpose | Compile service call |
| --- | --- | --- | --- |
| `/api/check` | POST | Live diagnostics while typing | Clippy endpoint |
| `/api/run` | POST | Run the program, return output | Execute, `tests: false` |
| `/api/submit` | POST | Grade an exercise with hidden tests | Execute, `tests: true`, code + `hidden_tests` |
| `/api/health` | GET | Liveness + which runner URL is configured | none |

**Shared types (`crates/shared`)**

```rust
#[derive(Serialize, Deserialize)]
pub struct CodeRequest { pub code: String }

#[derive(Serialize, Deserialize)]
pub struct SubmitRequest { pub exercise_id: String, pub code: String }

#[derive(Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: Severity,          // Error | Warning | Note
    pub code: Option<String>,        // e.g. "E0382"
    pub message: String,
    pub line: u32, pub col: u32,     // 1-based, in the learner's code
    pub end_line: u32, pub end_col: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CheckResponse { pub ok: bool, pub diagnostics: Vec<Diagnostic> }

#[derive(Serialize, Deserialize)]
pub struct RunResponse { pub success: bool, pub stdout: String, pub stderr: String,
                         pub diagnostics: Vec<Diagnostic> }

#[derive(Serialize, Deserialize)]
pub struct TestCase { pub name: String, pub passed: bool, pub output: Option<String> }

#[derive(Serialize, Deserialize)]
pub struct SubmitResponse { pub compiled: bool, pub passed: bool, pub tests: Vec<TestCase>,
                            pub diagnostics: Vec<Diagnostic>, pub stderr: String }

#[derive(Serialize, Deserialize)]
pub struct ApiError { pub error: String }
```

**The `runner` crate**

- `PlaygroundClient` — base URL from the `RUNNER_URL` env var (default `https://play.rust-lang.org`), one shared `reqwest::Client` with a 15 s timeout, a `User-Agent` naming the project.
- `execute(code, tests: bool)` — sends `channel: stable, mode: debug, edition: 2024, crateType: bin, tests, backtrace: false, code`. Claude Code should confirm the current request/response field names against the [playground-api crate docs](https://docs.rs/playground-api/latest/playground_api/endpoints/struct.ExecuteRequest.html) or the open-source rust-playground repo before coding — the playground has no formal API contract.
- `clippy(code)` — same idea for live checking.
- `parse_diagnostics(stderr) -> Vec<Diagnostic>` — parses rustc's human-readable output: a header like `error[E0382]: borrow of moved value` followed by `--> src/main.rs:9:30`. Ignore diagnostics that point outside `src/main.rs`.
- `parse_tests(stdout) -> Vec<TestCase>` — parses `test tests::name ... ok|FAILED` lines and the failure output blocks.
- `assemble_submission(code, hidden_tests) -> String` — appends tests after the learner's code (never before, so line numbers stay correct).

The parsers are pure functions: put real captured playground outputs in `crates/runner/tests/fixtures/` and test against them.

**Guardrails**

- Reject code over 20 KB with HTTP 413; reject an unknown `exercise_id` with 404.
- Cache results by a hash of (endpoint, code) in a small in-memory LRU. Vercel reuses warm instances, so this cuts repeat calls cheaply; it's best-effort only.
- Map compile-service timeouts and 5xx responses to HTTP 502 with a friendly `ApiError`.
- Be a polite client: the public Playground is a free community service with no uptime promise. Debounce on the frontend, never call it on every keystroke, and keep `RUNNER_URL` configurable so a self-hosted runner can replace it.
- Never return `hidden_tests` or `solution` in any response.

## Progress storage

For the MVP, progress lives in the learner's browser via `gloo-storage` (`localStorage`). No accounts, no database, nothing to secure.

```rust
#[derive(Serialize, Deserialize, Default)]
pub struct Progress {
    pub version: u32,                          // bump + migrate if the shape changes
    pub completed: BTreeSet<String>,           // exercise ids (checkpoints, challenges, practice, test-outs)
    pub concepts_read: BTreeSet<String>,
    pub failed_submits: BTreeMap<String, u32>, // unlocks the solution after 3
    pub starred_snippets: BTreeSet<String>,
    pub last_visited: Option<String>,          // route to resume from
}
// Unlocked concepts are never stored: they're recomputed with content::unlocked(graph, &progress),
// so editing the course graph can't leave stale unlock state behind.
```

- Keys: `progress` (the struct above) and `draft:<exercise id>` (the learner's code).
- Wrap every read/write so a missing, corrupt or blocked store falls back to `Progress::default()` instead of crashing.
- Add Export / Import buttons (download/upload the JSON) so progress can move between browsers.
- Moving to server-side accounts later is a stretch goal; keep all storage behind a `storage.rs` module so it can be swapped.

## Deploying to Vercel

One Vercel project serves both halves: the Trunk build output as static files, and `api/*.rs` as Rust functions. Vercel's official [Rust runtime](https://vercel.com/docs/functions/runtimes/rust) is in **Beta** on all plans (docs last updated Dec 8, 2025), so do a hello-world deploy in Milestone 0 before building anything else.

**Root `Cargo.toml` (API part)** — follows Vercel's documented pattern:

```toml
[workspace]
members = ["crates/*", "frontend"]
default-members = [".", "crates/*"]   # keep the WASM app out of the API build
resolver = "2"

[package]
name = "rustly-api"
version = "0.1.0"
edition = "2024"

[dependencies]
tokio = { version = "1", features = ["full"] }
vercel_runtime = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
shared  = { path = "crates/shared" }
content = { path = "crates/content", features = ["server"] }
runner  = { path = "crates/runner" }

[[bin]]
name = "run"
path = "api/run.rs"

[[bin]]
name = "check"
path = "api/check.rs"

[[bin]]
name = "submit"
path = "api/submit.rs"

[[bin]]
name = "health"
path = "api/health.rs"

[profile.release]
codegen-units = 1
lto = "fat"
opt-level = 3
```

**`vercel.json`** — builds the frontend and routes everything that isn't `/api` to the SPA:

```json
{
  "installCommand": "curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal && . $HOME/.cargo/env && rustup target add wasm32-unknown-unknown && cargo install trunk --locked",
  "buildCommand": ". $HOME/.cargo/env && cd frontend && trunk build --release",
  "outputDirectory": "frontend/dist",
  "rewrites": [
    { "source": "/((?!api/).*)", "destination": "/index.html" }
  ]
}
```

Notes for Claude Code:

- `cargo install trunk` from source is slow (several minutes). If builds are too slow, download a prebuilt Trunk release binary in `installCommand`, or use **Plan B** below.
- Check whether the Rust function runtime needs any `functions` entry in `vercel.json` for the current Beta; follow the docs page, not memory.
- Set env var `RUNNER_URL` in Vercel project settings (Production and Preview).

**Plan B (if Vercel's build image fights the WASM build):** build in GitHub Actions and upload the result with `vercel build` then `vercel deploy --prebuilt`, using a `VERCEL_TOKEN` secret. The Rust toolchain then lives in CI, which is easier to cache.

**Local development**

- Frontend alone: `cd frontend && trunk serve` with `[[proxy]]` in `Trunk.toml` pointing `/api` at the local API.
- API: `vercel dev` from the root (needs the Vercel CLI and Rust installed locally).
- Simplest loop: run `vercel dev` and let it serve both.

**Deploy flow:** push to a branch → Vercel preview URL; merge to `main` → production. Turn on GitHub branch protection so CI must pass first.

## Build milestones

Nine milestones, each small enough for one Claude Code session. Don't start the next until every box in the current one is ticked. Commit at the end of each.

**M0 — Deploy spike (de-risk hosting first)**

- [ ] Workspace skeleton exists; `cargo build` passes at the root
- [ ] `/api/health` returns `{"ok":true}` on a real Vercel URL
- [ ] A Leptos "Hello Rustly" page is served from the same Vercel project
- [ ] Refreshing `/learn/anything` serves the app, not a Vercel 404
- [ ] If any box fails, switch to Plan B (CI prebuilt deploy) before continuing

**M1 — Shared types + runner crate**

- [ ] `shared` compiles for both `wasm32-unknown-unknown` and native
- [ ] `runner` can execute a hello-world through the playground (integration test, `#[ignore]` by default)
- [ ] `parse_diagnostics` and `parse_tests` pass fixture tests built from real captured outputs (at least: clean build, one error, multiple errors + warnings, one failing test, panic)

**M2 — Content crate + first chapters**

- [ ] Loader parses `chapter.toml`, concept front matter, snippets from code fences, and exercise TOML; validation test fails on bad files
- [ ] `server` feature hides `hidden_tests` and `solution` from the frontend build
- [ ] Course graph validation passes (all `requires` exist, no cycles, every concept reachable, one checkpoint per concept) and `unlocked()` fixture tests pass
- [ ] Book chapters 3, 4 and 5 written: concepts with 2–4 snippets each, checkpoints and a challenge; every solution passes and every starter fails its hidden tests; every snippet behaves as its fence says
- [ ] Playground crate list checked for tokio, tokio-stream, tokio-util and bytes

**M3 — API endpoints**

- [ ] `/api/run`, `/api/check`, `/api/submit` work locally with `vercel dev` and on a preview deploy
- [ ] 413 / 404 / 502 errors return `ApiError` JSON
- [ ] Responses never contain hidden tests (add a test that asserts this)

**M4 — Frontend shell + lesson pages**

- [ ] Router, home page with the course map (locked / unlocked / done), concept lesson pages rendering markdown
- [ ] All five snippet kinds render and behave as specified; ` #  ` lines are hidden on the page but still compiled
- [ ] Locked concepts show their title, a lock icon and what unlocks them; locked URLs redirect to the course map
- [ ] Layout works at 1280 px and 375 px wide

**M5 — Editor + Run**

- [ ] Monaco loads, shows starter code with Rust syntax colouring
- [ ] Run shows stdout/stderr; buttons disable while running
- [ ] Drafts persist across page reloads; "Reset code" restores the starter

**M6 — Live diagnostics**

- [ ] Typing an error shows a red squiggle at the right line/column within \~2 s of stopping
- [ ] Stale responses are ignored; identical code is not re-checked
- [ ] Problems tab lists errors; clicking one moves the cursor there

**M7 — Exercises, grading, progress**

- [ ] Submit shows per-test pass/fail; passing a checkpoint unlocks the next concept and passing a challenge unlocks the next chapter, with a toast and a Next button
- [ ] Test-out completes a whole chapter; the Tokio track unlocks after Book chapters 10, 13 and 16
- [ ] Hints reveal one at a time; solution unlocks after 3 failed submits
- [ ] `/snippets` lists every unlocked snippet, is searchable, and keeps stars across reloads
- [ ] Export/Import progress works

**M8 — Polish + final deploy**

- [ ] CI green on `main`; production URL live
- [ ] README with architecture summary, local setup and a screenshot/GIF
- [ ] All 15 MVP chapters written (10 Book + 5 Tokio); /credits page and THIRD\_PARTY\_LICENSES.md in place
- [ ] Lighthouse accessibility score checked; keyboard-only walk-through done

## Testing and CI

- **Unit tests** (`cargo test`): runner parsers against fixtures, content loader and course-graph validation, \`unlocked()\` fixtures, code assembly, shared type round-trips.
- **Integration tests** (`cargo test -- --ignored`): real calls to the compile service, including every reference solution passing, every starter failing, and every snippet behaving as its fence says. Run manually and nightly, not on every push, to stay polite to the Playground.
- **Frontend:** `cargo check --target wasm32-unknown-unknown -p frontend` and `trunk build` in CI; a few `wasm-bindgen-test` tests for storage and debounce logic are a bonus.
- **CI workflow** (`.github/workflows/ci.yml`): `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, frontend check + build. Cache with `Swatinem/rust-cache`.

## Risks and fallbacks

| Risk | Fallback |
| --- | --- |
| Vercel Rust runtime (Beta) misbehaves with the workspace layout | Move API crates out of the workspace or use Plan B prebuilt deploys; last resort, host the API on Shuttle or Fly.io and keep only the frontend on Vercel |
| Public Playground is slow, down, or rate-limits | Show a clear error; self-host the open-source rust-playground in Docker and change `RUNNER_URL` |
| Trunk/WASM build too slow on Vercel | Prebuilt Trunk binary, or CI prebuilt deploy |
| Playground response format changes | All parsing is in `runner` with fixture tests, so breakage shows up in one place |
| Learners see hidden tests | `server` feature keeps them out of the WASM bundle; API never returns them |
| Monaco + WASM interop gets fiddly | Textarea fallback; diagnostics as a list |

## Stretch goals (after the MVP)

- GitHub sign-in and server-side progress (Postgres via the Vercel marketplace, `sqlx`)
- "Explain this error" panel linking rustc error codes (E0382 etc.) to the official error index
- Format button (the Playground also exposes rustfmt)
- A self-hosted runner so you control uptime and limits
- Authoring mode: preview a lesson file before committing it
- Leaderboard or streaks

## Kickoff prompt for Claude Code

Save this doc as `HANDOFF.md` in an empty repo, then paste:

```text
Read HANDOFF.md fully. It is the spec for Rustly, my Rust capstone: a Rust-learning
platform written entirely in Rust (Leptos CSR frontend, Rust Vercel functions backend,
shared crates), deployed on Vercel.

Rules:
- Work one milestone at a time, starting with M0. Do not start the next milestone until
  every acceptance box in the current one is done and you've shown me the evidence
  (command output, URL, or test results).
- Before using any crate or Vercel feature, check its current docs/version; the doc
  flags where things may have changed (Vercel Rust runtime is Beta; Playground API
  field names must be verified).
- Keep api/*.rs thin; put logic in crates/runner with fixture-based tests.
- Never send hidden_tests or solution to the browser.
- Lessons follow the Rust Book and the Tokio tutorial (see Curriculum). Write concise
  lessons in our own words, link to the original page, keep license credits current,
  and adapt anything needing stdin, files or network into pure, testable functions.
- Exercises gate progress: a checkpoint unlocks the next concept, a challenge unlocks the
  next chapter. Every concept lesson has 2–4 code snippets, all tested in CI.
- Run cargo fmt, clippy and tests before each commit. Commit at the end of each milestone
  with a clear message.
- When something in HANDOFF.md turns out wrong, tell me, propose the fix, and update
  HANDOFF.md once I agree.

Start with M0: scaffold the workspace and get /api/health plus a Leptos hello page live
on a Vercel preview URL.
```

## Sources

- [The Rust Programming Language (table of contents)](https://doc.rust-lang.org/book/toc.html)
- [rust-lang/book repository (MIT / Apache-2.0)](https://github.com/rust-lang/book)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [tokio-rs/website repository (MIT)](https://github.com/tokio-rs/website)
- [Vercel: Using the Rust Runtime with Vercel functions](https://vercel.com/docs/functions/runtimes/rust)
- [playground-api crate: ExecuteRequest](https://docs.rs/playground-api/latest/playground_api/endpoints/struct.ExecuteRequest.html)
- [playground-api crate: endpoint list](https://docs.rs/playground-api/latest/playground_api/endpoints/index.html)
- [Rust Playground](https://play.rust-lang.org)
