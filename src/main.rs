use webgpu_demo::sync_run;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

fn main() {
    sync_run();
}
