use skia_safe::{gpu::gl::FramebufferInfo, Point};
use winit::{application::ApplicationHandler, event::{ElementState, KeyEvent, Modifiers, MouseButton, MouseScrollDelta, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}};
use gl_rs as gl;
use glutin::{config::GlConfig, display::GetGlDisplay, prelude::GlDisplay, surface::GlSurface};
use std::{ffi::CString, num::NonZeroU32};

use crate::{rendering::{elements::element::EventType, renderer::Renderer}, window::WindowingSystem};

use super::{app_configuration::AngustConfiguration, resource_loader::configuration_loader::load_angust_configuration};


pub struct Application<State> {
    pub state: State,

    pub angust_config: AngustConfiguration,

    windowing_system: WindowingSystem,
    fb_info: FramebufferInfo,
    event_loop: Option<EventLoop<()>>,
    modifiers: Modifiers,

    mouse_position: Option<Point>,
    is_mouse_pressed: bool,
    renderer: Renderer,
}

impl<State> Application<State> {
    pub fn new(initial_state: State, app_title: String) -> Self {
        let event_loop = EventLoop::new()
            .expect("Failed to create event loop");
        let mut windowing_system = WindowingSystem::new(&event_loop, app_title);

        gl::load_with(|s| {
            windowing_system
                .gl_config
                .display()
                .get_proc_address(CString::new(s).unwrap().as_c_str())
        });
    
        let fb_info = {
            let mut fboid: i32 = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };
    
            skia_safe::gpu::gl::FramebufferInfo {
                fboid: fboid.try_into().expect("Failed to get framebuffer ID"),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
                ..Default::default()
            }
        };

        let renderer = Renderer::new(
            &windowing_system.window, 
            &mut windowing_system.gr_context, 
            fb_info, 
            windowing_system.gl_config.num_samples() as usize, 
            windowing_system.gl_config.stencil_size() as usize
        );

        let angust_config = load_angust_configuration();

        Self {
            state: initial_state,
            angust_config,
            windowing_system,
            fb_info,
            event_loop: Some(event_loop),
            modifiers: Modifiers::default(),
            mouse_position: None,
            is_mouse_pressed: false,
            renderer,
        }
    }

    pub fn run(&mut self) {
        if let Some(event_loop) = self.event_loop.take() {  // Take the event loop, leaving None
            event_loop.run_app(self).expect("Failed to run the application");
        } else {
            panic!("Event loop already consumed or not initialized");
        }
    }
}

impl<State> ApplicationHandler for Application<State> {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        self.windowing_system.window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            // Window handling events
            WindowEvent::CloseRequested => {
                event_loop.exit();
                return;
            },
            WindowEvent::Resized(physical_size) => {
                let (width, height): (u32, u32) = physical_size.into();
                self.windowing_system.gl_surface.resize(
                    &self.windowing_system.gl_context,
                    NonZeroU32::new(width.max(1)).unwrap(),
                    NonZeroU32::new(height.max(1)).unwrap(),
                );
            
                self.renderer.resize_surface(&self.windowing_system.window, &mut self.windowing_system.gr_context, self.fb_info, self.windowing_system.gl_config.num_samples() as usize, self.windowing_system.gl_config.stencil_size() as usize);
                self.windowing_system.window.request_redraw();
            },
            WindowEvent::ModifiersChanged(new_modifiers) => {
                self.modifiers = new_modifiers;
            },
            WindowEvent::RedrawRequested => {
                // Render and flush the Skia context
                self.renderer.render_frame(&mut self.windowing_system.gr_context);
                self.windowing_system.gr_context.flush_and_submit();

                // Swap buffers to show the rendered content
                self.windowing_system
                    .gl_surface
                    .swap_buffers(&self.windowing_system.gl_context)
                    .expect("Failed to swap buffers");
            }

            // Mouse and keyboard events
            WindowEvent::MouseInput { state, button, .. } => {
                match (state, button) {
                    (ElementState::Pressed, MouseButton::Left) => {
                        self.is_mouse_pressed = true;
                        if let Some(mouse_position) = self.mouse_position {
                            self.renderer.handle_event(mouse_position, EventType::MouseDown);
                            self.windowing_system.window.request_redraw();
                        }
                    },
                    (ElementState::Released, MouseButton::Left) => {
                        self.is_mouse_pressed = false;
                        if let Some(mouse_position) = self.mouse_position {
                            self.renderer.handle_event(mouse_position, EventType::MouseUp);
                            self.windowing_system.window.request_redraw();
                        }
                    },
                    _ => ()
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = Some(Point::new(position.x as f32, position.y as f32));

                if self.is_mouse_pressed {
                    if let Some(mouse_position) = self.mouse_position {
                        self.renderer.handle_event(mouse_position, EventType::MouseDrag);
                        self.windowing_system.window.request_redraw();
                    }
                }
            },
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll_delta = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y * 5.0,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                };
            
                if let Some(mouse_position) = self.mouse_position {
                    self.renderer.handle_event(mouse_position, EventType::MouseRoll(scroll_delta));
                    self.windowing_system.window.request_redraw();
                }
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent { logical_key, .. },
                ..
            } => {
                if self.modifiers.state().super_key() && logical_key == "q" {
                    event_loop.exit();
                }
                self.windowing_system.window.request_redraw();
            }
            _ => (),
        }

        event_loop.set_control_flow(ControlFlow::Wait);
    }
}