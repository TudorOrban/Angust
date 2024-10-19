use std::any::Any;

use super::reactivity::ComponentEvent;

pub type ComponentStateType = dyn ComponentState + 'static;

pub trait ComponentState: AsAny {
    fn get_property(&self, property_name: &str) -> Option<Box<dyn Any>>;
    fn set_property(&mut self, property_name: &str, value: Box<dyn Any>);
    fn get_all_properties(&self) -> Vec<&str>;

    fn get_nested_state(&self, property_name: &str) -> Option<&dyn ComponentState>;
}

pub trait ReactiveState : ComponentState {
    fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
        where
            F: 'static + FnMut(&ComponentEvent);
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
