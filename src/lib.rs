#![feature(nll)]


#[cfg(all(target_arch = "wasm32",feature = "stdw"))]
#[macro_use]
extern crate stdweb;

#[cfg(all(target_arch = "wasm32",not(feature="stdw")))]
#[macro_use]
extern crate lazy_static;

#[cfg(target_arch = "wasm32")]
pub mod events;

#[cfg(not(target_arch = "wasm32"))]
extern crate glutin;

#[cfg(not(target_arch = "wasm32"))]
pub mod events {
    pub use glutin::*;
}




pub struct Callback<'r>(pub &'r FnMut(f64));

unsafe impl Send for App {}




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



mod app;
pub use app::*;
