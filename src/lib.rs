// mod surface_state;
mod error;
mod type_is;

use error::WDError;
use type_is::TypeIs;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::dpi::PhysicalSize;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;

extern crate console_error_panic_hook;
use std::panic;

use tracing::{error, info, warn};

use winit::keyboard::Key;
#[cfg(target_arch = "wasm32")]
use winit::window::Window;
#[cfg(target_arch = "wasm32")]
use winit::platform::web;

use winit::{ event::*, event_loop::{EventLoop}, window::WindowBuilder, };
#[cfg(target_arch = "wasm32")]
use wgpu::SurfaceTarget;

// use wasm_bindgen_test::__rt::node::Node;
// use wgpu::web_sys::Node;

fn log_init() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            // console_error_panic_hook::set_once();
            // std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            // console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");
            // tracing_wasm::set_as_global_default();
            panic::set_hook(Box::new(console_error_panic_hook::hook));
            // console_error_panic_hook::set_once();
            tracing_wasm::set_as_global_default();
        } else {
            // use env_logger;
            // env_logger::init();
            tracing_subscriber::fmt::init();
        }
    }
}


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// pub enum SurfaceTarget<'window> {
//     Window(Box<dyn WindowHandle + 'window>),
//     Canvas(HtmlCanvasElement),
//     OffscreenCanvas(OffscreenCanvas),
// }

#[cfg(target_arch = "wasm32")]
pub fn log_match<'window>(target: impl Into<SurfaceTarget<'window>> + std::marker::Copy,) {
// pub fn log_match<'window>(target: &wgpu::web_sys::OffscreenCanvas) {
    // info!("target: {:?}", target);
    info!("Architecture: {}", std::env::consts::ARCH);

    // let my_instance = TypeIs(target);
    // info!("{:#?}", my_instance.canvas());
    // let var: wgpu::web_sys::OffscreenCanvas = target.into();
    // let var = target.into();
    match target {
        // SurfaceTarget::Window(winit_window) => { info!("****** Window"); },
        #[cfg(any(webgpu, webgl))]
        SurfaceTarget::Canvas(html_canvas) => { info!("****** Canvas"); },
        #[cfg(any(webgpu, webgl))]
        SurfaceTarget::OffscreenCanvas(offscreen_canvas) => { info!("****** OffscreenCanvas"); },

        _ => { info!("****** Unknown"); }
    }
}

// use surface_state::State;
// NOTICE:
// WIP:
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    log_init();

    let event_loop = EventLoop::new().unwrap();
    // let window_size = winit::dpi::PhysicalSize::new(800, 800);
    // info!("window_size = {:#?}\n", window_size);

    // let win_builder = winit::window::WindowBuilder::new().with_inner_size(window_size);
    // let win_builder = win_builder.with_inner_size(window_size)
    //                              .with_title("What does the fox say?"); // canvas alt="" id="example" width="300" height="300"

    // info!("{:?}", win_builder);
    // info!("window_attributes = {:#?}\n", &win_builder.window_attributes());
    // let new_window = win_builder.build(&event_loop).unwrap();

    let window = winit::window::Window::new(&event_loop).unwrap();

    // info!("\n NEW WINDOW: {:#?}\n", new_window);

    // if let Some(result) = window.request_inner_size(window_size) {
    //     info!("request_inner_size = {:#?}\n", result);
    // } else {
    //     info!("NONE \n");
    // };

    // info!("\n NEW WINDOW: {:#?}\n", window);

    let size = window.inner_size();
    // info!(" ~~~~~~~~~~~ window size: {:?}\n", size);
    // let window = winit::window::WindowBuilder::new().with_canvas(canvas).build(&event_loop).unwrap();

    // wgpu::web_sys::window


    #[cfg(target_arch = "wasm32")]
    {
        let styles = "border: 1px solid black; background: grey; margin: 10px; padding: 10px;";

        let winit_canvas: wgpu::web_sys::HtmlCanvasElement = window.canvas().unwrap();
        winit_canvas.set_width(200);
        winit_canvas.set_height(200);
        winit_canvas.set_title("what does the fox say?");
        winit_canvas.set_attribute("style", styles);

        // info!("{:?}", winit_canvas.get_context("webgpu"));

        wgpu::web_sys::window().ok_or(WDError::HtmlOperationError("Can't find window".into()))
            .and_then(|js_window| js_window.document().ok_or(WDError::HtmlOperationError("Can't find document".into())))
            .and_then(|document| document.body().ok_or(WDError::HtmlOperationError("Can't find body".into())))
            .and_then(|body| {
                // let offscreen_canvas = wgpu::web_sys::OffscreenCanvas::new(400, 400).unwrap();
                // body.append_child(&offscreen_canvas).map_err(|err| WDError::HtmlOperationError(err.as_string().expect("Can't append canvas")))
                body.append_child(&winit_canvas).map_err(|err| WDError::HtmlOperationError(err.as_string().expect("Can't append canvas")))
            }).unwrap();
    }

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::all(), ..Default::default() });
    let surface = instance.create_surface(&window).unwrap();

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::default(), compatible_surface: Some(&surface), force_fallback_adapter: false, },).await.unwrap();
    let (device, _queue) = adapter.request_device(&wgpu::DeviceDescriptor { required_features: wgpu::Features::empty(), required_limits: if cfg!(target_arch = "wasm32") { wgpu::Limits::downlevel_webgl2_defaults() } else { wgpu::Limits::default() }, label: None, }, None,).await.unwrap();
    // let (device, _queue) = adapter.request_device(&wgpu::DeviceDescriptor { required_features: wgpu::Features::empty(), required_limits: if cfg!(target_arch = "wasm32") { wgpu::Limits::downlevel_webgl2_defaults() } else { wgpu::Limits::downlevel_defaults() }, label: None, }, None,).await.unwrap();
    // let (device, _queue) = adapter.request_device(&wgpu::DeviceDescriptor { required_features: wgpu::Features::all(), required_limits: wgpu::Limits::downlevel_webgl2_defaults(), label: None, }, None,).await.unwrap();
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats.iter().copied().filter(|f| f.is_srgb()).next().unwrap_or(surface_caps.formats[0]);
    // let config = wgpu::SurfaceConfiguration { usage: wgpu::TextureUsages::RENDER_ATTACHMENT, format: surface_format, width: size.width, height: size.height, present_mode: surface_caps.present_modes[0], alpha_mode: surface_caps.alpha_modes[0], view_formats: vec![], desired_maximum_frame_latency: 2 };
    let mut config = wgpu::SurfaceConfiguration { usage: wgpu::TextureUsages::RENDER_ATTACHMENT, format: surface_format, width: size.width, height: size.height, present_mode: surface_caps.present_modes[0], alpha_mode: surface_caps.alpha_modes[0], view_formats: vec![], desired_maximum_frame_latency: 2 };
    surface.configure(&device, &config);

    let window_id_clone = window.id().clone();

    // winit_event_loop.run(move |event, _, event_handler| match event {
    let _ = event_loop.run(move |event, event_handler| match event {
        Event::WindowEvent { ref event, window_id, } if window_id == window_id_clone => match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput { event: KeyEvent { logical_key: Key::Named(winit::keyboard::NamedKey::Exit), .. }, ..} => { event_handler.exit(); },
            WindowEvent::Resized(physical_size) => {
                config.width = physical_size.width / 2;
                config.height = physical_size.height / 2;
                surface.configure(&device, &config);
                // info!("Resized to {:?}", physical_size);
                // state.resize(*physical_size);
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
