use std::any::Any;

pub trait ComponentState {
    fn get_property(&self, property_name: &str) -> Option<&dyn Any>;
    fn set_property(&mut self, property_name: &str, value: Box<dyn Any>);
    fn get_all_properties(&self) -> Vec<&str>;

    
    fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
    where
        F: Fn() + 'static;
}
