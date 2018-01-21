#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use self::native::*;


#[cfg(all(target_arch = "wasm32",feature="stdw"))]
pub mod stdw;
#[cfg(all(target_arch = "wasm32",feature="stdw"))]
pub use self::stdw::*;


#[cfg(all(target_arch = "wasm32",not(feature="stdw")))]
pub mod web;
#[cfg(all(target_arch = "wasm32",not(feature="stdw")))]
pub use self::web::*;