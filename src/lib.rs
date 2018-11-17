extern crate wasm_bindgen;
extern crate web_sys;

pub mod events;
mod render_loop;
use render_loop::*;
use wasm_bindgen::prelude::*;

pub struct Callback<'r>(pub &'r FnMut(f64));

//unsafe impl Send for App {}

pub struct AppConfig {
    pub title: String,
    pub size: (u32, u32),
    pub vsync: bool,
}

impl AppConfig {
    pub fn new<T: Into<String>>(title: T, size: (u32, u32)) -> AppConfig {
        AppConfig {
            title: title.into(),
            size,
            vsync: true,
        }
    }
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
}

mod app;
pub use app::*;
