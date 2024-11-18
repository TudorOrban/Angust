
&nbsp;

# Inputs and Outputs

There are many scenarios in which you may want to share state between a parent component and a child component. This is made possible by component **Inputs** and **Outputs**.

## Defining Inputs

Consider the following example, where you have a parent `AppComponent` which contains a `HeaderComponent`:

```rust
#[component_state]
struct AppComponentState {
    is_dark_theme: bool,
}

impl AppComponentState {

}

pub struct AppComponent;

impl AppComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("app-component".to_string(), Box::new(move || {
            let state = AppComponentState::new(false);

            let component = Component::new(
                "app-component".to_string(),
                "src/app/app_component.html".to_string(),
                state
            );

            Box::new(component)
        }));
    }
}
```

```html
<div>
    <header-component></header-component>

    <!-- Rest of the content -->
</div>
```

and the Header component needs access to the `is_dark_theme`. To achieve this, you need to provide the `HeaderComponent` with a setter of the `is_dark_theme` property. This is done similar to registering Component Functions: use a macro provided by Angust to transform the setter, then pass the result to the `ComponentFunctions` struct:

```rust
#[component_state]
struct HeaderComponentState {
    is_dark_theme: bool,
}

impl HeaderComponentState {

    pub fn set_is_dark_theme(&mut self, is_dark_theme: bool) {
        self.is_dark_theme = is_dark_theme;
    }
}

pub struct HeaderComponent;

impl HeaderComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("header-component".to_string(), Box::new(move || {
            let state = HeaderComponentState::new(false);

            let mut component = Component::new(
                "header-component".to_string(),
                "src/app/header_component.html".to_string(),
                state
            );

            let component_functions = ComponentFunctions::new(
                vec![], vec![], vec![], vec![], vec![], 
                vec![
                    ("set_is_dark_theme", wrap_fn_mut!(HeaderComponentState, HeaderComponentState::set_is_dark_theme, bool))
                ],
                None
            );
            component.add_component_functions(component_functions);

            Box::new(component)
        }));
    }
}
```

Now you only need to provide the `is_dark_theme` like this:

```html
<div>
    <header-component [is_dark_theme]="is_dark_theme"></header-component>

    <!-- Rest of the content -->
</div>
```

&nbsp;

## Defining Outputs

Outputs are a way to notify parent components when changes occur in the state of children components. 

**Outputs are not currently available**, but they will be in the near future. Stay tuned for updates and consider getting in touch with us to contribute to Angust's future!

&nbsp;

## Next Step
Next, dive into [Lifecycle Hooks](https://tudororban.github.io/Angust/v0/user-guide/components/lifecycle-hooks).

&nbsp;