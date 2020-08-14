#[derive(Clone, Debug)]
pub struct GraphicsConfig {
    pub clear_color: wgpu::Color,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            clear_color: wgpu::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }
        }
    }
}
