#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use self::native::*;

#[cfg(all(target_arch = "wasm32"))]
pub mod stdw;
#[cfg(all(target_arch = "wasm32"))]
pub use self::stdw::*;
