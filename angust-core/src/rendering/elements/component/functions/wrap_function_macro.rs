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

#[macro_export]
macro_rules! wrap_fn_mut {
    ($state:ty, $func:path, $($t:ty),*) => {
        Box::new(move |state: &mut $state, args: Vec<Box<dyn Any>>| {
            let mut arg_iter = args.into_iter();
            let result = $func(
                state,
                $(
                    *arg_iter.next()
                        .expect("Missing argument")
                        .downcast::<$t>()
                        .expect(&format!("Type mismatch in argument downcasting for {}", stringify!($t)))
                )*
            );
        })
    };
}

#[macro_export]
macro_rules! wrap_init_mut {
    ($state:ty, $func:path) => {
        Box::new(move |state: &mut $state| {
            let result = $func(state);
        })
    };
}
