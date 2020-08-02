pub mod math;

use pixels::{wgpu::Surface, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use math::Vector2f;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const BLOB_SIZE: f64 = 5.0;

const BLOB_COUNT: usize = 100;

const REPEL_FORCE: f64 = 0.0001;
const REPEL_DISTANCE: f64 = 20.0;
const FRICTION_FORCE: f64 = 0.025;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, surface);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new(BLOB_COUNT);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| log::error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            if input.mouse_released(0) {
                if let Some((x, y)) = input.mouse() {
                    let x = (x / 2.0) as f64;
                    let y = (y / 2.0) as f64;
                    world.blobs.push(Blob::new(world.blobs.len(), (x, y)));
                }
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

#[derive(Default, Debug, Clone)]
pub struct Blob {
    pub id: usize,
    pub position: Vector2f,
    pub velocity: Vector2f,
    pub acceleration: Vector2f,
}

impl Blob {
    pub fn new(id: usize, position: impl Into<Vector2f>) -> Self {
        Self {
            id,
            position: position.into(),
            ..Default::default()
        }
    }

    pub fn contains_point(&self, point: &Vector2f) -> bool {
        point.x >= self.position.x 
            && point.x <= self.position.x + BLOB_SIZE
            && point.y >= self.position.y
            && point.y <= self.position.y + BLOB_SIZE
    }
}

pub struct World {
    pub blobs: Vec<Blob>
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new(blob_count: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut blobs = Vec::with_capacity(blob_count);

        for id in 0..blob_count {
            let position = (rng.gen_range(0, WIDTH) as f64, rng.gen_range(0, HEIGHT) as f64);
            blobs.push(Blob::new(id, position));
        }

        Self {
            blobs,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        for index in 0..self.blobs.len() {
            let blob = &self.blobs[index];
            let mut blob_forces: Vec<Vector2f> = Vec::new();
            
            for other in &self.blobs {
                if blob.id != other.id {
                    let force_vector = blob.position.vector_to(&other.position);
                    if force_vector.magnitude() <= REPEL_DISTANCE {
                        blob_forces.push(REPEL_FORCE * force_vector.normalized());
                    }    
                }
            }

            let blob = &mut self.blobs[index];

            for force in blob_forces {
                blob.acceleration += force;
            }

            if blob.position.x <= 0.0 || blob.position.x + BLOB_SIZE > WIDTH as f64 {
                blob.velocity.x *= -1.0;
            }
            
            if blob.position.y <= 0.0 || blob.position.y + BLOB_SIZE > HEIGHT as f64 {
                blob.velocity.y *= -1.0;
            }

            blob.acceleration /= FRICTION_FORCE;

            blob.velocity.x += blob.acceleration.x;
            blob.velocity.y += blob.acceleration.y;

            blob.position.x += blob.velocity.x;
            blob.position.y += blob.velocity.y;

            blob.position.x = math::clamp(blob.position.x, 0.0, WIDTH as f64);
            blob.position.y = math::clamp(blob.position.y, 0.0, HEIGHT as f64);
        }
    }

    fn pixel_color(&self, index: usize) -> [u8; 4] {
        let position = index_to_position(index);
        for blob in &self.blobs {
            if blob.contains_point(&position) {
                return [255, 255, 255, 255];
            }
        }

        [0, 0, 0, 0]
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&self.pixel_color(i));
        }
    }
}

#[inline]
fn index_to_position(index: usize) -> Vector2f {
    let x = index % WIDTH as usize;
    let y = index / HEIGHT as usize;
    
    (x as f64, y as f64).into()
}

#[inline]
fn _position_to_index(position: impl Into<Vector2f>) -> usize {
    let position = position.into();

    let index = position.y as u32 * HEIGHT + position.x as u32;

    index as usize
}
