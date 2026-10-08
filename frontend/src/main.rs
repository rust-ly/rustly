mod app;
mod learner;
mod storage;
mod topbar;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
