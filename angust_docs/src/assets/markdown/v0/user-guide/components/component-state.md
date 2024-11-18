
&nbsp;

# Component State

## Basics

For each Angust Component, you can have a **Component State**. This is a struct that holds the current state of the Component. However, it is not just a plain Rust struct, but rather has two essential properties:

- it is *reflective*, meaning Angust can interact with its properties at runtime without having access to the struct. This is necessary for substituting state placeholders.
- it is *reactive*, meaning a change in the state will trigger a rerendering of the template HTML.

These properties enable dynamic patterns such as:

```html
<div>
    {{ content }}
</div>
```

Here `{{ content }}` is a **state placeholder**, meaning Angust will substitute it with the actual value of the `content` property on the corresponding Component State. Moreover, a change in `content` will be immediately reflected on screen.

&nbsp;

## Defining State

As opposed to the Angular Javascript world, Rust structs are not naturally reflective or reactive. In order for Angust to provide the functionality underlined above, each of your Component State structs, as well as substructs, needs to implement two traits: `ReflectiveState` and `ReactiveState`. 

However, it is not necessary that you implement this manually, as we provide a procedural macro that automates this process:

```rust
use angust_macros::component_state;

#[component_state]
struct SomeComponentState {
    content: String
}
```

The `angust_macros` crate should be already added to your dependencies by the `angust_cli create_project` command.

Once you define your struct, you can provide it, with some initial values, in the component registration:

```rust
pub struct SomeComponent;

impl SomeComponent {
    pub fn register(registry: &mut HashMap<String, ComponentFactory>) {
        registry.insert("some-component".to_string(), Box::new(move || {
            let state = SomeComponentState::new(
                "Welcome to Angust!".to_string(),
            );

            let mut component = Component::new(
                "some-component".to_string(),
                "src/app/some_component.html".to_string(),
                state
            );

            Box::new(component)
        }));
    }

}
```

Now the `{{ content }}` placeholder in the template should be replaced with "Welcome to Angust!".

&nbsp;

> **Important Note**: Reactivity of the Component State will be considerably refactored in the near future. Expect changes before the release of the initial version v1.

&nbsp;

## Next Step

Next you can learn about how to define [Component Functions](https://tudororban.github.io/Angust/v0/user-guide/components/component-functions).

&nbsp;