use std::any::Any;

pub trait ComponentState {
    fn get_property(&self, property_name: &str) -> Option<&dyn Any>;
    fn set_property(&mut self, property_name: &str, value: Box<dyn Any>);
}

// pub struct NoState {
//     no_content: bool,
// }

// impl ComponentState for NoState {
//     fn get_property(&self, _property_name: &str) -> Option<&dyn Any> {
//         None
//     }

//     fn set_property(&mut self, _property_name: &str, _value: Box<dyn Any>) {}
// }


// impl Default for NoState {
//     fn default() -> Self {
//         NoState {}
//     }
// }
// component_state! {
//     NoState { }
// }
