use kuchiki::Attributes;

use crate::rendering::elements::styles::Styles;

use super::{appearance_parser::update_appearance_style, dimension_parser::update_dimension_style, layout_parser::update_layout_style, text_parser::update_text_style};

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
static APPEARANCE_PROPERTIES: [&str; 5] = ["background-color", "color", "border-width", "border-color", "border-radius"];
static TEXT_PROPERTIES: [&str; 3] = ["font-size", "font-weight", "text-align"];

fn dispatch_by_key_and_update_style(styles: &mut Styles, key: &str, value: &str) {
    if LAYOUT_PROPERTIES.contains(&key) {
        update_layout_style(styles, key, value);
    } else if DIMENSION_PROPERTIES.contains(&key) {
        update_dimension_style(styles, key, value);
    } else if APPEARANCE_PROPERTIES.contains(&key) {
        update_appearance_style(styles, key, value);
    } else if TEXT_PROPERTIES.contains(&key) {
        update_text_style(styles, key, value);
    } else {
        println!("Unknown style key: {}", key);
    }
}