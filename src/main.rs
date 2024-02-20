// mod main_window;

// use main_window::run;
extern crate console_error_panic_hook;
// use std::panic;

use webgpu_demo::run;
use env_logger;
// use log::info;

fn main() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    run();
}
