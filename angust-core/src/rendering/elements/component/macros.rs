#[macro_export]
macro_rules! define_component_state {
    ($name:ident { $($field:ident: $type:ty),* $(,)? }) => {

        #[derive(Debug, Clone)]
        pub struct $name {
            $(pub $field: $crate::rendering::elements::component::reactivity::ReactiveField<$type>,)*
        }

        impl $name {
            pub fn new($($field: $type,)*) -> Self {
                Self {
                    $($field: $crate::rendering::elements::component::reactivity::ReactiveField::new($field),)*
                }
            }
        }

        impl $crate::rendering::elements::component::component_state::ComponentState for $name {
            fn get_property(&self, property_name: &str) -> Option<Box<dyn Any>> {
                match property_name {
                    $(stringify!($field) => Some(Box::new(self.$field.value.clone())),)*
                    _ => None,
                }
            }

            fn set_property(&mut self, property_name: &str, value: Box<dyn Any>) {
                match property_name {
                    $(
                        stringify!($field) => {
                            if let Ok(casted_value) = value.downcast::<$type>() {
                                self.$field.set(*casted_value);
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

            
            fn get_nested_state(&self, property_name: &str) -> Option<&dyn $crate::rendering::elements::component::component_state::ComponentState> {
                match property_name {
                    $(
                        stringify!($field) => {
                            // If the field implements `ComponentState`, return it as nested state
                            if let Some(nested) = self.$field.value.as_any().downcast_ref::<$type>()
                                .and_then(|state| state.as_any().downcast_ref::<dyn $crate::rendering::elements::component::component_state::ComponentState>()) {
                                Some(nested)
                            } else {
                                None
                            }
                        },
                    )*
                    _ => None,
                }
            }
        
        }

        impl $crate::rendering::elements::component::component_state::ReactiveState for $name {
            fn subscribe_to_property<F>(&mut self, property_name: &str, callback: F)
            where
                F: 'static + FnMut(&$crate::rendering::elements::component::reactivity::ComponentEvent),
            {
                match property_name {
                    $(stringify!($field) => self.$field.subscribe(callback),)*
                    _ => {}
                }
            }
        }
    };
}
