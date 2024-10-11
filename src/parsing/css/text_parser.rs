use crate::rendering::elements::styles::{FontFamily, FontStyle, FontWeight, Styles, WhiteSpace};

use super::dimension_parser::parse_dimension;

pub fn update_text_style(styles: &mut Styles, key: &str, value: &str) {
    match key {
        "white-space" => styles.white_space = parse_white_space(value),
        "font-size" => styles.font_size = parse_dimension(value),
        "font-weight" => styles.font_weight = parse_font_weight(value),
        "font-family" => styles.font_family = parse_font_family(value),
        "font-style" => styles.font_style = parse_font_style(value),
        // "text-align" => styles.text_align = parse_text_align(value),
        _ => println!("Unhandled text property: {}", key),
    }
}

fn parse_white_space(value: &str) -> Option<WhiteSpace> {
    match value {
        "normal" => Some(WhiteSpace::Normal),
        "nowrap" => Some(WhiteSpace::NoWrap),
        "pre" => Some(WhiteSpace::Pre),
        "pre-line" => Some(WhiteSpace::PreLine),
        "pre-wrap" => Some(WhiteSpace::PreWrap),
        _ => None,
    }
}

fn parse_font_weight(value: &str) -> Option<FontWeight> {
    match value {
        "lighter" => Some(FontWeight::FW300),
        "normal" => Some(FontWeight::FW400),
        "bold" => Some(FontWeight::FW600),
        "bolder" => Some(FontWeight::FW700),
        "100" => Some(FontWeight::FW100),
        "200" => Some(FontWeight::FW200),
        "300" => Some(FontWeight::FW300),
        "400" => Some(FontWeight::FW400),
        "500" => Some(FontWeight::FW500),
        "600" => Some(FontWeight::FW600),
        "700" => Some(FontWeight::FW700),
        "800" => Some(FontWeight::FW800),
        "900" => Some(FontWeight::FW900),
        _ => None,
    }
}

fn parse_font_family(value: &str) -> Option<FontFamily> {
    match value {
        "Arial" => Some(FontFamily::Arial),
        "Helvetica" => Some(FontFamily::Helvetica),
        "Times New Roman" => Some(FontFamily::TimesNewRoman),
        "Courier" => Some(FontFamily::Courier),
        "Verdana" => Some(FontFamily::Verdana),
        _ => None,
    }
}

fn parse_font_style(value: &str) -> Option<FontStyle> {
    match value {
        "normal" => Some(FontStyle::Normal),
        "italic" => Some(FontStyle::Italic),
        "oblique" => Some(FontStyle::Oblique),
        _ => None,
    }
}