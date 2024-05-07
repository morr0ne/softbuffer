use std::num::NonZeroU32;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};

include!("utils/winit_app.rs");

const BUFFER_WIDTH: usize = 256;
const BUFFER_HEIGHT: usize = 128;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let app = winit_app::WinitAppBuilder::with_init(|elwt| {
        let window = winit_app::make_window(elwt, |w| w);

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        (window, surface)
    })
    .with_event_handler(|state, event, elwt| {
        let (window, surface) = state;
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                surface
                    .resize(
                        NonZeroU32::new(BUFFER_WIDTH as u32).unwrap(),
                        NonZeroU32::new(BUFFER_HEIGHT as u32).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                for y in 0..BUFFER_HEIGHT {
                    for x in 0..BUFFER_WIDTH {
                        let red = x as u32 % 255;
                        let green = y as u32 % 255;
                        let blue = (x as u32 * y as u32) % 255;

                        let color = blue | (green << 8) | (red << 16);
                        buffer[y * BUFFER_WIDTH + x] = color;
                    }
                }
                buffer.present().unwrap();
            }
            Event::WindowEvent {
                event:
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    },
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }
            _ => {}
        }
    });

    winit_app::run_app(event_loop, app);
}
