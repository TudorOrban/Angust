
#[macro_export]
macro_rules! component_state {
    ($name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        use std::any::Any;

        use angust::rendering::elements::component::component::ComponentState;

        #[derive(Debug)]
        pub struct $name {
            $(pub $field: $type),*
        }

        // impl $name {
        //     pub fn new($( $field: $type ),*) -> Self {
        //         $name { $($field),* }
        //     }
        // }

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
                                self.$field = *casted_value;
                            }
                        },
                    )*
                    _ => {},
                }
            }
        }
    };
}