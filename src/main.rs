pub mod app_state;
pub mod blob;
pub mod math;
pub mod world;

use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::ControlFlow;

use app_state::AppState;
use world::World;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const BLOB_SIZE: f64 = 5.0;

const BLOB_COUNT: usize = 100;

const REPEL_FORCE: f64 = 0.0001;
const REPEL_DISTANCE: f64 = 20.0;
const FRICTION_FORCE: f64 = 0.025;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let (mut state, event_loop) = AppState::new(WIDTH, HEIGHT)?;
    
    let mut world = World::new(WIDTH as usize, HEIGHT as usize, BLOB_SIZE);
    
    world.repel_force(REPEL_FORCE)
         .repel_distance(REPEL_DISTANCE)
         .friction_force(FRICTION_FORCE)
         .add_random_blobs(BLOB_COUNT);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(state.pixels.get_frame());
            if state.pixels
                .render()
                .map_err(|e| log::error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if state.input.update(&event) {
            // Close events
            if state.input.key_pressed(VirtualKeyCode::Escape) || state.input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = state.input.window_resized() {
                state. pixels.resize(size.width, size.height);
            }

            if state.input.mouse_released(0) {
                if let Some((x, y)) = state.input.mouse() {
                    let x = (x / 2.0) as f64;
                    let y = (y / 2.0) as f64;
                    world.add_blob(x, y);
                }
            }

            // Update internal state and request a redraw
            world.update();
            state.window.request_redraw();
        }
    });
}

