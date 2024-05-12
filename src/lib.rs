#![allow(warnings)]
// mod surface_state;
mod error;
mod type_is;
mod styles;
mod surface_presets;
mod events;
mod initializer;

// use type_is::TypeIs;
extern crate console_error_panic_hook;

use crate::surface_presets::{surface_presets, append_canvas};
use crate::events::*;
use crate::events::redraw_requested::*;
use crate::initializer::initialize;

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
                        // config.width = physical_size.to_logical(1.0).width;
                        // config.height = physical_size.to_logical(1.0).height;
                        surface.configure(&device, &config);
                    },
                    WindowEvent::ScaleFactorChanged { ref mut inner_size_writer, .. } => {
                        // NOTICE: Will reduce the size of the surface but not increase it (web responcive mode)
                        inner_size_writer.request_inner_size(size).unwrap();
                    },
                    WindowEvent::RedrawRequested if window_id == win_id => {
                        info!("redrawed");
                        update();
                        let surface_texture = surface.get_current_texture().unwrap();
                        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Encoder"), });

                        match render(&mut queue, surface_texture, encoder, view) {
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
