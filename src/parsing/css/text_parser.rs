use crate::rendering::elements::styles::Styles;

pub fn update_text_style(styles: &mut Styles, key: &str, value: &str) {
    match key {
        // "font-size" => styles.font_size = parse_dimension(value),  // Assuming font_size uses Dimension
        // "font-weight" => styles.font_weight = parse_font_weight(value),
        // "text-align" => styles.text_align = parse_text_align(value),
        _ => println!("Unhandled text property: {}", key),
    }
}