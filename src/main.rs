use webgpu_demo::run;
// use tracing::{error, info, warn};
// use winit::{ event::*, event_loop::{EventLoop}, window::WindowBuilder, };


#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
fn main() {
    pollster::block_on(run());
}
