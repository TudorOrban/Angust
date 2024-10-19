use kuchiki::NodeRef;

use crate::{parsing::css::css_parser, rendering::elements::component::component_state::ComponentState};
use crate::rendering::elements::button::Button;
use crate::rendering::elements::component::component_factory_registry::create_component;
use crate::rendering::elements::container::Container;
use crate::rendering::elements::element::Element;
use crate::rendering::elements::image::Image;
use crate::rendering::elements::styles::Styles;

use super::{directive_parser, html_parser::{self, ParsingContext}};

pub fn dispatch_element_processing<State : ComponentState>(
    elem_data: &kuchiki::ElementData, 
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Option<Box<dyn Element>> {
    match elem_data.name.local.as_ref() {
        "div" => Some(process_div_element::<State>(elem_data, node, parent_styles, context)),
        "button" => Some(process_button_element::<State>(elem_data, node, parent_styles, context)),
        "img" => process_image_element::<State>(elem_data, node, parent_styles, context),
        component_name => process_custom_component::<State>(component_name, elem_data, node, parent_styles, context),
    }
}

fn process_div_element<State : ComponentState>(
    elem_data: &kuchiki::ElementData, 
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Box<dyn Element> {
    let mut container = Container::new();
    let attributes = elem_data.attributes.borrow();

    let should_add_to_dom = directive_parser::parse_if_expression(context, &attributes);
    if should_add_to_dom.is_err() {
        println!("Error parsing @if directive: {:?}", should_add_to_dom.err());
        return Box::new(container) // TODO: Report error
    }
    if !should_add_to_dom.unwrap() {
        return Box::new(container)
    }

    let for_loop_context = directive_parser::parse_for_expression(context, &attributes);
    if for_loop_context.is_err() {
        println!("Error parsing @for directive: {:?}", for_loop_context.err());
        return Box::new(container) // TODO: Report error
    } else {
        println!("For loop context: {:?}", for_loop_context.unwrap());
    }

    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);
    container.set_styles(styles);

    node.children()
        .filter_map(|child| html_parser::map_dom_to_elements::<State>(&child, Some(&styles), context))
        .for_each(|child_element| container.add_child(child_element));

    Box::new(container)
}

fn process_button_element<State : ComponentState>(
    elem_data: &kuchiki::ElementData, 
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Box<dyn Element> {
    let attributes = elem_data.attributes.borrow();
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);

    let on_click_handler_name = directive_parser::parse_on_click_attribute(&attributes, context);

    let mut button = Button::new(on_click_handler_name, None, Some(styles));

    let mut child_container = Container::new();
    node.children()
        .filter_map(|child| html_parser::map_dom_to_elements::<State>(&child, Some(&styles), context))
        .for_each(|child_element| {
            child_container.add_child(child_element);
            child_container.set_styles(styles.clone());
        });
        
    button.add_child(Box::new(child_container));

    Box::new(button)
}

fn process_image_element<State : ComponentState>(
    elem_data: &kuchiki::ElementData, 
    _: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Option<Box<dyn Element>> {
    let attributes = elem_data.attributes.borrow();
    let src = attributes.get("src").unwrap_or_default();
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);

    let relative_path = context.angust_config.clone().unwrap_or_default().pathing_config.assets_dir_path.to_string() + "/img";
    let image = Image::new(
        relative_path, src.to_string(), Some(styles)
    );
    Some(Box::new(image))
}

fn process_custom_component<State : ComponentState>(
    component_name: &str, 
    elem_data: &kuchiki::ElementData, 
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Option<Box<dyn Element>> {
    let skippable_elements = vec!["!DOCTYPE", "html", "head", "meta", "body", "title", "h1"]; // To be implemented in the future
    if skippable_elements.contains(&component_name) {
        return html_parser::general_traversal::<State>(node, parent_styles, context)
    }
    
    let attributes = elem_data.attributes.borrow();
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);

    if let Some(mut component_box) = create_component(component_name) {
        component_box.set_styles(styles);
        Some(component_box)
    } else {
        // Continue processing children (To be reported as an error in the future)
        println!("Component not found: {}", component_name);
        return html_parser::general_traversal::<State>(node, Some(&styles), context)
    }
}