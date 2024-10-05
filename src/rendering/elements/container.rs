use skia_safe::{Canvas, Color, Point};

use crate::rendering::rendering_interface::element_renderer::ElementRenderer;

use super::{common_types::{Position, Size}, element::{Element, ElementType, EventType}, element_id_generator::IDGenerator, styles::Styles};


pub struct Container {
    _id: String,
    position: Position,
    size: Size,
    styles: Styles,
    children: Vec<Box<dyn Element>>,
}

impl Container {
    pub fn new() -> Self {
        let id = IDGenerator::get();
        Self {
            _id: id,
            position: Position::default(),
            size: Size::default(),
            styles: Styles::default(),
            children: Vec::new(),
        }
    }
}

impl Element for Container {
    fn render(&self, canvas: &Canvas) {
        ElementRenderer::render_element(
            canvas,
            self.position,
            self.size,
            self.styles.background_color.unwrap_or(Color::TRANSPARENT),
            self.styles.border.unwrap_or_default().width,
            self.styles.border.unwrap_or_default().color,
        );
    
        for child in &self.children {
            child.render(canvas);
        }
    }

    fn update(&mut self) {
        for child in &mut self.children {
            child.update();
        }
    }
    
    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType) {
        for child in &mut self.children {
            child.handle_event(cursor_position, event_type);
        }
    }

    fn set_id(&mut self, id: String) {
        self._id = id;
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

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

    fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn Element>>> {
        Some(&mut self.children)
    }
    
    fn compute_allocation_plan(&mut self) {

    }

    fn enact_allocation_plan(&mut self, allocated_position: Position, allocated_size: Size) {

    }
}