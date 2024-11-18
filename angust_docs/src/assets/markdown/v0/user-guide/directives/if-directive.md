&nbsp;

# If Directive

The if directive is a *structural* directive, meaning it can manipulate the HTML layout. Specifically, the if directive can add or remove an element from the layout based on whether or not a condition evaluates to true.

## Basic Usage

Consider the following example:

```rust
#[component_state]
struct AppComponentState {
    is_dark_theme: bool
}

impl AppComponentState {

    fn toggle_dark_theme(&mut self) {
        self.is_dark_theme = !self.is_dark_theme;
    }
}
```

Suppose you want to provide some visual clue that the dark theme is on. Then you can simply add to your HTML template:

```html
<!-- Content -->

<div @if="is_dark_theme">
    Dark Theme is on
</div>
```

Now the `div` will only be included in the DOM if `is_dark_theme` is true. 

Moreover, recall that Angust Component State is *reactive*, meaning changes in its properties trigger rerenders. Hence, the above `div` will automatically enter and exit the layout when the dark theme is toggled.

&nbsp;

## Complex Expressions

The `@if` directive supports complex Rust expressions. For instance, if you have a property:

```rust
#[component_state]
struct AppComponentState {
    count: i32,
}
```

then you can put:

```html
<div @if="count >= 5">
    Count is greater than 5.
</div>
```

The directive supports:
- numbers, string literals and booleans
- arithmetic, comparison and binary operations
- state properties (or loop variables - see next section)
- function calls, as long as the functions are registered as described in the [Component Functions](https://tudororban.github.io/Angust/v0/user-guide/components/component-functions) section.

**Note**: The if directive currently works only for `div` elements. We will soon enable it for other elements as well.

&nbsp;

## Next Step

Next, learn about the [for directive](https://tudororban.github.io/Angust/v0/user-guide/directives/for-directive).

&nbsp;
