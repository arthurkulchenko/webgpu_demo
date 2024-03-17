mod surface_state;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::dpi::PhysicalSize;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;

use winit::{event::*, event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder},};
use log::info;

use surface_state::State;
// NOTICE: 
// WIP:
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() {
    let winit_event_loop = EventLoop::new();

    // let winit_window = WindowBuilder::new().build(&winit_event_loop).unwrap();
    let winit_window = WindowBuilder::new().build(&winit_event_loop).unwrap();
    let winit_window_2 = WindowBuilder::new().build(&winit_event_loop).unwrap();

    let mut state = State::new(winit_window_2).await;

    #[cfg(target_arch = "wasm32")]
    {
        // NOTICE: Winit prevents sizing with CSS, so we have to set the size manually when on web.
        // NOTICE: Only half of the size is set to make the canvas
        state.window().set_inner_size(PhysicalSize::new(200*2, 200*2));
        // winit_window.set_inner_size(PhysicalSize::new(200, 200));
        let canvas2 = web_sys::Element::from(state.window().canvas());
        let canvas = web_sys::Element::from(winit_window.canvas());
        // let canvas = web_sys::Element::from(winit_window.canvas());

        web_sys::window().and_then(|js_window| js_window.document()).and_then(|document| {
            // NOTICE: Append winit window with event loop reference inside.
            document.get_element_by_id("body")?.append_child(&canvas2).ok()?;
            document.get_element_by_id("body")?.append_child(&canvas).ok()?;
            Some(())
        }).expect("Couldn't append canvas to document body.");
    }

    winit_event_loop.run(move |event, _, control_flow| match event {

        // Event::WindowEvent { ref event, window_id } if window_id == state.window().id() => match event {
        Event::WindowEvent { ref event, window_id } if window_id == winit_window.id() => match event {

            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                // NOTICE: ".." means we don't care about the rest of the values in the struct while matching
                // We pattern match like we create Key and this key is what we are interested in
                input: KeyboardInput { state: ElementState::Pressed, virtual_keycode: Some(VirtualKeyCode::Escape), .. }, ..
            } => { *control_flow = ControlFlow::Exit },
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
