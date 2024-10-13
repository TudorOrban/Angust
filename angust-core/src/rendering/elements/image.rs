use image::DynamicImage;
use skia_safe::{Canvas, Point};

use crate::{application::resource_loader::image_loader, rendering::rendering_interface::element_renderer::ElementRenderer};

use super::{common_types::{OptionalSize, Position, Size}, element::{Element, ElementType, EventType}, element_id_generator::IDGenerator, styles::Styles};


pub struct Image {
    _id: String,
    image_path: String,
    image: Option<DynamicImage>,
    position: Position,
    size: Size,
    styles: Styles,
    natural_size: Size,
    requested_size: OptionalSize,
}

impl Image {
    pub fn new(image_directory_relative_path: String, image_relative_path: String, styles: Option<Styles>) -> Self {
        let id = IDGenerator::get();
        let image = image_loader::load_image(image_directory_relative_path, image_relative_path.clone())
            .map_or(None, |image| Some(image));

        Self {
            _id: id,
            image_path: image_relative_path,
            image: image,
            position: Position::default(),
            size: Size::default(),
            styles: styles.unwrap_or_default(),
            natural_size: Size::default(),
            requested_size: OptionalSize::default(),
        }
    }

    pub fn set_styles(&mut self, styles: Styles) -> &mut Self {
        self.styles = styles;
        self
    }

    pub fn set_image_path(&mut self, image_path: String) -> &mut Self {
        self.image_path = image_path;
        self
    }

    pub fn get_image_path(&self) -> String {
        self.image_path.clone()
    }
}

impl Element for Image {
    fn render(&self, canvas: &Canvas) {
        if let Some(image) = &self.image {
            ElementRenderer::render_image(
                image,
                canvas, 
                self.get_position(), 
                self.get_size(), 
            );
        }
    }

    fn update(&mut self) {}

    fn handle_event(&mut self, _: Point, _: &EventType) {}

    fn set_id(&mut self, id: String) {
        self._id = id;
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn add_child(&mut self, _: Box<dyn Element>) {}

    fn get_id(&self) -> String {
        self._id.clone()
    }

    fn get_element_type(&self) -> ElementType {
        ElementType::Image
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn get_styles(&self) -> Styles {
        self.styles
    }

    fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn Element>>> { None }

    // Layout system
    fn set_natural_size(&mut self, size: Size) {
        self.natural_size = size;
    }

    fn set_requested_size(&mut self, requested_size: OptionalSize) {
        self.requested_size = requested_size;
    }

    fn get_natural_size(&self) -> Size {
        self.natural_size
    }

    fn get_requested_size(&self) -> OptionalSize { self.requested_size }

    fn get_effective_size(&self) -> Size {
        let effective_width = if let Some(width) = self.get_requested_size().width {
            width.value
        } else {
            self.get_natural_size().width
        };
        let effective_height = if let Some(height) = self.get_requested_size().height {
            height.value
        } else {
            self.get_natural_size().height
        };

        Size {
            width: effective_width,
            height: effective_height,
        }   
    }

    fn is_text_wrapper(&self) -> bool { false }

    fn estimate_sizes(&mut self) {
        let estimated_image_size = self.image.as_ref()
            .map_or(Size::default(), |image| Size {
                width: image.width() as f32,
                height: image.height() as f32,
            });
        self.set_natural_size(estimated_image_size);

        let sizing_policy = self.get_styles().sizing_policy.unwrap_or_default();
        self.set_requested_size(OptionalSize { width: sizing_policy.width, height: sizing_policy.height });
    }

    fn allocate_space(&mut self, allocated_position: Position, allocated_size: Size) {
        self.position = allocated_position;
        self.size = allocated_size;
    }
    
    fn layout(&mut self, allocated_position: Position, allocated_size: Size) {
        self.estimate_sizes();
        self.allocate_space(allocated_position, allocated_size);
    }
}
