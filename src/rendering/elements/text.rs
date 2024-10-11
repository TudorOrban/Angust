use skia_safe::{Canvas, Color, Point};

use crate::rendering::{layout::space_allocation_system::text::size_estimator::{determine_text_element_lines, estimate_text_element_size}, rendering_interface::element_renderer::ElementRenderer};

use super::{common_types::{OptionalSize, Position, Size}, element::{Element, ElementType, EventType}, element_id_generator::IDGenerator, styles::{Dimension, Styles, Unit, WhiteSpace}};


pub struct Text {
    _id: String,
    content: String,
    lines: Vec<String>,
    position: Position,
    size: Size,
    styles: Styles,
    natural_size: Size,
}

impl Text {
    pub fn new(content: String) -> Self {
        let id = IDGenerator::get();
        Self {
            _id: id,
            content: content.clone(),
            lines: vec![content],
            position: Position::default(),
            size: Size::default(),
            styles: Styles::default(),
            natural_size: Size::default(),
        }
    }

    pub fn set_styles(&mut self, styles: Styles) -> &mut Self {
        self.styles = styles;
        self
    }

    pub fn set_content(&mut self, content: String) -> &mut Self {
        self.content = content;
        self
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}

impl Element for Text {
    fn render(&self, canvas: &Canvas) {
        ElementRenderer::render_multi_line_text(
            canvas, 
            self.get_position(), 
            self.lines.clone(),
            self.get_styles().text_color.unwrap_or(Color::BLACK),
            self.get_styles().font_size.unwrap_or(Dimension { value: 16.0, unit: Unit::Px }).value,
            self.get_styles().font_weight.unwrap_or_default(),
            self.get_styles().font_family.unwrap_or_default(),
            self.get_styles().font_style.unwrap_or_default(),
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

    fn set_requested_size(&mut self, _: OptionalSize) {}

    fn get_natural_size(&self) -> Size {
        self.natural_size
    }

    fn get_requested_size(&self) -> OptionalSize { OptionalSize::default() }

    fn get_effective_size(&self) -> Size { self.get_natural_size() }

    fn is_text_wrapper(&self) -> bool { false }

    fn estimate_sizes(&mut self) {
        let estimated_text_size = estimate_text_element_size(self);
        self.set_natural_size(estimated_text_size);
    }

    fn allocate_space(&mut self, allocated_position: Position, allocated_size: Size) {
        let line_height = self.get_styles().font_size.unwrap_or(Dimension { value: 16.0, unit: Unit::Px }).value;
        
        self.position = {
            let x = allocated_position.x;
            let y = allocated_position.y + line_height;
            Position { x, y }
        };
        self.size = allocated_size;
        
        if self.get_styles().white_space.unwrap_or_default() == WhiteSpace::Normal {
            self.lines = determine_text_element_lines(self);
        } else {
            self.lines = vec![self.content.clone()];
            println!("Lines: {:?}", self.lines);
        }
    }
}