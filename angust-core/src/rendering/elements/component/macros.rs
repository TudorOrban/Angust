#[macro_export]
macro_rules! component_state {
    ($name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        use std::any::Any;
        use std::sync::{Arc, Mutex};
        use crate::rendering::elements::component::component::ComponentState;
        use crate::rendering::elements::component::reactivity::{Signal, SignalImpl};

        
        #[derive(Debug)]
        pub struct $name {
            $(pub $field: $type,)*
            $(pub signal_$field: Arc<Mutex<SignalImpl<$type>>>,)*
        }

        impl $name {
            pub fn new($($field: $type,)*) -> Self {
                Self {
                    $($field,)*
                    $(signal_$field: Arc::new(Mutex::new(SignalImpl::new())),)*
                }
            }
        }

        impl ComponentState for $name {
            fn get_property(&self, property_name: &str) -> Option<&dyn Any> {
                match property_name {
                    $(stringify!($field) => Some(&self.$field as &dyn Any),)*
                    _ => None,
                }
            }

            fn set_property(&mut self, property_name: &str, value: Box<dyn Any>) {
                match property_name {
                    $(
                        stringify!($field) => {
                            if let Ok(casted_value) = value.downcast::<$type>() {
                                let mut current_value = self.$field;
                                if current_value != *casted_value {
                                    current_value = *casted_value;
                                    let mut signal = self.signal_$field.lock().unwrap();
                                    signal.emit(&current_value);
                                }
                            }
                        },
                    )*
                    _ => {},
                }
            }
        }
    };
}
