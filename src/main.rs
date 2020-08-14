// pub mod app;
pub mod common;
pub mod ecs;
pub mod graphics;

use futures::executor::block_on;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::graphics::{State, GraphicsOptions};

pub fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;

    let mut state = block_on(State::new(&window, GraphicsOptions::default()))?;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        _ => {}
                    },
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(_) => {
            state.update();
            state.render();
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}

// use app::{App, World};
// use common::config::Config;

// const WIDTH: u32 = 500;
// const HEIGHT: u32 = 500;

// const BLOB_COUNT: usize = 10;

// fn old_main() -> anyhow::Result<()> {
//     env_logger::init();

//     let config = Config::load_default_config_file()?;

//     let mut world = World {
//         width: WIDTH as usize,
//         height: HEIGHT as usize,
//         config,
//         blobs: Vec::new(),
//     };
//     world.add_random_blobs(BLOB_COUNT);

//     let app = App::new(world);

//     app.run()
// }
