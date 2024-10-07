use skia_safe::Color;

use crate::rendering::elements::styles::{Border, Styles};

use super::dimension_parser::parse_dimension;


pub fn update_appearance_style(styles: &mut Styles, key: &str, value: &str) {
    match key {
        "background-color" => styles.background_color = parse_color(value),
        "color" => styles.text_color = parse_color(value),
        "border-width" => styles.border = Some(Border {
            width: parse_dimension(value).unwrap_or_default(),
            color: styles.border.unwrap_or_default().color,
            radius: styles.border.unwrap_or_default().radius,
        }),
        "border-color" => styles.border = Some(Border {
            width: styles.border.unwrap_or_default().width,
            color: parse_color(value).unwrap_or(Color::BLACK),
            radius: styles.border.unwrap_or_default().radius,
        }),
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