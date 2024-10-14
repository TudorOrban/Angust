use crate::rendering::elements::{container::Container, element::Element, styles::Unit};


pub fn resolve_surplus(
    container: &mut Container,
    deficit: &mut f32,
) {
    // let percentage_width_children: Vec<Box<dyn Element>> = container.children.into_iter()
    //     .filter(|child| child.get_styles().sizing_policy.unwrap_or_default().width.unwrap_or_default().unit == Unit::Percent)
    //     .collect();
}