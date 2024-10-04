// src/window/mod.rs

use glutin::{
    config::{ConfigTemplateBuilder, Config, GlConfig}, // Import GlConfig to bring trait methods into scope
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    prelude::NotCurrentGlContext,
    surface::{Surface as GlutinSurface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use skia_safe::gpu::DirectContext;
use std::num::NonZeroU32;
use winit::dpi::LogicalSize;
use winit::window::{Window, WindowAttributes};
use winit::event_loop::EventLoop;

pub struct WindowingSystem {
    pub window: Window,
    pub gl_surface: GlutinSurface<WindowSurface>,
    pub gl_context: PossiblyCurrentContext,
    pub gl_config: Config,
    pub gr_context: DirectContext,
}

impl WindowingSystem {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window_attributes = WindowAttributes::default()
            .with_title("Cartan")
            .with_inner_size(LogicalSize::new(1200, 800));

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(true);

        let display_builder = DisplayBuilder::new().with_window_attributes(window_attributes.into());
        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() < accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.expect("Could not create window with OpenGL context");

        let raw_window_handle = window
            .raw_window_handle()
            .expect("Failed to retrieve RawWindowHandle");

        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(Some(raw_window_handle));

        let not_current_gl_context = unsafe {
            gl_config
                .display()
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_config
                        .display()
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
        };

        let (width, height): (u32, u32) = window.inner_size().into();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .expect("Could not create gl window surface")
        };

        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("Could not make GL context current when setting up skia renderer");

        let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            gl_config
                .display()
                .get_proc_address(std::ffi::CString::new(name).unwrap().as_c_str())
        })
        .expect("Could not create interface");

        let gr_context = skia_safe::gpu::direct_contexts::make_gl(interface, None)
            .expect("Could not create Skia DirectContext");

        Self {
            window,
            gl_surface,
            gl_context,
            gl_config,
            gr_context,
        }
    }
}
