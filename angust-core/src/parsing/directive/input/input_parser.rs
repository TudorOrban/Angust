use std::collections::HashMap;

use regex::Regex;


pub fn parse_input_attributes(
    attributes: &kuchiki::Attributes,
) -> HashMap<String, String> {
    let mut inputs = HashMap::new();
    let re = Regex::new(r"\[\s*(\w+)\s*\]").unwrap(); // Regex to capture [property]
    
    for (key, value) in attributes.map.iter() {
        let key_local = key.local.clone().to_string();
        let optional_capture = re.captures(&key_local);
        if optional_capture.is_none() {
            continue;
        }

        let captures = optional_capture.unwrap().get(1);
        if captures.is_none() {
            continue;
        }

        let property_name = captures.unwrap().as_str();
        let bound_value = value.value.to_string();

        inputs.insert(property_name.to_string(), bound_value);
    }

    inputs
}