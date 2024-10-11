use skia_safe::{
    gpu::{self, gl::FramebufferInfo, SurfaceOrigin},
    ColorType, Surface,
};
use winit::window::Window;
use skia_safe::gpu::DirectContext;

use super::{elements::{common_types::{Position, Size}, element::{Element, EventType}}, ui_manager::UIManager};

pub struct Renderer {
    pub ui_body: Box<dyn Element>,
    pub surface: Surface,
    ui_manager: UIManager,
}

impl Renderer {
    pub fn new(window: &Window, gr_context: &mut DirectContext, fb_info: FramebufferInfo, sample_count: usize, stencil_bits: usize) -> Self {
        let surface = Self::create_surface(
            window,
            fb_info,
            gr_context,
            sample_count,
            stencil_bits,
        );

        let screen_size = window.inner_size();
        ui_body.estimate_sizes(); // Start backwards recursion to estimate element sizes
        ui_body.allocate_space( // Start forwards recursion to allocate space
            Position { x: 0.0, y: 0.0 },
            Size { width: screen_size.width as f32, height: screen_size.height as f32 
        });

        Self { 
            ui_body,
            surface,
            ui_manager: UIManager::new(ui_body),
        }
    }

    pub fn render_frame(&mut self, _gr_context: &mut DirectContext) {
        let canvas = self.surface.canvas();
        canvas.clear(skia_safe::Color::WHITE);

        self.ui_manager.render(canvas);
    }

    pub fn handle_event(&mut self, cursor_position: skia_safe::Point, event_type: EventType) {
        self.ui_manager.handle_event(cursor_position, &event_type);
    }
    
    fn create_surface(
        window: &Window,
        fb_info: FramebufferInfo,
        gr_context: &mut DirectContext,
        sample_count: usize,
        stencil_bits: usize,
    ) -> Surface {
        Renderer::create_or_resize_surface(window, gr_context, fb_info, sample_count, stencil_bits)
    }

    pub fn resize_surface(&mut self, window: &Window, gr_context: &mut DirectContext, fb_info: FramebufferInfo, sample_count: usize, stencil_bits: usize) {
        self.surface = Renderer::create_or_resize_surface(window, gr_context, fb_info, sample_count, stencil_bits);
    }

    fn create_or_resize_surface(
        window: &Window,
        gr_context: &mut DirectContext,
        fb_info: FramebufferInfo,
        sample_count: usize,
        stencil_bits: usize,
    ) -> Surface {
        let size = window.inner_size();
        let size = (
            size.width.try_into().expect("Could not convert width"),
            size.height.try_into().expect("Could not convert height"),
        );
        let backend_render_target = gpu::backend_render_targets::make_gl(
            size, 
            sample_count, 
            stencil_bits, 
            fb_info
        );

        gpu::surfaces::wrap_backend_render_target(
            gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        ).expect("Failed to create or resize Skia surface")
    }
}