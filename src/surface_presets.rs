use crate::{error::WDError, styles, Config, State};
use std::{sync::Arc, num::NonZeroU32};
use wgpu::{
    Trace, MemoryHints, Surface, SurfaceConfiguration, SurfaceTexture, SurfaceError,
    TextureView, CommandEncoder, Device, Queue, Limits
};
use tracing::{info, warn, error};
use winit::window::Window;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;

pub async fn surface_presets(window: &winit::window::Window, initial_config: Arc<Config>) -> State {
    let ctx_object = wgpu::Instance::new(&wgpu::InstanceDescriptor { backends: wgpu::Backends::all(), ..Default::default() });
    let surface = ctx_object.create_surface(window).unwrap();
    let adapter = ctx_object.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(), compatible_surface: Some(&surface), force_fallback_adapter: false,
        },
    ).await.unwrap();
    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            memory_hints: MemoryHints::MemoryUsage,
            trace: Trace::Off,
            required_features: wgpu::Features::empty(),
            required_limits: if cfg!(target_arch = "wasm32") { Limits::downlevel_webgl2_defaults() } else { Limits::default() },
            label: None,
        }
    ).await.unwrap();

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats.iter().copied().filter(|f| f.is_srgb()).next().unwrap_or(surface_caps.formats[0]);
    let config = SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        // NOTICE: Do nothing on native
        width: initial_config.width,
        height: initial_config.height,
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2
    };
    // let shader = device.create_shader_module(include_wgsl!("shader.wgsl"),);
    surface.configure(&device, &config);
    State::new(window, surface, device, queue, config).await
}

pub fn append_canvas(window: winit::window::Window) -> winit::window::Window {
    #[cfg(target_arch = "wasm32")]
    {
        let canvas: wgpu::web_sys::HtmlCanvasElement = window.canvas().unwrap();
        // canvas.get_context("webgpu");
        wgpu::web_sys::window()
            .ok_or(WDError::HtmlError("Can't find window".into()))
            .and_then(|js_window| js_window.document().ok_or(WDError::HtmlError("Can't find document".into())))
            .and_then(|document|
                document.get_element_by_id("webassembly-canvas-wrapper").ok_or(WDError::HtmlError("Can't find wrapper".into()))
            )
            .and_then(|canvas_wrapper| {
                let _ = canvas_wrapper.set_attribute("style", styles::BODY);
                canvas_wrapper.append_child(&canvas).map_err(|err| WDError::HtmlError(err.as_string().expect("Can't append canvas")))
            }).unwrap();
    }
    window
}
