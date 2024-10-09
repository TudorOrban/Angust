use std::ops::Add;

use super::styles::{Dimension, Directions};


#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }
}

impl Add for Size {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

pub struct Space {
    pub horizontal: f32,
    pub vertical: f32,
}

impl Default for Space {
    fn default() -> Self {
        Self {
            horizontal: 0.0,
            vertical: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OptionalSize {
    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
}

impl Default for OptionalSize {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
        }
    }
}

pub struct ScrollbarState {
    pub thumb_scrollbar_width_ratio: f32,
    pub is_overflowing: Directions,
    pub current_scroll_position: Position,
    pub is_dragging: bool,
    pub drag_start_position: Position,
    pub drag_start_scroll_position: Position,
}

impl Default for ScrollbarState {
    fn default() -> Self {
        Self {
            thumb_scrollbar_width_ratio: 1.0,
            is_overflowing: Directions {
                horizontal: false,
                vertical: false,
            },
            current_scroll_position: Position::default(),
            is_dragging: false,
            drag_start_position: Position::default(),
            drag_start_scroll_position: Position::default(),
        }
    }
}