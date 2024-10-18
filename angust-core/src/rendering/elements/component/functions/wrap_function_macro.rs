
use std::any::Any;
        
use crate::rendering::elements::component::component_state::ComponentState;
use crate::rendering::elements::component::functions::dynamic_params_functions::{ArgExtractor, MethodExt};

#[macro_export]
macro_rules! wrap_fn {
    ($func:expr) => {
        Box::new(move |state: &dyn ComponentState, args: Vec<Box<dyn Any>>| -> Box<dyn Any> {
            let mut arg_iter = args.into_iter();
            let extracted_args = <$func as MethodExt<_, _>>::Args::extract(&mut arg_iter)
                .expect("Failed to extract arguments");

            // Handle casting of different return types
            let result = $func(state.downcast_ref().unwrap(), extracted_args);
            Box::new(result) as Box<dyn Any>
        })
    };
}