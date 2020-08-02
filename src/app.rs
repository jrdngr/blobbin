pub mod state;

use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{EventLoop, ControlFlow};

pub use state::AppState;
use crate::world::World;

pub struct App {
    world: World,
}

impl App {
    pub fn new(world: World) -> Self {
        Self { world }
    }

    pub fn run(mut self) -> ! {
        let event_loop = EventLoop::new();
        let mut state = AppState::new(self.world.width as u32, self.world.height as u32, &event_loop).unwrap();

        event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
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
                self.world.update();
                state.window.request_redraw();
            }
        });
    }
}
