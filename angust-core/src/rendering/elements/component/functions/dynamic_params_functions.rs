use std::any::Any;

use crate::rendering::elements::component::component_state::ComponentState;

pub trait ArgExtractor {
    type Args;
    fn extract(iter: &mut dyn Iterator<Item=Box<dyn Any>>) -> Option<Self::Args>;
}

pub trait MethodExt<State: ComponentState, R> {
    type Args: ArgExtractor;
    fn call(state: &State, args: Self::Args) -> R;
}


