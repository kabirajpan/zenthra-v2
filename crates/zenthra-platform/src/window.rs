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
        decorations: bool,
        transparent: bool,
        blur: bool,
        app_id: Option<&str>,
        icon: Option<winit::window::Icon>,
    ) -> Self {
        #[allow(unused_mut)]
        let mut attrs = winit::window::Window::default_attributes()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .with_decorations(decorations)
            .with_transparent(transparent)
            .with_blur(blur);

        #[cfg(target_os = "linux")]
        {
            use winit::platform::wayland::WindowAttributesExtWayland;
            use winit::platform::x11::WindowAttributesExtX11;
            let id = app_id.unwrap_or("zenthree");
            attrs = WindowAttributesExtWayland::with_name(attrs, id, id);
            attrs = WindowAttributesExtX11::with_name(attrs, id, id);
        }

        if let Some(ref icon) = icon {
            attrs = attrs.with_window_icon(Some(icon.clone()));
        }

        let winit_window = Arc::new(event_loop.create_window(attrs).unwrap());
        if blur {
            winit_window.set_blur(true);
        }
        if let Some(icon) = icon {
            winit_window.set_window_icon(Some(icon));
        }
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

    pub fn set_blur(&self, blur: bool) {
        self.winit_window.set_blur(blur);
    }
}
