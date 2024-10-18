

#[macro_export]
macro_rules! wrap_fn {
    ($state:ty, $func:path) => {
        wrap_fn!($state, $func,);
    };
    ($state:ty, $func:path, $($t:ty),+) => {
        Box::new(move |state: &$state, args: Vec<Box<dyn Any>>| -> Box<dyn Any> {
            let mut arg_iter = args.into_iter();

            let result = {
                let func = $func;
                func(
                    state,
                    $(
                        *arg_iter.next().expect("Missing argument")
                            .downcast::<$t>()
                            .expect("Type mismatch in argument downcasting"),
                    )+
                )
            };

            Box::new(result)
        }) as Box<dyn Fn(&$state, Vec<Box<dyn Any>>) -> Box<dyn Any>>
    };
}
