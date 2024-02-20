use env_logger;
// use log::info;

use winit::{
    event::*,
    event_loop::{EventLoop},
    window::WindowBuilder,
    keyboard::{Key, NamedKey},
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let _ = event_loop.run(move |event, eventloop_target_ref| match event {
        Event::WindowEvent { ref event, window_id, } if window_id == window.id() => match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                event: KeyEvent {
                           logical_key: Key::Named(NamedKey::Escape),
                           state: ElementState::Pressed,
                           ..
                       },
                ..
            } => eventloop_target_ref.exit(),
            _ => {}
        },
        _ => {}
    });
}
