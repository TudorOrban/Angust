
use regex::Regex;

use crate::{
    parsing::html::{error::ParsingError, html_parser::ParsingContext}, 
    rendering::elements::component::state::{nested_reflectivity::access_field, reactivity::ReactiveState}
};


// State placeholders {{ component_state_property }}
pub fn parse_state_placeholder<State: ReactiveState>(
    text: &str,
    state: &State,
    context: &mut ParsingContext<State>,
) -> Result<String, ParsingError> {
    let re = Regex::new(r"\{\{(\s*[^}]+\s*)\}\}").unwrap();
    let mut result = String::new();
    let mut last_end = 0;

    for cap in re.captures_iter(text) {
        let property_access_path = match cap.get(1) {
            Some(m) => m.as_str().trim(),
            None => continue,
        };

        // Append the text before the current placeholder
        result.push_str(&text[last_end..cap.get(0).unwrap().start()]);

        // Append the substituted value
        let property_state = substitute_state_placeholder(property_access_path, state, context)?;
        result.push_str(&property_state);

        last_end = cap.get(0).unwrap().end();
    }

    result.push_str(&text[last_end..]);

    Ok(result)
}

fn substitute_state_placeholder<State: ReactiveState>(
    property_access_path: &str,
    state: &State,
    context: &mut ParsingContext<State>,
) -> Result<String, ParsingError> {
    let property = access_field(state, property_access_path, &context.for_loop_contexts)?;
    if let Some(val) = property.as_any().downcast_ref::<String>() {
        return Ok(val.clone());
    } else if let Some(val) = property.as_any().downcast_ref::<f64>() {
        return Ok(val.to_string());
    } else if let Some(val) = property.as_any().downcast_ref::<usize>() {
        return Ok(val.to_string());
    } else {
        return Err(ParsingError::FieldAccessError(format!("Property '{}' is not a string", property_access_path)));
    }
}
