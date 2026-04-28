use std::sync::Arc;
use zenthra_render::gpu::GpuContext;

pub struct Window {
    pub gpu: GpuContext,
    pub winit_window: Arc<winit::window::Window>,
    pub title: String,
}

impl Window {
    pub async fn new(
        event_loop: &winit::event_loop::ActiveEventLoop,
        title: &str,
        width: u32,
        height: u32,
    ) -> Self {
        let attrs = winit::window::Window::default_attributes()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height));

        let winit_window = Arc::new(event_loop.create_window(attrs).unwrap());
        let gpu = GpuContext::new(winit_window.clone()).await;

        Self {
            gpu,
            winit_window,
            title: title.to_string(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        let sf = self.winit_window.scale_factor();
        self.gpu.resize(new_size, sf);
    }

    pub fn request_redraw(&self) {
        self.winit_window.request_redraw();
    }

    pub fn scale_factor(&self) -> f64 {
        self.gpu.scale_factor
    }

    pub fn width(&self) -> u32 {
        self.gpu.size.width
    }
    pub fn height(&self) -> u32 {
        self.gpu.size.height
    }
}
