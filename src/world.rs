use crate::{math::{self, Vector2f}, blob::Blob};

const MAX_ACCELERATION: f64 = 10000.0;

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
    pub fn update(&mut self, delta_time: f64) {
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
                blob.acceleration += force * delta_time;
            }

            if blob.position.x <= 0.0 || blob.position.x + self.blob_size > self.width as f64 {
                blob.velocity.x *= -1.0;
            }
            
            if blob.position.y <= 0.0 || blob.position.y + self.blob_size > self.height as f64 {
                blob.velocity.y *= -1.0;
            }

            blob.acceleration /= self.friction_force * delta_time;
            if blob.acceleration.magnitude() > MAX_ACCELERATION {
                blob.acceleration = blob.acceleration.with_magnitude(MAX_ACCELERATION);
            }

            blob.velocity.x += blob.acceleration.x * delta_time;
            blob.velocity.y += blob.acceleration.y * delta_time;

            blob.position.x += blob.velocity.x * delta_time;
            blob.position.y += blob.velocity.y * delta_time;

            blob.position.x = math::clamp(blob.position.x, 0.0, self.width as f64);
            blob.position.y = math::clamp(blob.position.y, 0.0, self.height as f64);
        }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: [`wgpu::TextureFormat::Rgba8UnormSrgb`]
    pub fn draw(&self, frame: &mut [u8]) {
        for item in frame.iter_mut() {
            *item = 0;
        }

        for blob in &self.blobs {
            let start = self.position_to_index(&blob.position);
            let end = start + 4;

            if end <= frame.len() {
                let pixels = &mut frame[start..end];
                pixels.copy_from_slice(&[255, 255, 255, 255]);    
            }
        }
    }

    #[inline]
    fn position_to_index(&self, position: &Vector2f) -> usize {
        let index = position.y as usize * self.height + position.x as usize;

        // 4 bytes per pixel color
        4 * index
    }
}
