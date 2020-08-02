pub mod state;

use std::time::{Instant, Duration};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{EventLoop, ControlFlow};

pub use state::AppState;
use crate::world::World;

const FRAME_TIME: u64 = 1000 / 60;
const CONFIG_REFRESH_RATE: Duration = Duration::from_secs(5);

pub struct App {
    world: World,
    last_event: Instant,
    last_frame: Instant,
    last_config_refresh: Instant,
}

impl App {
    pub fn new(world: World) -> Self {
        Self {
            world,
            last_event: Instant::now(),
            last_frame: Instant::now(),
            last_config_refresh: Instant::now(),
        }
    }

    pub fn run(mut self) -> ! {
        let event_loop = EventLoop::new();
        let mut state = AppState::new(self.world.width as u32, self.world.height as u32, &event_loop).unwrap();
        self.last_frame = Instant::now();

        event_loop.run(move |event, _, control_flow| {
            let current_event = Instant::now();
            let delta_time = (current_event - self.last_event).as_secs_f64();
            self.last_event = current_event;

            if current_event - self.last_config_refresh >= CONFIG_REFRESH_RATE {
                let _ = self.world.refresh_config();
                self.last_config_refresh = current_event;
            }

            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                let current_frame = Instant::now();
                if current_frame - self.last_frame >= Duration::from_millis(FRAME_TIME) {
                    self.last_frame = current_frame;
                    self.world.draw(state.pixels.get_frame());
                    if state.pixels
                        .render()
                        .map_err(|e| log::error!("pixels.render() failed: {}", e))
                        .is_err()
                    {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
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
                        self.world.add_blob(x, y);
                    }
                }
    
                // Update internal state and request a redraw
                self.world.update(delta_time);
                state.window.request_redraw();
            }
        });
    }
}
