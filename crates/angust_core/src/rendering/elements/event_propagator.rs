use super::{button::EventPropagationData, element::{Element, EventType}};


pub fn propagate_event(element: &mut dyn Element, cursor_position: skia_safe::Point, event_type: &EventType) -> Vec<EventPropagationData> {
    let mut event_targets = Vec::new();

    for child in element.get_children_mut().unwrap_or(&mut vec![]) {
        let child_event_targets = child.propagate_event(cursor_position, event_type);
        event_targets.extend(child_event_targets);
    }

    event_targets
}