mod app;

use app::*;
use leptos::logging::log;
use leptos::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("csr mode - mounting to body");
    leptos::mount_to_body(|| view! { <App/> })
}
