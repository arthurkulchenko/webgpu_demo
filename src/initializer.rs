use crate::{Config, append_canvas};
use std::{panic, sync::Arc};
use winit::{window::Window, event::*, event_loop::{EventLoop}, keyboard::Key};
use tracing::{info, warn, error};
use wgpu::{Surface, SurfaceConfiguration, SurfaceTexture, SurfaceError, TextureView, CommandEncoder, Device, Queue, Limits};

pub fn initialize(config: Arc<Config>) -> (winit::window::Window, winit::event_loop::EventLoop<()>) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            panic::set_hook(Box::new(console_error_panic_hook::hook));
            tracing_wasm::set_as_global_default();
        } else {
            tracing_subscriber::fmt::init();
        }
    }
    // warn!("warn");
    // info!("info");
    // error!("error");
    let runtime = EventLoop::new().unwrap();
    let window_attributes = Window::default_attributes()
        .with_title("wgpu canvas")
        .with_inner_size(winit::dpi::LogicalSize::new(config.width, config.height));
        // .build(&runtime).unwrap();
    let window = Some(runtime.create_window(window_attributes).unwrap());
    let window = append_canvas(window.expect("REASON"));

    (window, runtime)
}
