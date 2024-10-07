use skia_safe::Color;


#[derive(Clone, Copy, Debug)]
pub struct Styles {
    pub display: Option<DisplayType>,
    pub flex_direction: Option<FlexDirection>,
    pub flex_wrap: Option<FlexWrap>,
    pub justify_content: Option<JustifyContent>,
    pub align_items: Option<AlignItems>,
    pub align_content: Option<AlignContent>,
    pub overflow: Option<Overflow>,
    pub sizing_policy: Option<SizingPolicy>,
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
            sizing_policy: Some(SizingPolicy::default()),
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
#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexDirection {
    Row,            // Lay out children horizontally.
    Column,         // Lay out children vertically.
}

impl Default for FlexDirection {
    fn default() -> Self {
        Self::Column
    }
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
pub struct SizingPolicy {
    pub width: Option<Dimension>,
    pub height: Option<Dimension>,
    pub min_width: Option<Dimension>,
    pub max_width: Option<Dimension>,
    pub min_height: Option<Dimension>,
    pub max_height: Option<Dimension>,
}

impl Default for SizingPolicy {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Dimension {
    pub value: f32,
    pub unit: Unit,
}

impl Default for Dimension {
    fn default() -> Self {
        Self {
            value: 0.0,
            unit: Unit::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Unit {
    Px,
    Vh,
    Vw,
    Rem,
    Percent,
}

impl Default for Unit {
    fn default() -> Self {
        Self::Px
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Spacing {
    pub spacing_x: Dimension,
    pub spacing_y: Dimension,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            spacing_x: Dimension::default(),
            spacing_y: Dimension::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Margin {
    pub top: Dimension,
    pub right: Dimension,
    pub bottom: Dimension,
    pub left: Dimension,
}

impl Default for Margin {
    fn default() -> Self {
        Self {
            top: Dimension::default(),
            right: Dimension::default(),
            bottom: Dimension::default(),
            left: Dimension::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Padding {
    pub top: Dimension,
    pub right: Dimension,
    pub bottom: Dimension,
    pub left: Dimension,
}

impl Default for Padding {
    fn default() -> Self {
        Self {
            top: Dimension::default(),
            right: Dimension::default(),
            bottom: Dimension::default(),
            left: Dimension::default(),
        }
    }
}

impl Padding {
    pub fn horizontal(&self) -> f32 {
        self.left.value + self.right.value
    }

    pub fn vertical(&self) -> f32 {
        self.top.value + self.bottom.value
    }
}

// Appearance properties
#[derive(Clone, Copy, Debug)]
pub struct Border {
    pub width: Dimension,
    pub color: Color,
    pub radius: BorderRadius,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            width: Dimension::default(),
            color: Color::BLACK,
            radius: BorderRadius::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BorderRadius {
    pub top_left: Dimension,
    pub top_right: Dimension,
    pub bottom_right: Dimension,
    pub bottom_left: Dimension,
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self {
            top_left: Dimension::default(),
            top_right: Dimension::default(),
            bottom_right: Dimension::default(),
            bottom_left: Dimension::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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