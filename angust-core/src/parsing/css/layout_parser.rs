use crate::rendering::elements::styles::{AlignContent, AlignItems, Dimension, DisplayType, FlexDirection, FlexWrap, JustifyContent, Margin, Overflow, Padding, Spacing, Styles};

use super::dimension_parser::parse_dimension;


pub fn update_layout_style(styles: &mut Styles, key: &str, value: &str) {
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