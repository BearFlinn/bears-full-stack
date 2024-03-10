pub mod view;
#[cfg(feature = "ssr")]
pub mod controller;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::view::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}