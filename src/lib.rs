// mod surface_state;
mod error;
mod type_is;
mod styles;

// use type_is::TypeIs;

extern crate console_error_panic_hook;

use std::panic;
use winit::{ event::*, event_loop::{EventLoop}, keyboard::Key };
use tracing::{info, warn, error};
use wgpu::{ Surface, SurfaceConfiguration, SurfaceTexture, SurfaceError, TextureView, CommandEncoder, Device, Queue, Limits };

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use error::WDError;
#[cfg(target_arch = "wasm32")]
use winit::platform::web;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;

// fn render(surface)
fn initialize() -> (winit::window::Window, winit::event_loop::EventLoop<()>) {
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

async fn surface_presets(window: &winit::window::Window) -> (Surface, Device, Queue, SurfaceConfiguration) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::all(), ..Default::default() });
    let surface = instance.create_surface(window).unwrap();
    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(), compatible_surface: Some(&surface), force_fallback_adapter: false,
        },
    ).await.unwrap();
    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: if cfg!(target_arch = "wasm32") { Limits::downlevel_webgl2_defaults() } else { Limits::default() },
            label: None,
        },
        None,
    ).await.unwrap();
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats.iter().copied().filter(|f| f.is_srgb()).next().unwrap_or(surface_caps.formats[0]);

    let size = window.inner_size();
    info!("size: {:?}", size);

    let config = SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        // NOTICE: Do nothing on native
        width: size.width,
        height: size.height,
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2
    };
    surface.configure(&device, &config);
    info!("config: {:#?}", config);


    (surface, device, queue, config)
}

fn append_canvas(window: winit::window::Window) -> winit::window::Window {
    #[cfg(target_arch = "wasm32")]
    {
        let canvas: wgpu::web_sys::HtmlCanvasElement = window.canvas().unwrap();
        // NOTICE: Not taken into account while running natively
        canvas.set_width(400);
        canvas.set_height(300);
        canvas.set_title("what does the fox say?");
        // let _ = canvas.set_attribute("style", styles::CANVAS);
        // info!("{:#?}", canvas.get_context("webgpu"));
        wgpu::web_sys::window()
            .ok_or(WDError::HtmlError("Can't find window".into()))
            .and_then(|js_window| js_window.document().ok_or(WDError::HtmlError("Can't find document".into())))
            .and_then(|document| document.body().ok_or(WDError::HtmlError("Can't find body".into())))
            .and_then(|body| {
                // let _ = body.set_attribute("style", styles::BODY);
                body.append_child(&canvas).map_err(|err| WDError::HtmlError(err.as_string().expect("Can't append canvas")))
            }).unwrap();
    }
    window
}

fn input(event: &WindowEvent) -> bool {
// fn input(&mut self, event: &WindowEvent) -> bool {
    false
}

fn update() {
}

use wgpu::{ Operations, LoadOp, StoreOp, Color, RenderPassColorAttachment };
fn color_attachments(view: &TextureView) -> Vec<Option<RenderPassColorAttachment>> {
    // NOTICE: Currently, we are clearing the screen with a bluish color
    vec![
        Some(
            RenderPassColorAttachment {
                view: view,
                resolve_target: None,
                // DOC: This tells wgpu what to do with the colors on the screen (specified by view)
                ops: Operations {
                    // DOC: The load field tells wgpu how to handle colors stored from the previous frame
                    load: LoadOp::Clear(Color { r: 0.3, g: 0.5, b: 0.6, a: 1.0, }),
                    // DOC: The store field tells wgpu whether we want to store the rendered results to the Texture behind our
                    // TextureView (in this case, it's the SurfaceTexture) We use StoreOp::Store as we do want
                    // to store our render results.
                    store: StoreOp::Store,
                },
            }
        ),
    ]
}

fn render(queue: &mut Queue, output: SurfaceTexture, mut encoder: CommandEncoder, view: TextureView) -> Result<(), SurfaceError> {
    {
        let _render_pass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &color_attachments(&view),
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            }
        );
    }

    // submit will accept anything that implements IntoIter
    queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
async fn run() {
    let (window, runtime) = initialize();
    let (surface, device, mut queue, mut config) = surface_presets(&window).await;

    let size = window.inner_size();

    let win_id = window.id().clone();
    let win_ref = &window;
    let _ = runtime.run(
        move |mut event, event_handler| {
            match event {
                // Event::WindowEvent { ref mut event, window_id, } if window_id == win_id => match event {
                // Event::WindowEvent { ref event, window_id, } if window_id == win_id => if input(event) match event {
                Event::WindowEvent { ref mut event, window_id, } if window_id == win_id && !input(event) => match event {
                    // NOTICE: Window events
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        event: KeyEvent { logical_key: Key::Named(winit::keyboard::NamedKey::Escape), .. }, ..
                    } => {
                        event_handler.exit();
                    },
                    // NOTICE: WindowEvent::Resized event required for canvas be displyed
                    WindowEvent::Resized(physical_size) => {
                        // DEBUG: Go nuts on web if not divided by 2, but on native it reduces surface in 4 times if divided by 2
                        config.width = physical_size.width / 2;
                        config.height = physical_size.height / 2;
                        info!("resized");
                        config.width = physical_size.to_logical(1.0).width;
                        config.height = physical_size.to_logical(1.0).height;
                        surface.configure(&device, &config);
                    },
                    WindowEvent::ScaleFactorChanged { ref mut inner_size_writer, .. } => {
                        // NOTICE: Will reduce the size of the surface but not increase it (web responcive mode)
                        inner_size_writer.request_inner_size(size).unwrap();
                    },
                    WindowEvent::RedrawRequested if window_id == win_id => {
                        info!("redrawed");
                        update();
                        let output = surface.get_current_texture().unwrap();
                        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Encoder"), });

                        match render(&mut queue, output, encoder, view) {
                            Ok(_) => {}
                            // Reconfigure the surface if lost
                            Err(wgpu::SurfaceError::Lost) => {
                                // config.width = physical_size.width / 2;
                                // config.height = physical_size.height / 2;
                                // surface.configure(&device, &config);
                            },
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                error!("Out of memory");
                                event_handler.exit();
                            },
                            // All other errors (Outdated, Timeout) should be resolved by the next frame
                            Err(e) => eprintln!("{:?}", e),
                        }
                    },
                    WindowEvent::CursorMoved { device_id, position } => {
                        win_ref.request_redraw();
                    },
                    _ => {}
                },
                
                // // NOTICE: RedrawRequested will only trigger once unless we manually request it.
                // Event::MainEventsCleared => { window.request_redraw(); },
                _ => {}
            }
        }
    );
    let win_idd = window.id().clone();
}

pub fn sync_run() {
    pollster::block_on(run());
}
