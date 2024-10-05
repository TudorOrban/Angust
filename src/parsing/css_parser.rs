use skia_safe::Color;

use crate::rendering::elements::styles::{AlignContent, AlignItems, DisplayType, FlexDirection, FlexWrap, JustifyContent, Margin, Overflow, Padding, Styles};

#[allow(dead_code)]
fn update_style_from_key_value(styles: &mut Styles, key: &str, value: &str) {
    match key {
        "display" => styles.display = parse_display(value),
        "flex-direction" => styles.flex_direction = parse_flex_direction(value),
        "flex-wrap" => styles.flex_wrap = parse_flex_wrap(value),
        "justify-content" => styles.justify_content = parse_justify_content(value),
        "align-items" => styles.align_items = parse_align_items(value),
        "align-content" => styles.align_content = parse_align_content(value),
        "overflow" => styles.overflow = parse_overflow(value),
        "background-color" => styles.background_color = parse_color(value),
        "color" => styles.text_color = parse_color(value),
        "margin" => styles.margin = parse_margin(value),
        "padding" => styles.padding = parse_padding(value),
        _ => {
            println!("Unknown style key: {}", key);
        }
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
    value.parse::<f32>().ok().map(|v| Margin { left: v, top: v, right: v, bottom: v })
}

fn parse_padding(value: &str) -> Option<Padding> {
    value.parse::<f32>().ok().map(|v| Padding { left: v, top: v, right: v, bottom: v })
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
