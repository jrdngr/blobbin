use crate::{math::{self, Vector2f}, blob::Blob};

pub struct World {
    pub width: usize,
    pub height: usize,
    pub blob_size: f64,
    pub repel_force: f64,
    pub repel_distance: f64,
    pub friction_force: f64,
    pub blobs: Vec<Blob>
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new(width: usize, height: usize, blob_size: f64) -> Self {

        Self {
            width,
            height,
            blob_size,
            repel_force: 0.0,
            repel_distance: 0.0,
            friction_force: 0.0,
            blobs: Vec::new(),
        }
    }

    pub fn blob_size(&mut self, size: f64) -> &mut Self {
        self.blob_size = size;
        self
    }

    pub fn repel_force(&mut self, force: f64) -> &mut Self {
        self.repel_force = force;
        self
    }

    pub fn repel_distance(&mut self, distance: f64) -> &mut Self {
        self.repel_distance = distance;
        self
    }

    pub fn friction_force(&mut self, force: f64) -> &mut Self {
        self.friction_force = force;
        self
    }

    pub fn add_blob(&mut self, x: f64, y: f64) {
        let id = self.blobs.len();
        self.blobs.push(Blob::new(id, self.blob_size, (x, y)));
    }

    pub fn add_random_blob(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0, self.width) as f64;
        let y =  rng.gen_range(0, self.height) as f64;
        
        self.add_blob(x, y);
    }

    pub fn add_random_blobs(&mut self, count: usize) {
        for _ in 0..count {
            self.add_random_blob();
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub fn update(&mut self) {
        for index in 0..self.blobs.len() {
            let blob = &self.blobs[index];
            let mut blob_forces: Vec<Vector2f> = Vec::new();
            
            for other in &self.blobs {
                if blob.id != other.id {
                    let force_vector = blob.position.vector_to(&other.position);
                    if force_vector.magnitude() <= self.repel_distance {
                        blob_forces.push(self.repel_force * force_vector.normalized());
                    }    
                }
            }

            let blob = &mut self.blobs[index];

            for force in blob_forces {
                blob.acceleration += force;
            }

            if blob.position.x <= 0.0 || blob.position.x + self.blob_size > self.width as f64 {
                blob.velocity.x *= -1.0;
            }
            
            if blob.position.y <= 0.0 || blob.position.y + self.blob_size > self.height as f64 {
                blob.velocity.y *= -1.0;
            }

            blob.acceleration /= self.friction_force;

            blob.velocity.x += blob.acceleration.x;
            blob.velocity.y += blob.acceleration.y;

            blob.position.x += blob.velocity.x;
            blob.position.y += blob.velocity.y;

            blob.position.x = math::clamp(blob.position.x, 0.0, self.width as f64);
            blob.position.y = math::clamp(blob.position.y, 0.0, self.height as f64);
        }
    }

    fn pixel_color(&self, index: usize) -> [u8; 4] {
        let position = index_to_position(index, self.width, self.height);
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
    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&self.pixel_color(i));
        }
    }
}

#[inline]
fn index_to_position(index: usize, width: usize, height: usize) -> Vector2f {
    let x = index % width;
    let y = index / height;
    
    (x as f64, y as f64).into()
}

#[inline]
fn _position_to_index(position: impl Into<Vector2f>, height: u32) -> usize {
    let position = position.into();

    let index = position.y as u32 * height + position.x as u32;

    index as usize
}
