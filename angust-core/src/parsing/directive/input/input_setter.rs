use std::{any::Any, collections::HashMap};

use crate::rendering::elements::component::{component::Component, state::reactivity::ReactiveState};


pub fn trigger_input_setters<State: ReactiveState>(
    component: &mut Component<State>, 
    inputs: HashMap<String, Box<dyn Any>>
) {
    for (input_name, input_value) in inputs.into_iter() {
        let setter_name = format!("set_{}", input_name);

        let setter = component.component_functions.input_setters.get(&setter_name);
        if setter.is_none() {
            continue;
        }
        let setter = setter.unwrap();

        let vec = vec![input_value];
        setter(&mut component.state, vec);
    }
}