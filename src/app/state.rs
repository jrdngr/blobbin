use wgpu::Surface;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub struct AppState {
    pub window: Window,
    pub input: WinitInputHelper,
    pub pixels: Pixels,
}

impl AppState {
    pub fn new(width: u32, height: u32, event_loop: &EventLoopWindowTarget<()>) -> anyhow::Result<Self> {
        let input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new(width as f64, height as f64);
            WindowBuilder::new()
                .with_title("Blobbin")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };
    
        let pixels = {
            let window_size = window.inner_size();
            let surface = Surface::create(&window);
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, surface);
            Pixels::new(width, height, surface_texture)?
        };

        let state = Self {
            window,
            input,
            pixels,
        };

        Ok(state)
    }
}
