use crate::graphics::{Vertex, Instance};

pub struct Object {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    instances: Vec<Instance>,
    num_indices: u32,
    instance_buffer_size: usize,
}

impl Object {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex], indices: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(vertices),
            wgpu::BufferUsage::VERTEX,
        );
        let index_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(indices),
            wgpu::BufferUsage::INDEX,
        );
        let num_indices = indices.len() as u32;

        let instance_buffer = device.create_buffer_with_data(
            &[],
            wgpu::BufferUsage::STORAGE_READ,
        );

        Self {
            vertex_buffer,
            index_buffer,
            instance_buffer,
            instances: Vec::new(),
            num_indices,
            instance_buffer_size: 0,
        }
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }

    pub fn instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }

    pub fn instance_buffer_size(&self) -> usize {
        self.instance_buffer_size
    }

    pub fn num_instances(&self) -> usize {
        self.instances.len()
    }

    pub fn add_instance(&mut self, device: &wgpu::Device, position: cgmath::Vector3<f32>, rotation: cgmath::Quaternion<f32>) {
        self.instances.push(Instance{ position, rotation });

        let instance_data = self.instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        self.instance_buffer_size = instance_data.len() * std::mem::size_of::<cgmath::Matrix4<f32>>();

        self.instance_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&instance_data),
            wgpu::BufferUsage::STORAGE_READ,
        );

    }
}
