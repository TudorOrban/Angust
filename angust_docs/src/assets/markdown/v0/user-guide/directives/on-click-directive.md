&nbsp;

# On Click Directive

The onclick directive is a simple directive that can be used to trigger effects in response to user input.

&nbsp;

## Basic Usage

Suppose you have a function registered as described in the [Component Functions](https://tudororban.github.io/Angust/v0/user-guide/components/component-functions) section. Following the same example:

```rust
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

you can use the `@onclick` directive to trigger `toggle_dark_theme` whenever the user clicks on a button:

```html
<button @onclick="toggle_dark_theme()">
    Dark Theme
</button>
```

If you need your event handler to receive some extra parameters, you can do so just as easily.

&nbsp;

## Next step

Now that you've learned about Components and Directives, you should be able to build dynamic and modular apps with Angust. To further increase the scalability and decoupling of your app, we recommend learning about [Services](https://tudororban.github.io/Angust/v0/user-guide/services/overview) next.

&nbsp;