// #[macro_export]
// macro_rules! wrap_fn {
//     ($state:ty, $func:path) => {
//         wrap_fn!($state, $func,);
//     };
//     ($state:ty, $func:path, $($t:ty),+) => {
//         Box::new(move |state: &$state, args: Vec<&dyn Any>| -> &dyn Any {
//             let mut arg_iter = args.into_iter();

//             // Call the actual function with downcasted arguments
//             let result = {
//                 let func = $func;
//                 func(
//                     state,
//                     $(
//                         arg_iter.next().expect("Missing argument")
//                             .downcast_ref::<$t>()
//                             .expect("Type mismatch in argument downcasting")
//                             .clone() // Make sure cloning works for the argument type
//                     )+
//                 )
//             };

//             // Return result as a dynamically sized reference
//             &result as &dyn Any
//         }) as Box<dyn for<'a, 'b> Fn(&'a $state, Vec<&'b dyn Any>) -> &dyn Any>
//     };
// }
