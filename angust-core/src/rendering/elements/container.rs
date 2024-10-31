use skia_safe::{Canvas, Color, Point};

use crate::{application::event_handling::scrollbar_movement_handler::handle_scrollbar_movement, rendering::{
    layout::{
        effective_size_estimator, size_estimator, space_allocation_system::container::container_space_allocator
    },
    rendering_interface::element_renderer::ElementRenderer,
}};

use super::{
    common_types::{OptionalSize, Position, ScrollbarState, Size}, component::{component::ComponentInterface, state::reflectivity::ReflectiveState}, element::{Element, ElementType, EventType}, element_id_generator::ElementIDGenerator, event_propagator, styles::{Directions, Styles}
};

pub struct Container {
    _id: String,
    position: Position,
    size: Size,
    natural_size: Size,
    requested_size: OptionalSize,
    styles: Styles,
    pub children: Vec<Box<dyn Element>>,
    pub scrollbar_state: ScrollbarState,
}

impl Container {
    pub fn new() -> Self {
        let id = ElementIDGenerator::get();
        Self {
            _id: id,
            position: Position::default(),
            size: Size::default(),
            natural_size: Size::default(),
            requested_size: OptionalSize::default(),
            styles: Styles::default(),
            children: Vec::new(),
            scrollbar_state: ScrollbarState::default(),
        }
    }

    pub fn add_children(&mut self, children: Vec<Box<dyn Element>>) -> &mut Self {
        self.children.extend(children);
        self
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

        if self.scrollbar_state.is_overflowing.horizontal && self.scrollbar_state.thumb_scrollbar_width_ratio < 1.0 {
            ElementRenderer::render_scrollbar(
                canvas,
                Position {
                    x: self.position.x,
                    y: self.position.y + self.size.height - 10.0,
                },
                Size {
                    width: self.size.width,
                    height: 10.0,
                },
                Directions {
                    horizontal: true,
                    vertical: false,
                },
                self.scrollbar_state.current_scroll_position.x,
                self.scrollbar_state.thumb_scrollbar_width_ratio
            );
        }
    }

    fn update(&mut self) {
        for child in &mut self.children {
            child.update();
        }
    }

    fn handle_event(&mut self, cursor_position: Point, event_type: &EventType) {
        handle_scrollbar_movement(self, cursor_position, event_type);

        for child in &mut self.children {
            child.handle_event(cursor_position, event_type);
        }
    }

    fn propagate_event(&mut self, cursor_position: skia_safe::Point, event_type: &EventType) -> Vec<String> {
        event_propagator::propagate_event(self, cursor_position, event_type)
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

    fn set_styles(&mut self, styles: Styles) {
        self.styles = styles;
    }

    fn add_child(&mut self, child: Box<dyn Element>) {
        self.children.push(child);
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

    fn get_children(&self) -> Option<&Vec<Box<dyn Element>>> {
        Some(&self.children)
    }
    
    fn get_component_interface(&mut self) -> Option<&mut dyn ComponentInterface> {
        None
    }

    fn get_state(&self) -> Option<&dyn ReflectiveState> {
        None
    }

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

    fn get_requested_size(&self) -> OptionalSize {
        self.requested_size.clone()
    }

    fn get_effective_size(&self) -> Size {
        effective_size_estimator::estimate_effective_size(&self.get_requested_size(), &self.get_natural_size())
    }

    fn is_text_wrapper(&self) -> bool {
        self.children.len() == 1 && self.children[0].get_element_type() == ElementType::Text
    }

    // First pass: Traverse the DOM from leaves to root and estimate the size of each container.
    fn estimate_sizes(&mut self) {

        if !self.children.is_empty() {
            for child in &mut self.children {
                child.estimate_sizes();
            }

            size_estimator::estimate_parent_container_sizes(self);
        } else {
            size_estimator::estimate_leaf_container_sizes(self);
        }
    }

    // Second pass: Traverse the DOM from root to leaves and allocate space to each container.
    fn allocate_space(&mut self, allocated_position: Position, allocated_size: Size) {
        self.position = allocated_position;
        self.size = allocated_size;

        if self.is_text_wrapper() {
            self.children[0].allocate_space(allocated_position, allocated_size);
            return;
        }

        container_space_allocator::allocate_space_to_children(self, allocated_position, allocated_size);
    }
    
    fn layout(&mut self, allocated_position: Position, allocated_size: Size) {
        self.estimate_sizes();
        self.allocate_space(allocated_position, allocated_size);
    }

    // Reactivity
    fn react_to_state_change(&mut self, component_id: String) {
        for child in &mut self.children {
            child.react_to_state_change(component_id.clone());
        }
    }
}
