use regex::Regex;

use crate::rendering::elements::component::component::ComponentState;

use super::html_parser::ParsingContext;


pub fn parse_on_click_attribute(
    attributes: &kuchiki::Attributes,
    _: &ParsingContext
) -> Option<String> {
    if let Some(on_click_value) = attributes.get("@onclick") {
        let handler = on_click_value.to_string();
        let handler = handler.trim_start_matches("handle_event('");
        let handler = handler.trim_end_matches("')");
        let handler = handler.to_string();
        return Some(handler);
    }
    None
}

pub fn parse_state_placeholder<State: ComponentState>(
    text: &str,
    state: &State,
) -> Result<String, String> {
    let re = Regex::new(r"\{\{(\s*[^}]+\s*)\}\}").unwrap();
    let mut result = text.to_string();

    for cap in re.captures_iter(text) {
        let matched_text = match cap.get(0) {
            Some(text) => text,
            None => continue,
        };

        let key = cap[1].trim();
        let property = match state.get_property(key) {
            Some(prop) => prop,
            None => return Err(format!("Property '{}' not found in state", key)),
        };

        let value = match property.downcast_ref::<String>() {
            Some(val) => val,
            None => return Err(format!("Property '{}' is not a String type", key)),
        };

        result = result.replace(matched_text.as_str(), value);
    }

    Ok(result)
}
