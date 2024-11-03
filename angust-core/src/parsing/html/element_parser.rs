use kuchiki::NodeRef;

use crate::{
    parsing::{
        directive::{for_parser, for_parser::ForLoopContext, if_parser, on_click_parser},
        css::css_parser,
    },
    rendering::elements::{
        container::Container,
        button::Button,
        element::Element,
        image::Image,
        styles::Styles,
        component::state::reactivity::ReactiveState,
    }
};

use super::{
    component_parser::process_custom_component,
    error::ParsingError,
    html_parser::{self, ParsingContext},
};


pub fn dispatch_element_processing<State : ReactiveState>(
    elem_data: &kuchiki::ElementData, 
    node: &NodeRef, 
    parent_styles: Option<&Styles>, 
    context: &mut ParsingContext<State>,
) -> Result<Box<dyn Element>, ParsingError> {
    match elem_data.name.local.as_ref() {
        "div" => process_div_element::<State>(elem_data, node, parent_styles, context),
        "button" => process_button_element::<State>(elem_data, node, parent_styles, context),
        "img" => process_image_element::<State>(elem_data, parent_styles, context),
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

    // Parse on_click event handler into context AST
    let (on_click_handler_name, handler_ast) = on_click_parser::parse_on_click_expression(&attributes, context)?;
    context.add_template_event_handler_ast(on_click_handler_name.clone(), handler_ast);
    
    // Parse children
    let mut child_container = Container::new();
    map_dom_children_to_elements::<State>(node, &mut child_container, context, &styles).unwrap();
        
    let mut button = Button::new(Some(on_click_handler_name), None, Some(styles));
    button.add_child(Box::new(child_container));

    Ok(Box::new(button))
}

fn process_image_element<State : ReactiveState>(
    elem_data: &kuchiki::ElementData, 
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
