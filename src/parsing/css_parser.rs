use kuchiki::Attributes;
use skia_safe::Color;

use crate::rendering::elements::styles::{AlignContent, AlignItems, Dimension, DisplayType, FlexDirection, FlexWrap, JustifyContent, Margin, Overflow, Padding, SizingPolicy, Spacing, Styles, Unit};

pub fn parse_styles(attributes: &Attributes) -> Styles {
    let mut styles = Styles::default();

    if let Some(class_name) = attributes.get("class") {
        styles = apply_class_styles(class_name);
    }

    if let Some(style_attr) = attributes.get("style") {
        styles = parse_inline_styles(style_attr);
    }

    styles
}

fn apply_class_styles(class_name: &str) -> Styles {
    println!("Applying class styles for class: {}", class_name);
    Styles::default()
}

fn parse_inline_styles(style_str: &str) -> Styles {
    let mut styles = Styles::default();

    // Split the style string by semicolons to get "key: value" pairs
    style_str.split(';').filter_map(|item| {
        let parts: Vec<&str> = item.splitn(2, ':').map(str::trim).collect();
        if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }
    }).for_each(|(key, value)| {
        dispatch_by_key_and_update_style(&mut styles, key, value);
    });

    styles
}

static LAYOUT_PROPERTIES: [&str; 8] = ["display", "flex-direction", "flex-wrap", "justify-content", "align-items", "margin", "padding", "spacing"];
static DIMENSION_PROPERTIES: [&str; 6] = ["width", "height", "min-width", "max-width", "min-height", "max-height"];
static COLOR_PROPERTIES: [&str; 2] = ["background-color", "color"];
static TEXT_PROPERTIES: [&str; 3] = ["font-size", "font-weight", "text-align"];

fn dispatch_by_key_and_update_style(styles: &mut Styles, key: &str, value: &str) {
    if LAYOUT_PROPERTIES.contains(&key) {
        update_layout_style(styles, key, value);
    } else if DIMENSION_PROPERTIES.contains(&key) {
        update_dimension_style(styles, key, value);
    } else if COLOR_PROPERTIES.contains(&key) {
        update_color_style(styles, key, value);
    } else if TEXT_PROPERTIES.contains(&key) {
        update_text_style(styles, key, value);
    } else {
        println!("Unknown style key: {}", key);
    }
}

// Layout properties
fn update_layout_style(styles: &mut Styles, key: &str, value: &str) {
    match key {
        "display" => styles.display = parse_display(value),
        "flex-direction" => styles.flex_direction = parse_flex_direction(value),
        "flex-wrap" => styles.flex_wrap = parse_flex_wrap(value),
        "justify-content" => styles.justify_content = parse_justify_content(value),
        "align-items" => styles.align_items = parse_align_items(value),
        "align-content" => styles.align_content = parse_align_content(value),
        "overflow" => styles.overflow = parse_overflow(value),
        "margin" => styles.margin = parse_margin(value),
        "padding" => styles.padding = parse_padding(value),
        "spacing" => styles.spacing = parse_spacing(value),
        _ => println!("Unhandled layout property: {}", key),
    }
}

fn parse_display(value: &str) -> Option<DisplayType> {
    match value {
        "block" => Some(DisplayType::Block),
        "inline-block" => Some(DisplayType::InlineBlock),
        "flex" => Some(DisplayType::Flex),
        _ => None,
    }
}

fn parse_flex_direction(value: &str) -> Option<FlexDirection> {
    match value {
        "row" => Some(FlexDirection::Row),
        "column" => Some(FlexDirection::Column),
        _ => None,
    }
}

fn parse_flex_wrap(value: &str) -> Option<FlexWrap> {
    match value {
        "nowrap" => Some(FlexWrap::NoWrap),
        "wrap" => Some(FlexWrap::Wrap),
        "wrap-reverse" => Some(FlexWrap::WrapReverse),
        _ => None,
    }
}

fn parse_justify_content(value: &str) -> Option<JustifyContent> {
    match value {
        "flex-start" => Some(JustifyContent::FlexStart),
        "flex-end" => Some(JustifyContent::FlexEnd),
        "center" => Some(JustifyContent::Center),
        "space-between" => Some(JustifyContent::SpaceBetween),
        "space-around" => Some(JustifyContent::SpaceAround),
        _ => None,
    }
}

fn parse_align_items(value: &str) -> Option<AlignItems> {
    match value {
        "flex-start" => Some(AlignItems::FlexStart),
        "flex-end" => Some(AlignItems::FlexEnd),
        "center" => Some(AlignItems::Center),
        "baseline" => Some(AlignItems::Baseline),
        "stretch" => Some(AlignItems::Stretch),
        _ => None,
    }
}

fn parse_align_content(value: &str) -> Option<AlignContent> {
    match value {
        "flex-start" => Some(AlignContent::FlexStart),
        "flex-end" => Some(AlignContent::FlexEnd),
        "center" => Some(AlignContent::Center),
        "space-between" => Some(AlignContent::SpaceBetween),
        "space-around" => Some(AlignContent::SpaceAround),
        "stretch" => Some(AlignContent::Stretch),
        _ => None,
    }
}

fn parse_overflow(value: &str) -> Option<Overflow> {
    match value {
        "visible" => Some(Overflow::Visible),
        "hidden" => Some(Overflow::Hidden),
        "scroll" => Some(Overflow::Scroll),
        "auto" => Some(Overflow::Auto),
        _ => None,
    }
}

fn parse_margin(value: &str) -> Option<Margin> {
    let parts: Vec<&str> = value.split_whitespace().collect();
    let dimensions = parts.iter().map(
        |value| parse_dimension(value)
    ).collect::<Option<Vec<Dimension>>>()?;

    Some(Margin {
        top: dimensions.get(0).cloned().unwrap_or_default(),
        right: dimensions.get(1).cloned().unwrap_or_else(|| dimensions.get(0).cloned().unwrap_or_default()),
        bottom: dimensions.get(2).cloned().unwrap_or_else(|| dimensions.get(0).cloned().unwrap_or_default()),
        left: dimensions.get(3).cloned().unwrap_or_else(|| dimensions.get(1).cloned().unwrap_or_else(|| dimensions.get(0).cloned().unwrap_or_default())),
    })
}

fn parse_padding(value: &str) -> Option<Padding> {
    parse_margin(value).map(|margin| Padding {
        top: margin.top,
        right: margin.right,
        bottom: margin.bottom,
        left: margin.left,
    })
}

fn parse_spacing(value: &str) -> Option<Spacing> {
    let parts: Vec<&str> = value.split_whitespace().collect();
    if parts.len() == 2 {
        let x_dim = parse_dimension(parts[0])?;
        let y_dim = parse_dimension(parts[1])?;
        Some(Spacing { spacing_x: x_dim, spacing_y: y_dim })
    } else {
        None
    }
}

// Dimension properties
fn update_dimension_style(styles: &mut Styles, key: &str, value: &str) {
    let dimension = parse_dimension(value);
    if styles.sizing_policy.is_none() {
        styles.sizing_policy = Some(SizingPolicy::default());
    }
    let sizing_policy = styles.sizing_policy.as_mut().unwrap();  // It's now safe to unwrap
    match key {
        "width" => sizing_policy.width = dimension,
        "height" => sizing_policy.height = dimension,
        "min-width" => sizing_policy.min_width = dimension,
        "max-width" => sizing_policy.max_width = dimension,
        "min-height" => sizing_policy.min_height = dimension,
        "max-height" => sizing_policy.max_height = dimension,
        _ => {}
    }
}

fn parse_dimension(value: &str) -> Option<Dimension> {
    let numeric_part = value[..value.len() - 2].trim();  // Slice off the last two characters for the unit
    if let Ok(val) = numeric_part.parse::<f32>() {
        let unit = parse_unit(&value[numeric_part.len()..].trim())?;
        Some(Dimension { value: val, unit })
    } else {
        None
    }
}

fn parse_unit(value: &str) -> Option<Unit> {
    if value.ends_with("px") {
        Some(Unit::Px)
    } else if value.ends_with("vh") {
        Some(Unit::Vh)
    } else if value.ends_with("vw") {
        Some(Unit::Vw)
    } else if value.ends_with("rem") {
        Some(Unit::Rem)
    } else if value.ends_with("%") {
        Some(Unit::Percent)
    } else {
        None
    }
}

// Color properties
fn update_color_style(styles: &mut Styles, key: &str, value: &str) {
    match key {
        "background-color" => styles.background_color = parse_color(value),
        "color" => styles.text_color = parse_color(value),
        _ => println!("Unhandled color property: {}", key),
    }
}

fn parse_color(value: &str) -> Option<Color> {
    // Handle rgb() and rgba() formats
    let trimmed_value = value.trim();
    if trimmed_value.starts_with("rgb(") && trimmed_value.ends_with(")") {
        parse_rgb_color(trimmed_value)
    } else if trimmed_value.starts_with("rgba(") && trimmed_value.ends_with(")") {
        parse_rgba_color(trimmed_value)
    } else {
        None
    }
}

fn parse_rgb_color(value: &str) -> Option<Color> {
    // Expecting input like "rgb(255, 0, 0)"
    value.trim_matches(|c| c == 'r' || c == 'g' || c == 'b' || c == '(' || c == ')')
        .split(',')
        .map(str::trim)
        .map(|num| num.parse::<u8>().ok())
        .collect::<Option<Vec<u8>>>()
        .and_then(|rgb| {
            if rgb.len() == 3 {
                Some(Color::from_rgb(rgb[0], rgb[1], rgb[2]))
            } else {
                None
            }
        })
}

fn parse_rgba_color(value: &str) -> Option<Color> {
    // Expecting input like "rgba(255, 0, 0, 0.5)"
    let parts: Vec<&str> = value.trim_matches(|c| c == 'r' || c == 'g' || c == 'b' || c == 'a' || c == '(' || c == ')')
        .split(',')
        .map(str::trim)
        .collect();
    
    if parts.len() == 4 {
        let r = parts[0].parse::<u8>().ok()?;
        let g = parts[1].parse::<u8>().ok()?;
        let b = parts[2].parse::<u8>().ok()?;
        let a = parts[3].parse::<f32>().ok()?;
        let alpha = (a * 255.0) as u8;
        Some(Color::from_argb(alpha, r, g, b))
    } else {
        None
    }
}

// Text properties
fn update_text_style(styles: &mut Styles, key: &str, value: &str) {
    match key {
        // "font-size" => styles.font_size = parse_dimension(value),  // Assuming font_size uses Dimension
        // "font-weight" => styles.font_weight = parse_font_weight(value),
        // "text-align" => styles.text_align = parse_text_align(value),
        _ => println!("Unhandled text property: {}", key),
    }
}