// mod surface_state;
mod error;
mod type_is;

// use type_is::TypeIs;

extern crate console_error_panic_hook;

use std::panic;
use winit::{ event::*, event_loop::{EventLoop}, keyboard::Key };
use tracing::{error, info, warn};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use error::WDError;
#[cfg(target_arch = "wasm32")]
use winit::platform::web;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;

fn log_init() {
    info!("Initializing logging");
    warn!("Initializing logging");
    error!("Initializing logging");

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            panic::set_hook(Box::new(console_error_panic_hook::hook));
            tracing_wasm::set_as_global_default();
        } else {
            tracing_subscriber::fmt::init();
        }
    }
}

// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);

//     // The `console.log` is quite polymorphic, so we can bind it with multiple
//     // signatures. Note that we need to use `js_name` to ensure we always call
//     // `log` in JS.
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_u32(a: u32);

//     // Multiple arguments too!
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_many(a: &str, b: &str);
// }

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    log_init();

    let event_loop = EventLoop::new().unwrap();

    // let win_builder = winit::window::WindowBuilder::new().with_inner_size(window_size);
    // let win_builder = win_builder.with_inner_size(window_size).with_title("What does the fox say?"); // canvas alt="What does the fox say?"

    let window = winit::window::Window::new(&event_loop).unwrap();
    let size = window.inner_size();

    #[cfg(target_arch = "wasm32")]
    {
        let canvas: wgpu::web_sys::HtmlCanvasElement = window.canvas().unwrap();

        canvas.set_width(200);
        canvas.set_height(200);
        canvas.set_title("what does the fox say?");
        let styles = "border: 1px solid black; background: grey; margin: 10px; padding: 10px;";
        let _ = canvas.set_attribute("style", styles);
        // info!("{:#?}", canvas.get_context("webgpu"));

        wgpu::web_sys::window()
            .ok_or(WDError::HtmlError("Can't find window".into()))
            .and_then(|js_window| js_window.document().ok_or(WDError::HtmlError("Can't find document".into())))
            .and_then(|document| document.body().ok_or(WDError::HtmlError("Can't find body".into())))
            .and_then(|body| {
                body.append_child(&canvas).map_err(|err| WDError::HtmlError(err.as_string().expect("Can't append canvas")))
            }).unwrap();
    }

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::all(), ..Default::default() });
    let surface = instance.create_surface(&window).unwrap();

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

    let mut config = wgpu::SurfaceConfiguration {
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

    let window_id_clone = window.id().clone();

    let _ = event_loop.run(move |event, event_handler| match event {
        Event::WindowEvent { ref event, window_id, } if window_id == window_id_clone => match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput { event: KeyEvent { logical_key: Key::Named(winit::keyboard::NamedKey::Exit), .. }, ..} => { event_handler.exit(); },
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
    });
}
