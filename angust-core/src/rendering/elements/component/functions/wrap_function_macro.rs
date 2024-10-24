#[macro_export]
macro_rules! wrap_fn {
    ($state:ty, $func:path, $($t:ty),*) => {
        Box::new(move |state: &$state, args: Vec<Box<dyn Any>>| -> Box<dyn Any> {
            let mut arg_iter = args.into_iter();
            let result = $func(
                state,
                $(
                    arg_iter.next()
                        .expect("Missing argument")
                        .downcast::<$t>()
                        .expect("Type mismatch in argument downcasting")
                        .as_ref()
                        .clone(),
                )*
            );

            Box::new(result) as Box<dyn Any>
        })
    };
}
