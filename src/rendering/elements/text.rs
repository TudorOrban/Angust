use skia_safe::{Canvas, Color, Point};

use crate::rendering::rendering_interface::element_renderer::ElementRenderer;

use super::{common_types::{OptionalSize, Position, Size}, element::{Element, ElementType, EventType}, styles::Styles};


pub struct Text {
    _id: String,
    content: String,
    position: Position,
    size: Size,
    styles: Styles,
    natural_size: Size,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self {
            _id: String::new(),
            content,
            position: Position::default(),
            size: Size::default(),
            styles: Styles::default(),
            natural_size: Size::default(),
        }
    }
}

impl Element for Text {
    fn render(&self, canvas: &Canvas) {
        ElementRenderer::render_text(
            canvas, 
            self.get_position(), 
            self.get_size(), 
            16.0, 
            Color::BLACK,
            self.content.clone(),
        );
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
        ElementType::Container
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

    fn set_requested_size(&mut self, _: OptionalSize) {}

    fn get_natural_size(&self) -> Size {
        self.natural_size
    }

    fn get_requested_size(&self) -> OptionalSize { OptionalSize::default() }

    fn get_effective_size(&self) -> Size { self.get_natural_size() }

    // Traverse the DOM from leaves to root and estimate the size of each container.
    fn estimate_sizes(&mut self) {
        self.set_natural_size(Size {
            width: 20.0 * self.content.len() as f32,
            height: 80.0,
        });
    }

    // Traverse the DOM from root to leaves and allocate space to each container.
    fn allocate_space(&mut self, allocated_position: Position, allocated_size: Size) {
        self.position = {
            let x = allocated_position.x;
            let y = allocated_position.y + 20.0;
            Position { x, y }
        };
        self.size = allocated_size;
    }
}