use crate::rendering::elements::styles::{Dimension, SizingPolicy, Styles, Unit};


pub fn update_dimension_style(styles: &mut Styles, key: &str, value: &str) {
    let dimension = parse_dimension(value);
    if styles.sizing_policy.is_none() {
        styles.sizing_policy = Some(SizingPolicy::default());
    }
    let sizing_policy = styles.sizing_policy.as_mut().unwrap();

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

pub fn parse_dimension(value: &str) -> Option<Dimension> {
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
