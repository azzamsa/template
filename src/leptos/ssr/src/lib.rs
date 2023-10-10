use cfg_if::cfg_if;
pub mod app;
pub mod error;
pub mod fallback;
pub mod logger;

pub use error::Error;

cfg_if! {
if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;
    use tracing_log::log;

    #[wasm_bindgen]
    pub fn hydrate() {
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();
        logging::log!("hydrate mode - hydrating");

        leptos::mount_to_body(move || {
            view! { <App/> }
        });
    }
}}
