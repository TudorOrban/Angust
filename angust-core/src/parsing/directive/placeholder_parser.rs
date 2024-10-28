
use regex::Regex;

use crate::{
    parsing::html::html_parser::ParsingContext, rendering::elements::component::component_state::{
        access_field, ReactiveState 
    }
};

// State placeholders {{ component_state_property }}
pub fn parse_state_placeholder<State: ReactiveState>(
    text: &str,
    state: &State,
    context: &mut ParsingContext<State>,
) -> Result<String, String> {
    let re = Regex::new(r"\{\{(\s*[^}]+\s*)\}\}").unwrap();
    let mut result = text.to_string();

    for cap in re.captures_iter(text) {
        let property_access_path = match cap.get(1) {
            Some(m) => m.as_str().trim(),
            None => continue,
        };

        result = substitute_state_placeholder(property_access_path, state, context)?;
    }

    Ok(result)
}

pub fn substitute_state_placeholder<State: ReactiveState>(
    property_access_path: &str,
    state: &State,
    context: &mut ParsingContext<State>,
) -> Result<String, String> {
    let property = access_field(state, property_access_path, context)?;
    if let Some(val) = property.as_any().downcast_ref::<String>() {
        return Ok(val.clone());
    } else {
        return Err(format!("Property '{}' is not a string", property_access_path));
    }
}
