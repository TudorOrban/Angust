#[macro_export]
macro_rules! define_component_state {
    ($name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        use $crate::rendering::elements::component::component_state::ComponentState;
        use $crate::rendering::elements::component::reactivity::ReactiveField;
        use std::any::Any;
        use std::sync::Arc;

        #[derive(Debug)]
        pub struct $name {
            $(pub $field: ReactiveField<$type>,)*
        }

        impl $name {
            pub fn new($($field: $type,)*) -> Self {
                Self {
                    $($field: ReactiveField::new($field),)*
                }
            }
        }

        impl ComponentState for $name {
            fn get_property(&self, property_name: &str) -> Option<&dyn Any> {
                match property_name {
                    $(stringify!($field) => Some(&*self.$field as &dyn Any),)*
                    _ => None,
                }
            }

            fn set_property(&mut self, property_name: &str, value: Box<dyn Any>) {
                match property_name {
                    $(
                        stringify!($field) => {
                            if let Ok(casted_value) = value.downcast::<$type>() {
                                *self.$field = *casted_value;  // This will trigger listeners automatically
                            }
                        },
                    )*
                    _ => {},
                }
            }
            
            fn get_all_properties(&self) -> Vec<&str> {
                vec![
                    $( stringify!($field), )*
                ]
            }

            fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
            where
                F: Fn() + 'static,
            {
                match property_name {
                    $(
                        stringify!($field) => {
                            self.$field.subscribe(callback);
                        },
                    )*
                    _ => {},  // Do nothing if the property doesn't exist
                }
            }
        }
    };
}
