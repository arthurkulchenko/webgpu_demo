#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
    // keyboard::{Key, NamedKey},
};

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn run() {
    // let event_loop = EventLoop::new().unwrap();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;

        window.set_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window().and_then(|win| win.document())
                         .and_then(|doc| {
                             let dst = doc.get_element_by_id("body")?;
                             let canvas = web_sys::Element::from(window.canvas());
                             dst.append_child(&canvas).ok()?;
                             Some(())
                         }).expect("Couldn't append canvas to document body.");
    }


    // let _ = event_loop.run(move |event, eventloop_target_ref| match event {
    //             Event::WindowEvent { ref event, window_id, } if window_id == window.id() => match event {
    //         WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
    //             event: KeyEvent {
    //                        logical_key: Key::Named(NamedKey::Escape),
    //                        state: ElementState::Pressed,
    //                        ..
    //                    },
    //             ..
    //         } => eventloop_target_ref.exit(),
    //         _ => {}
    //     },
    //     _ => {}
    // });
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { ref event, window_id } if window_id == window.id() => match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}
