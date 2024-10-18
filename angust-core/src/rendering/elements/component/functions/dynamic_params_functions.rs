use std::any::Any;

pub trait ArgExtractor {
    type Args;
    fn extract(iter: &mut dyn Iterator<Item=Box<dyn Any>>) -> Option<Self::Args>;
}

pub trait MethodExt<State, R> {
    type Args: ArgExtractor;
    fn call(state: &State, args: Self::Args) -> R;
}
