use skia_safe::Color;

use super::common_types::Size;


#[derive(Clone, Copy)]
pub struct Styles {
    pub display: Option<DisplayType>,
    pub flex_direction: Option<FlexDirection>,
    pub flex_wrap: Option<FlexWrap>,
    pub justify_content: Option<JustifyContent>,
    pub align_items: Option<AlignItems>,
    pub align_content: Option<AlignContent>,
    pub overflow: Option<Overflow>,
    pub size: Option<SizingPolicy>,
    pub margin: Option<Margin>,
    pub padding: Option<Padding>,
    pub spacing: Option<Spacing>,
    pub background_color: Option<Color>,
    pub text_color: Option<Color>,
    pub border: Option<Border>,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            display: Some(DisplayType::default()),
            flex_direction: Some(FlexDirection::default()),
            flex_wrap: Some(FlexWrap::default()),
            justify_content: Some(JustifyContent::default()),
            align_items: Some(AlignItems::default()),
            align_content: Some(AlignContent::default()),
            overflow: Some(Overflow::default()),
            size: Some(SizingPolicy::default()),
            margin: Some(Margin::default()),
            padding: Some(Padding::default()),
            spacing: Some(Spacing::default()),
            background_color: Some(Color::TRANSPARENT),
            text_color: Some(Color::BLACK),
            border: Some(Border::default()),
        }
    }
}

// Layout properties
#[derive(Clone, Copy)]
pub enum DisplayType {
    Block,          // The element takes up the full width of its container, starting on a new line.
    InlineBlock,    // The element does not start on a new line but can have width and height set.
    Flex,           // The element behaves as a flexible container that arranges its children dynamically.
}

impl Default for DisplayType {
    fn default() -> Self {
        Self::Block
    }
}

#[derive(Clone, Copy)]
pub enum FlexDirection {
    Row,            // Lay out children horizontally.
    Column,         // Lay out children vertically.
}

impl Default for FlexDirection {
    fn default() -> Self {
        Self::Row
    }
}

#[derive(Clone, Copy)]
pub enum FlexWrap {
    NoWrap,         // All children are laid out in a single line.
    Wrap,           // Children wrap around to additional lines as needed.
    WrapReverse,    // Children wrap around to additional lines in reverse order.
}

impl Default for FlexWrap {
    fn default() -> Self {
        Self::NoWrap
    }
}

#[derive(Clone, Copy)]
pub enum JustifyContent {
    FlexStart,      // Items are aligned at the start of the container.
    FlexEnd,        // Items are aligned at the end of the container.
    Center,         // Items are centered within the container.
    SpaceBetween,   // Items are evenly distributed; the first item is at the start, the last at the end.
    SpaceAround,    // Items are evenly distributed with equal space around each item.
}

impl Default for JustifyContent {
    fn default() -> Self {
        Self::FlexStart
    }
}

#[derive(Clone, Copy)]
pub enum AlignItems {
    FlexStart,      // Items are aligned at the start of the cross axis.
    FlexEnd,        // Items are aligned at the end of the cross axis.
    Center,         // Items are centered along the cross axis.
    Stretch,        // Items stretch to fill the container along the cross axis.
    Baseline,       // Items are aligned such as their baselines align.
}

impl Default for AlignItems {
    fn default() -> Self {
        Self::Stretch
    }
}

#[derive(Clone, Copy)]
pub enum AlignContent {
    FlexStart,      // Lines are packed at the start of the container.
    FlexEnd,        // Lines are packed at the end of the container.
    Center,         // Lines are centered in the container.
    SpaceBetween,   // Lines display evenly spaced between the start and end.
    SpaceAround,    // Lines display with equal spacing around them.
    Stretch,        // Lines stretch to take up the remaining space.
}

impl Default for AlignContent {
    fn default() -> Self {
        Self::Stretch
    }
}

#[derive(Clone, Copy)]
pub enum Overflow {
    Visible,        // Content is not clipped and may be rendered outside the container.
    Hidden,         // Content is clipped, and no scrollbars are provided.
    Scroll,         // Content is clipped and scrollbars are added only when necessary.
    Auto,           // Behavior depends on the user agent (similar to Scroll, but may show scrollbars differently).
}


impl Default for Overflow {
    fn default() -> Self {
        Self::Visible
    }
}

// Dimension properties
#[derive(Clone, Copy)]
pub struct SizingPolicy {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub mode: Option<SizeMode>,
}

impl Default for SizingPolicy {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            mode: Some(SizeMode::default()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SizeMode {
    Auto,              // Element sizes itself based on its content.
    Exact(Size),   // Use the specified width and height in pixels.
    FillParent,        // Expand to fill the available space, respecting max constraints if provided.
    FitParentWidth,    // Expand to fill the available width, respecting max constraints if provided.
    FitParentHeight,   // Expand to fill the available height, respecting max constraints if provided.
    Percent(f32),      // Use a percentage of the available space, e.g., width: 50%.
}

impl Default for SizeMode {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Clone, Copy)]
pub struct Spacing {
    pub spacing_x: f32,
    pub spacing_y: f32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            spacing_x: 0.0,
            spacing_y: 0.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Margin {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for Margin {
    fn default() -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for Padding {
    fn default() -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }
}

// Appearance properties
#[derive(Clone, Copy)]
pub struct Border {
    pub width: f32,
    pub color: Color,
    pub radius: BorderRadius,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            width: 0.0,
            color: Color::TRANSPARENT,
            radius: BorderRadius::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self {
            top_left: 0.0,
            top_right: 0.0,
            bottom_right: 0.0,
            bottom_left: 0.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Directions {
    pub horizontal: bool,
    pub vertical: bool,
}

impl Default for Directions {
    fn default() -> Self {
        Self {
            horizontal: true,
            vertical: true,
        }
    }
}