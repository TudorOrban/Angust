use image::DynamicImage;
use skia_safe::{Canvas, Point};

use crate::application::resource_loader::image_loader;

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
    pub fn new(image_directory_relative_path: String, image_relative_path: String) -> Self {
        let id = IDGenerator::get();
        let image = image_loader::load_image(image_directory_relative_path, image_relative_path.clone())
            .map_or(None, |image| Some(image));

        if image.is_none() {
            println!("Failed to load image: {}", image_relative_path);
        } else {
            println!("Successfully loaded image: {}", image_relative_path);
        }
        
        Self {
            _id: id,
            image_path: image_relative_path,
            image: image,
            position: Position::default(),
            size: Size::default(),
            styles: Styles::default(),
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
            // ElementRenderer::render_image(
            //     canvas, 
            //     self.get_position(), 
            //     self.get_size(), 
            //     image,
            // );
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
        ElementType::Text
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

    fn get_requested_size(&self) -> OptionalSize { OptionalSize::default() }

    fn get_effective_size(&self) -> Size { self.get_natural_size() }

    fn is_text_wrapper(&self) -> bool { false }

    fn estimate_sizes(&mut self) {
        let estimated_image_size = Size::default();
        self.set_natural_size(estimated_image_size);
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