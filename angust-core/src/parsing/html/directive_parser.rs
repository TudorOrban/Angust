use std::{collections::HashMap, sync::Arc};

use super::html_parser::ParsingContext;


pub fn parse_directives<State>(
    attributes: &kuchiki::Attributes,
    context: &ParsingContext<State>
) -> HashMap<String, Arc<dyn FnMut(&mut State) + 'static>> {
    let mut handlers = HashMap::new();
    if let Some(on_click_value) = attributes.get("@onclick") {
        if let Some(handler) = context.current_component_event_handlers.and_then(|h| h.get(on_click_value)) {
            // Clone the Arc containing the handler
            handlers.insert("click".to_string(), Arc::clone(handler));
        }
    }
    handlers
}