use skia_safe::Point;

use crate::rendering::rendering_interface::element_renderer::ElementRenderer;

use super::{common_types::{OptionalSize, Position, Size}, element::{Element, ElementType, EventType}, element_id_generator::IDGenerator, styles::{Dimension, Directions, Styles, Unit}};


pub struct Scrollbar {
    _id: String,
    position: Position,
    size: Size,
    requested_size: OptionalSize,
    directions: Directions,
}

impl Scrollbar {
    pub fn new(directions: Directions) -> Self {
        let id = IDGenerator::get();
        Self {
            _id: id,
            position: Position::default(),
            size: Size::default(),
            directions: directions,
            requested_size: get_scrollbar_requested_size(directions),
            // natural_size: Size::default(),
            // requested_size: OptionalSize::default(),
            // styles: Styles::default(),
            // children: Vec::new(),
        }
    }
}

impl Element for Scrollbar {
    fn render(&self, canvas: &skia_safe::Canvas) {
        ElementRenderer::render_scrollbar(
            canvas,
            self.position,
            self.size,
        );
    }

    fn update(&mut self) {
        
    }

    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType) {
        
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

    fn add_child(&mut self, child: Box<dyn Element>) {}

    fn get_id(&self) -> String {
        self._id.clone()
    }

    fn get_element_type(&self) -> ElementType {
        ElementType::Scrollbar
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn get_styles(&self) -> Styles { return Styles::default(); }

    fn get_children_mut(&mut self) -> Option<&mut Vec<Box<dyn Element>>> { None }

    // Layout system
    fn set_natural_size(&mut self, size: Size) {}

    fn set_requested_size(&mut self, requested_size: OptionalSize) {
        self.requested_size = requested_size;
    }

    fn get_natural_size(&self) -> Size { self.size }

    fn get_requested_size(&self) -> OptionalSize {
        self.requested_size
    }

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

    // Traverse the DOM from leaves to root and estimate the size of each container.
    fn estimate_sizes(&mut self) {
        self.requested_size = get_scrollbar_requested_size(self.directions);
    }

    // Traverse the DOM from root to leaves and allocate space to each container.
    fn allocate_space(&mut self, allocated_position: Position, allocated_size: Size) {
        self.position = allocated_position;
        self.size = allocated_size;
    }
}

fn get_scrollbar_requested_size(directions: Directions) -> OptionalSize {
    let default_scrollbar_thickness = 10.0;
    let scrollbar_main_axis_dimension = Dimension {
        value: 100.0,
        unit: Unit::Percent,
    };
    let scrollbar_cross_axis_dimension = Dimension {
        value: default_scrollbar_thickness,
        unit: Unit::Px,
    };

    if directions.horizontal {
        return OptionalSize {
            width: Some(scrollbar_main_axis_dimension),
            height: Some(scrollbar_cross_axis_dimension),
        }
    } else {
        return OptionalSize {
            width: Some(scrollbar_cross_axis_dimension),
            height: Some(scrollbar_main_axis_dimension),
        }
    }
}