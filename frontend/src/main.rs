mod app;
mod chapter;
mod course_map;
mod icons;
mod learner;
mod lesson;
mod routes;
mod storage;
mod topbar;

use app::NotFound;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
