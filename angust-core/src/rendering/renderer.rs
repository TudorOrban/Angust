use skia_safe::{
    gpu::{self, gl::FramebufferInfo, SurfaceOrigin},
    ColorType, Surface,
};
use winit::window::Window;
use skia_safe::gpu::DirectContext;

use super::{elements::{button::EventPropagationData, common_types::{Position, Size}, element::{Element, EventType}}, ui_manager::UIManager};

pub struct Renderer {
    pub surface: Surface,
    pub screen_size: Size,
    ui_manager: UIManager,
}

impl Renderer {
    pub fn new(window: &Window, gr_context: &mut DirectContext, fb_info: FramebufferInfo, sample_count: usize, stencil_bits: usize, ui_body: Box<dyn Element>) -> Self {
        let surface = Self::create_surface(
            window,
            fb_info,
            gr_context,
            sample_count,
            stencil_bits,
        );
        let screen_size = window.inner_size();

        Self { 
            surface,
            screen_size: Size { width: screen_size.width as f32, height: screen_size.height as f32 },
            ui_manager: UIManager::new(ui_body),
        }
    }

    pub fn layout(self: &mut Self) {
        self.ui_manager.layout(
            Position { x: 0.0, y: 0.0 },
            self.screen_size
        );
    }

    pub fn render_frame(&mut self, _gr_context: &mut DirectContext) {
        let canvas = self.surface.canvas();
        canvas.clear(skia_safe::Color::TRANSPARENT);

        self.ui_manager.render(canvas);
    }

    pub fn handle_event(&mut self, cursor_position: skia_safe::Point, event_type: EventType) {
        self.ui_manager.handle_event(cursor_position, &event_type);
    }

    pub fn propagate_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) -> Vec<EventPropagationData> {
        self.ui_manager.propagate_event(cursor_position, event_type)
    }

    pub fn react_to_state_change(&mut self, component_id: String) {
        self.ui_manager.react_to_state_change(component_id);
    }

    pub fn handle_route_change(&mut self, route: &String, component_name: &String) {
        self.ui_manager.handle_route_change(route, component_name);
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