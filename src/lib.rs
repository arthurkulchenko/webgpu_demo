#![allow(warnings)]
mod surface_state;
mod error;
mod type_is;
mod styles;
mod surface_presets;
mod events;
mod initializer;

// use type_is::TypeIs;
extern crate console_error_panic_hook;

use crate::surface_state::State;
use crate::MouseScrollDelta::LineDelta;
use winit::event::MouseScrollDelta;
use std::num::NonZeroU32;
use crate::surface_presets::{surface_presets, append_canvas};
use crate::events::*;
use crate::events::redraw_requested::*;
use crate::initializer::initialize;
use std::sync::Arc;

use std::panic;
use winit::{ event::*, event_loop::{EventLoop}, keyboard::{PhysicalKey, KeyCode, Key}, dpi::PhysicalPosition };

use tracing::{info, warn, error};
use wgpu::{ include_wgsl, Surface, SurfaceConfiguration, SurfaceTexture, SurfaceError, TextureView, CommandEncoder, Device, Queue, Limits };
// use wgpu::{, util::DeviceExt};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use error::WDError;
#[cfg(target_arch = "wasm32")]
use winit::platform::web;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;

pub struct Config {
    width: u32,
    height: u32
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
async fn run() {
    let initial_config = Arc::new(Config { width: 500, height: 500 });
    let (window, runtime) = initialize(Arc::clone(&initial_config));
    let win_id = window.id().clone();
    let mut state = surface_presets(&window, Arc::clone(&initial_config)).await;

    let _ = runtime.run(
        move |mut event, event_handler| {
            match event {
                // Event::WindowEvent { ref mut event, window_id, } if window_id == win_id => match event {
                // Event::WindowEvent { ref event, window_id, } if window_id == win_id => if input(event) match event {
                Event::WindowEvent { ref mut event, window_id, } if window_id == win_id && !input(event) => match event {
                    WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _, .. } => {
                        match event {
                            winit::event::KeyEvent { physical_key, .. } => {
                                match physical_key {
                                    PhysicalKey::Code(KeyCode::KeyR) => { info!("r Pressed") },
                                    PhysicalKey::Code(KeyCode::KeyG) => { info!("g Pressed") },
                                    PhysicalKey::Code(KeyCode::KeyB) => { info!("b Pressed") },
                                    _ => {}
                                }
                            }
                        }
                    }
                    WindowEvent::CursorMoved { device_id, position } => {
                        // state.window.request_redraw();
                    },
                    WindowEvent::MouseWheel { delta: MouseScrollDelta::PixelDelta(PhysicalPosition {x, y} ), .. } => {
                        // info!("{:?}, {:?}", x, y);
                        // let red: u32 = (state.color.r * 10_000.0) as u32;
                        // let blue: u32 = (state.color.b * 10_000.0) as u32;
                        // let green: u32 = (state.color.g * 10_000.0) as u32;

                        // let red = red + (*y * 10.0) as u32;
                        // let blue = blue + (*x * 10.0) as u32;
                        // let green = green + (*x * 10.0) as u32;

                        // let color = wgpu::Color { r: red as f64 / 10_000.0, g: green as f64 / 10_000.0, b: blue as f64 / 10_000.0, a: 1.0, };

                        // info!("{:?}, {:?}, {:?}", red, green, blue);
                        // let new_render = render(&mut state, color);
                        // state.color = color;
                        // info!("{:?}", color.r);

                        // state.window.request_redraw();
                    },
                    WindowEvent::ScaleFactorChanged { inner_size_writer, .. } => {
                        // NOTICE: Will reduce the size of the surface but not increase it (web responcive mode)
                        let size = state.window.inner_size();
                        inner_size_writer.request_inner_size(size).unwrap();
                    },
                    WindowEvent::Resized(physical_size) => {
                        // DEBUG: Go nuts on web if not divided by 2, but on native it reduces surface in 4 times if divided by 2
                        state.config.width = physical_size.width / 2;
                        state.config.height = physical_size.height / 2;
                        // info!("resized");
                        // config.width = physical_size.to_logical(1.0).width;
                        // config.height = physical_size.to_logical(1.0).height;
                        state.surface.configure(&state.device, &state.config);
                    },

                    WindowEvent::RedrawRequested if window_id == win_id => {
                        let new_render = render(&mut state);

                        match new_render {
                            Ok(_) => {},
                            // Reconfigure the surface if lost
                            Err(wgpu::SurfaceError::Lost) => {
                                // initial_config.width = physical_size.width / 2;
                                // initial_config.height = physical_size.height / 2;
                                // surface.configure(&device, &config);
                            },
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                error!("Out of memory");
                                event_handler.exit();
                            },
                            // All other errors (Outdated, Timeout) should be resolved by the next frame
                            Err(e) => eprintln!("{:?}", e),
                            _ => {},
                        }
                    },
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        event: KeyEvent { logical_key: Key::Named(winit::keyboard::NamedKey::Escape), .. }, ..
                    } => { event_handler.exit(); },
                    // NOTICE: HAS TO BE AT THE END
                    _ => {},
                },
                
                // // NOTICE: RedrawRequested will only trigger once unless we manually request it.
                // Event::MainEventsCleared => { window.request_redraw(); },
                _ => {}
            }
        }
    );
    // let win_idd = window.id().clone();
}

pub fn sync_run() {
    pollster::block_on(run());
}
