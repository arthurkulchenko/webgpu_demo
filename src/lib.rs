// mod surface_state;
mod error;
mod type_is;
mod styles;

// use type_is::TypeIs;

extern crate console_error_panic_hook;

use std::panic;
use winit::{ event::*, event_loop::{EventLoop}, keyboard::Key };
use tracing::{info, warn, error};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use error::WDError;
#[cfg(target_arch = "wasm32")]
use winit::platform::web;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;

// fn render(surface)

async fn surface_presets(window: &winit::window::Window) -> (wgpu::Surface, wgpu::Device, wgpu::SurfaceConfiguration) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::all(), ..Default::default() });
    let surface = instance.create_surface(window).unwrap();
    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(), compatible_surface: Some(&surface), force_fallback_adapter: false,
        },
    ).await.unwrap();
    let (device, _queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: if cfg!(target_arch = "wasm32") { wgpu::Limits::downlevel_webgl2_defaults() } else { wgpu::Limits::default() },
            label: None,
        },
        None,
    ).await.unwrap();
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats.iter().copied().filter(|f| f.is_srgb()).next().unwrap_or(surface_caps.formats[0]);

    let size = window.inner_size();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2
    };
    surface.configure(&device, &config);
    (surface, device, config)
}

fn append_canvas(window: winit::window::Window) -> winit::window::Window {
    #[cfg(target_arch = "wasm32")]
    {
        let canvas: wgpu::web_sys::HtmlCanvasElement = window.canvas().unwrap();
        canvas.set_width(300);
        canvas.set_height(300);
        canvas.set_title("what does the fox say?");
        let _ = canvas.set_attribute("style", styles::CANVAS);
        // info!("{:#?}", canvas.get_context("webgpu"));
        wgpu::web_sys::window()
            .ok_or(WDError::HtmlError("Can't find window".into()))
            .and_then(|js_window| js_window.document().ok_or(WDError::HtmlError("Can't find document".into())))
            .and_then(|document| document.body().ok_or(WDError::HtmlError("Can't find body".into())))
            .and_then(|body| {
                let _ =body.set_attribute("style", styles::BODY);
                body.append_child(&canvas).map_err(|err| WDError::HtmlError(err.as_string().expect("Can't append canvas")))
            }).unwrap();
    }
    window
}

fn initialize() -> (winit::window::Window, EventLoop<()>) {
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
    let window = winit::window::WindowBuilder::new().with_title("wgpu canvas").build(&runtime).unwrap();
    let window = append_canvas(window);
    (window, runtime)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
async fn run() {
    let (window, runtime) = initialize();
    let (surface, device, mut config) = surface_presets(&window).await;

    let win_id = window.id().clone();
    let _ = runtime.run(
        move |event, event_handler| {
            match event {
                Event::WindowEvent { ref event, window_id, } if window_id == win_id => match event {
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        event: KeyEvent { logical_key: Key::Named(winit::keyboard::NamedKey::Escape), .. }, ..
                    } => {
                        event_handler.exit();
                    },
                    // NOTICE: WindowEvent::Resized event required for canvas display
                    WindowEvent::Resized(physical_size) => {
                        config.width = physical_size.width / 2;
                        config.height = physical_size.height / 2;
                        surface.configure(&device, &config);
                    },
                    // WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    //     // new_inner_size is &&mut so we have to dereference it twice
                    //     state.resize(**new_inner_size);
                    // },
                    _ => {}
                },
                _ => {}
            }
        }
    );
}

pub fn sync_run() {
    pollster::block_on(run());
}
