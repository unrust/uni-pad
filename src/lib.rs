#![recursion_limit = "512"]

#[cfg(target_arch = "wasm32")]
#[path = "web_pad.rs"]
pub mod pad;

// NOT wasm-unknown-unknown
#[cfg(not(target_arch = "wasm32"))]
extern crate gamepad_rs;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native_pad.rs"]
pub mod pad;

pub use self::pad::*;
