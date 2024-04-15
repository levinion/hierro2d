use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

use crate::{state::State, Application};

pub async fn run(app: impl Application) {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut window = WindowBuilder::new().build(&event_loop).unwrap();
    app.window(&mut window);
    let mut state = State::new(window, app).await;

    event_loop
        .run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    logical_key: Key::Named(NamedKey::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                            state.resize(PhysicalSize::new(
                                (state.size().width as f64 * scale_factor) as u32,
                                (state.size().height as f64 * scale_factor) as u32,
                            ));
                        }
                        WindowEvent::RedrawRequested => {
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    state.resize(state.size())
                                }
                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    control_flow.exit();
                                }
                                Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        })
        .unwrap();
}
