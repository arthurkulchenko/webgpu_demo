// mod main_window;

// use main_window::run;
extern crate console_error_panic_hook;
// use std::panic;

use webgpu_demo::run;
use env_logger;
use pollster::FutureExt as _;
// use log::info;

fn main() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let hook_ref = Box::new(console_error_panic_hook::hook);
            std::panic::set_hook(hook_ref);
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    // run();
    pollster::block_on(run());
}
