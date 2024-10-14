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
    let value = value.trim();
    let unit_start = value.find(|c: char| !c.is_digit(10) && c != '.').unwrap_or(value.len());
    let (numeric_part, unit_part) = value.split_at(unit_start);
    let unit_part = unit_part.trim();

    if let (Ok(val), Some(unit)) = (numeric_part.parse::<f32>(), parse_unit(unit_part)) {
        Some(Dimension { value: val, unit })
    } else {
        None
    }
}

fn parse_unit(value: &str) -> Option<Unit> {
    match value {
        "px" => Some(Unit::Px),
        "vh" => Some(Unit::Vh),
        "vw" => Some(Unit::Vw),
        "rem" => Some(Unit::Rem),
        "%" => Some(Unit::Percent),
        _ => None,
    }
}
