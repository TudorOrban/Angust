use kuchiki::NodeRef;

use crate::parsing::directive::for_parser::ForLoopContext;
use crate::parsing::directive::{for_parser, if_parser, on_click_parser};
use crate::{parsing::css::css_parser, rendering::elements::component::component_state::ReactiveState};
use crate::rendering::elements::button::Button;
use crate::rendering::elements::component::component_factory_registry::create_component;
use crate::rendering::elements::container::Container;
use crate::rendering::elements::element::Element;
use crate::rendering::elements::image::Image;
use crate::rendering::elements::styles::Styles;

use super::error::ParsingError;
use super::html_parser::{self, ParsingContext};

pub fn dispatch_element_processing<State : ReactiveState>(
    elem_data: &kuchiki::ElementData, 
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Result<Box<dyn Element>, ParsingError> {
    match elem_data.name.local.as_ref() {
        "div" => process_div_element::<State>(elem_data, node, parent_styles, context),
        "button" => process_button_element::<State>(elem_data, node, parent_styles, context),
        "img" => process_image_element::<State>(elem_data, node, parent_styles, context),
        component_name => process_custom_component::<State>(component_name, elem_data, node, parent_styles, context),
    }
}

fn process_div_element<State : ReactiveState>(
    elem_data: &kuchiki::ElementData, 
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Result<Box<dyn Element>, ParsingError> {
    let mut container = Container::new();
    let attributes = elem_data.attributes.borrow();

    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);
    container.set_styles(styles);

    let should_add_to_dom = if_parser::parse_if_expression(&attributes, context)?;
    if !should_add_to_dom {
        return Ok(Box::new(container)); // Skip adding to DOM
    }

    let for_loop_context = for_parser::parse_for_expression(&attributes, context)?;
    let array_length = for_loop_context.array_length;

    if for_loop_context.is_for_loop {
        parse_for_loop(node, context, &for_loop_context, array_length, &styles, &mut container);
    } else {
        let children_results: Result<Vec<Box<dyn Element>>, ParsingError> = node.children()
            .map(|child| html_parser::map_dom_to_elements::<State>(&child, Some(&styles), context))
            .collect();

        let children = children_results?;
        for child_element in children {
            container.add_child(child_element);
        }
    }

    Ok(Box::new(container))
}

fn map_dom_children_to_elements<State : ReactiveState>(
    node: &NodeRef, 
    node_element: &mut Container,
    context: &mut ParsingContext<State>,
    styles: &Styles,
) -> Result<(), ParsingError> {
    let children_results: Result<Vec<Box<dyn Element>>, ParsingError> = node.children()
            .map(|child| html_parser::map_dom_to_elements::<State>(&child, Some(styles), context))
            .collect();

    let children = children_results?;
    for child_element in children {
        node_element.add_child(child_element);
    }

    Ok(())
}

fn parse_for_loop<State: ReactiveState>(
    node: &NodeRef, 
    context: &mut ParsingContext<State>,
    for_loop_context: &ForLoopContext,
    array_length: usize,
    styles: &Styles,
    container: &mut Container,
) {
    context.add_for_loop_context(for_loop_context.clone());

    for _ in 0..array_length {
        let mut child = Container::new();
        child.set_styles(styles.clone());

        map_dom_children_to_elements::<State>(node, &mut child, context, styles).unwrap();
        container.add_child(Box::new(child));

        context.increment_loop_index(&for_loop_context.context_id);
    }

    context.remove_loop_context(&for_loop_context.context_id);
}

fn process_button_element<State : ReactiveState>(
    elem_data: &kuchiki::ElementData, 
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Result<Box<dyn Element>, ParsingError> {
    let attributes = elem_data.attributes.borrow();
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);

    // let on_click_handler_name = directive_parser::parse_on_click_attribute(&attributes, context);
    let (on_click_handler_name, handler_ast) = on_click_parser::parse_on_click_expression(&attributes, context)?;
        
    context.add_template_event_handler_ast(on_click_handler_name.clone(), handler_ast);

    let mut button = Button::new(Some(on_click_handler_name), None, Some(styles));

    let mut child_container = Container::new();
    map_dom_children_to_elements::<State>(node, &mut child_container, context, &styles).unwrap();
        
    button.add_child(Box::new(child_container));

    Ok(Box::new(button))
}

fn process_image_element<State : ReactiveState>(
    elem_data: &kuchiki::ElementData, 
    _: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Result<Box<dyn Element>, ParsingError> {
    let attributes = elem_data.attributes.borrow();
    let src = attributes.get("src").unwrap_or_default();
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);

    let relative_path = context.angust_config.clone().unwrap_or_default().pathing_config.assets_dir_path.to_string() + "/img";
    let image = Image::new(
        relative_path, src.to_string(), Some(styles)
    );
    Ok(Box::new(image))
}

fn process_custom_component<State : ReactiveState>(
    component_name: &str, 
    elem_data: &kuchiki::ElementData, 
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Result<Box<dyn Element>, ParsingError> {
    let skippable_elements = vec!["!DOCTYPE", "html", "head", "meta", "body", "title", "h1"]; // To be implemented in the future
    if skippable_elements.contains(&component_name) {
        return html_parser::general_traversal::<State>(node, parent_styles, context)
    }
    
    let attributes = elem_data.attributes.borrow();
    let styles = css_parser::parse_styles(&attributes, parent_styles, &context.stylesheet);

    if let Some(mut component_box) = create_component(component_name) {
        component_box.set_styles(styles);
        Ok(component_box)
    } else {
        // Continue processing children (To be reported as an error in the future)
        println!("Component not found: {}", component_name);
        return html_parser::general_traversal::<State>(node, Some(&styles), context)
    }
}