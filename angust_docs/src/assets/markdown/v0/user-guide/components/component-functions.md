
&nbsp;

# Component Functions

**Component** functions are functions from the implementation of some component that you can use directly in the component template. 

&nbsp;

## Defining Functions

As with Components and their states, Component functions require some manual registration, in order for Angust to trigger them dynamically. Take a look at this example:

```rust
use angust_macros::component_state;

#[component_state]
struct SomeComponentState {
    is_dark_theme: String
}

impl SomeComponentState {

    pub fn toggle_dark_theme(&mut self) {
        self.is_dark_theme = !self.is_dark_theme;
    }
}
```

To register the `toggle_dark_theme` function, it needs to first be transformed into this type: `Box<dyn Fn(&mut State, Vec<Box<dyn Any>>)>`. We again provide macros that automatically perform this transformation, depending on the signature. For the `toggle_dark_theme`, the appropriate macro is:

```rust
use angust::wrap_fn_mut_no_params;

let toggle_dark_theme_any = wrap_fn_mut_no_params!(SomeComponentState, SomeComponentState::toggle_dark_theme);
```

Note that you lose Rust's type safety with this transformation, but otherwise it wouldn't be possible for Angust to access your functions.

The last step is to provide this function to the component during registration as follows:

```rust
pub struct SomeComponent;

impl SomeComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("some-component".to_string(), Box::new(move || {
            let state = SomeComponentState::new(false);

            let mut component = Component::new(
                "some-component".to_string(),
                "src/app/some_component.html".to_string(),
                state
            );

            let component_functions: ComponentFunctions<SomeComponentState> = ComponentFunctions::new(
                vec![], vec![], vec![], vec![], 
                vec![
                    ("toggle_dark_theme", wrap_fn_mut_no_params!(SomeComponentState, SomeComponentState::toggle_dark_theme))
                ], 
                vec![], None
            );
            component.add_component_functions(component_functions);

            Box::new(component)
        }));
    }
}
```

Here we pass the transformed function to the `ComponentFunctions` constructor and add `component_functions` to the component. Ignore the other arguments for now.

We will see in the [Directives](https://tudorban.github.io/Angust/v0/user-guide/directives/overview) how you can trigger the `toggle_dark_theme` from the template.

&nbsp;

## Next Step
Next, dive into [Component Inputs and Outputs](https://tudorban.github.io/Angust/v0/user-guide/components/inputs-and-outputs).

&nbsp;