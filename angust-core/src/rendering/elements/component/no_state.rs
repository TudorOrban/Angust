use super::component_state::ComponentState;

pub struct NoState {}

impl ComponentState for NoState {
    fn get_property(&self, _property_name: &str) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn set_property(&mut self, _property_name: &str, _value: Box<dyn std::any::Any>) {}

    fn get_all_properties(&self) -> Vec<&str> {
        vec![]
    }

    fn subscribe_to_property<F>(&mut self, _property_name: &str, _callback: F)
    where
        F: 'static + FnMut(&crate::rendering::elements::component::reactivity::ComponentEvent),
    {
    }
}