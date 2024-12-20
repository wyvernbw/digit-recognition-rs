extern crate blas_src;

pub mod executor;
pub mod gpu;

pub const MMUL: &[u8] =
    include_bytes!("../../target/spirv-builder/spirv-unknown-spv1.5/release/deps/leafeon_mmul.spv");

pub use executor::dot;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            tracing_subscriber::fmt::init();
        }
    }
}
