extern crate console_error_panic_hook;

use webgpu_demo::run;

// #[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
fn main() {
    log_init();
    pollster::block_on(run());
}

fn log_init() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");
        } else {
            use env_logger;
            env_logger::init();
        }
    }
}
