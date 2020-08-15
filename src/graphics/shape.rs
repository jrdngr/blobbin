use crate::graphics::Vertex;

#[derive(Clone, Debug)]
pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Shape {
    pub fn triangle(color: [f32; 3]) -> Self {
        let vertices = vec![
            Vertex::new_2d(0.0, 0.5, color),
            Vertex::new_2d(-0.5, -0.5, color),
            Vertex::new_2d(0.5, -0.5, color),
        ];

        let indices = vec![0, 1, 2];

        Self {
            vertices,
            indices,
        }
    }

    pub fn square(color: [f32; 3]) -> Self {
        let vertices = vec![
            Vertex::new_2d(-1.0, -1.0, color),
            Vertex::new_2d(1.0, -1.0, color),
            Vertex::new_2d(1.0, 1.0, color),
            Vertex::new_2d(-1.0, 1.0, color),
        ];
        
        #[rustfmt::skip]
        let indices = vec![
            0, 1, 3,
            3, 1, 2,
        ];

        Self {
            vertices,
            indices,
        }
    }

    pub fn circle(color: [f32; 3], resolution: u16) -> Self {
        use std::f32::consts::PI;

        // Any less than 3 and we can't really draw anything useful
        let resolution = resolution.max(3);

        let mut vertices = Vec::with_capacity(resolution as usize + 1);
        vertices.push(Vertex::new_2d(0.0, 0.0, color));

        let mut indices = Vec::with_capacity(resolution as usize * 3);

        let delta = (2.0 * PI) / resolution as f32;

        for i in 0..resolution {
            let theta = i as f32 * delta;
            let x = theta.cos();
            let y = theta.sin();
            vertices.push(Vertex::new_2d(x, y, color));

            indices.push(0);
            indices.push(i + 1);

            let next_index = (i + 2) % (resolution + 1); 
            indices.push(next_index);
        }

        Self {
            vertices,
            indices,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let circle = Shape::circle([1.0, 1.0, 1.0], 10);

        // Worst unit test ever
        dbg!(circle);
        assert!(false);
    }
}
