use crate::append_canvas;
use std::panic;
use winit::{ event::*, event_loop::{EventLoop}, keyboard::Key };
use tracing::{info, warn, error};
use wgpu::{ Surface, SurfaceConfiguration, SurfaceTexture, SurfaceError, TextureView, CommandEncoder, Device, Queue, Limits };

pub fn initialize() -> (winit::window::Window, winit::event_loop::EventLoop<()>) {
    info!("info");
    warn!("warn");
    error!("error");

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            panic::set_hook(Box::new(console_error_panic_hook::hook));
            tracing_wasm::set_as_global_default();
        } else {
            tracing_subscriber::fmt::init();
        }
    }

    let runtime = EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_title("wgpu canvas")
        .with_inner_size(winit::dpi::LogicalSize::new(600, 600))
        .build(&runtime).unwrap();
    let window = append_canvas(window);
    (window, runtime)
}
