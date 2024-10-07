use std::ops::Add;

use super::styles::Dimension;


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