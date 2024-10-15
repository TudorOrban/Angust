use super::html_parser::ParsingContext;


pub fn parse_directives<State>(
    attributes: &kuchiki::Attributes,
    context: &ParsingContext
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