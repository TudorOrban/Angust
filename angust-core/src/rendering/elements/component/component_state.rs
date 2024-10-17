use std::any::Any;

pub trait ComponentState: 'static {
    fn get_property(&self, property_name: &str) -> Option<&dyn Any>;
    fn set_property(&mut self, property_name: &str, value: Box<dyn Any>);
    fn get_all_properties(&self) -> Vec<&str>;
}
