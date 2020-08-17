use crate::graphics::{Color, Vertex, Object};

#[derive(Clone, Debug)]
pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Shape {
    pub fn to_object(&self, device: &wgpu::Device) -> Object {
        Object::new(device, &self.vertices, &self.indices)
    }
}

pub fn triangle(color: Color) -> Shape {
    let vertices = vec![
        Vertex::new_2d(0.0, 0.5, color),
        Vertex::new_2d(-0.5, -0.5, color),
        Vertex::new_2d(0.5, -0.5, color),
    ];

    let indices = vec![0, 1, 2];

    Shape { vertices, indices }
}

pub fn square(color: Color) -> Shape {
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

    Shape { vertices, indices }
}

pub fn circle(color: Color, resolution: u16) -> Shape {
    use std::f32::consts::PI;

    // Any less than 3 and we can't really draw anything useful
    let resolution = resolution.max(3);

    let mut vertices = Vec::with_capacity(resolution as usize + 1);
    let mut indices = Vec::with_capacity(resolution as usize * 3);

    let delta = (2.0 * PI) / resolution as f32;

    for i in 0..resolution {
        let theta = i as f32 * delta;
        let x = theta.cos();
        let y = theta.sin();
        vertices.push(Vertex::new_2d(x, y, color));

        indices.push(i);
        indices.push((i + 1) % resolution);
        indices.push(resolution);
    }

    vertices.push(Vertex::new_2d(0.0, 0.0, color));

    Shape { vertices, indices }
}
