// mod surface_state;
mod error;
use error::WDError;

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
use winit::window::Window;
use winit::{ event::*, event_loop::{EventLoop}, window::WindowBuilder, };

// use wasm_bindgen_test::__rt::node::Node;
// use web_sys::Node;

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

// use surface_state::State;
// NOTICE:
// WIP:
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    log_init();

    let event_loop = EventLoop::new().unwrap();
    let window: winit::window::Window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")] {
        let winit_canvas: web_sys::HtmlCanvasElement = window.canvas().unwrap();
        let html_window = web_sys::window().ok_or(WDError::HtmlOperationError("Can't find window".into()));
        let html_document = html_window.and_then(|js_window| js_window.document().ok_or(WDError::HtmlOperationError("Can't find document".into())));
        let html_body = html_document.clone().and_then(|document| document.body().ok_or(WDError::HtmlOperationError("Can't find body".into())));

        html_body.and_then(|body| {
            info!("Appedning canvas to body.");
            body.append_child(&winit_canvas).map_err(|err| WDError::HtmlOperationError(err.as_string().expect("Can't append canvas")))
        }).unwrap();

        // let canvas = web_sys::window().and_then(|js_window| js_window.document());
                                      // .and_then(|document| document.get_elements_by_tag_name("canvas"))
                                      // .and_then(|canvas| canvas.get_with_index(0))
                                      // .expect("Couldn't get canvas element by id.");

        // let swindow = web_sys::window();
        // let sdoc = swindow.and_then(|win| win.document());
        // let snodes: Option<web_sys::NodeList> = sdoc.and_then(|doc| { doc.query_selector_all(&format!("[data-raw-handle=\"{}\"]", 1)).ok()});
        // let snode: web_sys::Node = snodes.and_then(|nodes| nodes.get(0)).expect("WHERE IS THE canvas").into();
        // info!("snode.parent_element is {:?}", snode.parent_element());

        // let html_collection = html_document.and_then(|document| Ok(document.get_elements_by_tag_name("canvas"))).unwrap();
        // let canvas_on_page: web_sys::Element = html_collection.get_with_index(0).ok_or(WDError::HtmlOperationError("Couldn't get canvas".to_owned())).unwrap();
        // info!("{:?}", canvas_on_page); // Element



                            // web_sys::window().and_then(|win| win.document())
                            // .and_then(|doc| { doc.query_selector_all(&format!("[data-raw-handle=\"{}\"]", 1)).ok()})
                            // .and_then(|nodes| nodes.get(0))
                            // .expect("expected to find single canvas").into();
                            // canvas_node.set_attribute("width", "800");




//         window
// match target {
//             SurfaceTargetUnsafe::RawHandle {
//                 raw_display_handle: _,
//                 raw_window_handle,
//             } => {
//                 let canvas_element: web_sys::HtmlCanvasElement = match raw_window_handle {
//         raw_window_handle::RawWindowHandle::Web(handle) => {
//                         let canvas_node: wasm_bindgen::JsValue = web_sys::window()
//                             .and_then(|win| win.document())
//                             .and_then(|doc| {
//                                 doc.query_selector_all(&format!(
//                                     "[data-raw-handle=\"{}\"]",
//                                     handle.id
//                                 ))
//                                 .ok()
//                             })
//                             .and_then(|nodes| nodes.get(0))
//                             .expect("expected to find single canvas")
//                             .into();
//                         canvas_node.into()
//                     }
//                     raw_window_handle::RawWindowHandle::WebCanvas(handle) => {
//                         let value: &JsValue = unsafe { handle.obj.cast().as_ref() };
//                         value.clone().unchecked_into()
//                     }
//                     raw_window_handle::RawWindowHandle::WebOffscreenCanvas(handle) => {
//                         let value: &JsValue = unsafe { handle.obj.cast().as_ref() };
//                         let canvas: web_sys::OffscreenCanvas = value.clone().unchecked_into();
//                         let context_result = canvas.get_context("webgpu");

//                         return self.create_surface_from_context(
//                             Canvas::Offscreen(canvas),
//                             context_result,
//                         );
//                     }
//                     _ => panic!("expected valid handle for canvas"),




    }


    let size = window.inner_size();
    let instance_descriptor = wgpu::InstanceDescriptor { backends: wgpu::Backends::all(), ..Default::default() };
    let instance = wgpu::Instance::new(instance_descriptor);
    let surface = instance.create_surface(&window).unwrap();

    // canvas.get_context("webgpu");

    // let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::default(), compatible_surface: Some(&surface), force_fallback_adapter: false, },).await.unwrap();
    // let (device, _queue) = adapter.request_device(&wgpu::DeviceDescriptor { required_features: wgpu::Features::empty(), required_limits: if cfg!(target_arch = "wasm32") { wgpu::Limits::downlevel_webgl2_defaults() } else { wgpu::Limits::default() }, label: None, }, None,).await.unwrap();
    // let surface_caps = surface.get_capabilities(&adapter);
    // let surface_format = surface_caps.formats.iter().copied().filter(|f| f.is_srgb()).next().unwrap_or(surface_caps.formats[0]);
    // let config = wgpu::SurfaceConfiguration { usage: wgpu::TextureUsages::RENDER_ATTACHMENT, format: surface_format, width: size.width, height: size.height, present_mode: surface_caps.present_modes[0], alpha_mode: surface_caps.alpha_modes[0], view_formats: vec![], desired_maximum_frame_latency: 2 };
    // surface.configure(&device, &config);


    let window_id_clone = window.id().clone();

    // winit_event_loop.run(move |event, _, event_handler| match event {
    let _ = event_loop.run(move |event, event_handler| match event {
        Event::WindowEvent { ref event, window_id, } if window_id == window_id_clone => match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput { event: KeyEvent { logical_key: Key::Named(winit::keyboard::NamedKey::Exit), .. }, ..} => { event_handler.exit(); },
            // WindowEvent::Resized(physical_size) => { state.resize(*physical_size); },
            // WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            //     // new_inner_size is &&mut so we have to dereference it twice
            //     state.resize(**new_inner_size);
            // },
            _ => {}
        },
        _ => {}
    });
}
