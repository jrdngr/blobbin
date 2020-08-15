use winit::{
    event::{WindowEvent, VirtualKeyCode, ElementState, KeyboardInput},
};

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let target = (self.eye.x, self.eye.y, self.eye.z - 1.0).into();
        let view = cgmath::Matrix4::look_at(self.eye, target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);


pub struct CameraController {
    speed: f32,
    x_axis: f32,
    y_axis: f32,
    z_axis: f32,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            x_axis: 0.0,
            y_axis: 0.0,
            z_axis: 0.0,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.x_axis = if is_pressed { -1.0 } else { 0.0 };
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.x_axis = if is_pressed { 1.0 } else { 0.0 };
                        true
                    }
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.y_axis = if is_pressed { 1.0 } else { 0.0 };
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.y_axis = if is_pressed { -1.0 } else { 0.0 };
                        true
                    }
                    VirtualKeyCode::R | VirtualKeyCode::E => {
                        self.z_axis = if is_pressed { -1.0 } else { 0.0 };
                        true
                    }
                    VirtualKeyCode::F | VirtualKeyCode::Q => {
                        self.z_axis = if is_pressed { 1.0 } else { 0.0 };
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        camera.eye.x += self.x_axis * self.speed;
        camera.eye.y += self.y_axis * self.speed;
        camera.eye.z += self.z_axis * self.speed;
    }
}
