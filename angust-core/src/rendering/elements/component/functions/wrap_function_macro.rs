#[macro_export]
macro_rules! wrap_fn {
    ($state:ty, $func:path, $($t:ty),*) => {
        Box::new(move |state: &$state, args: Vec<&dyn Any>| -> &dyn Any {
            let mut arg_iter = args.into_iter();
            let result = $func(
                state,
                $(
                    arg_iter.next()
                        .expect("Missing argument")
                        .downcast_ref::<$t>()
                        .expect("Type mismatch in argument downcasting")
                        .clone(),
                )*
            );

            let boxed_result = Box::new(result);
            Box::leak(boxed_result) as &dyn Any
        })
    };
}
