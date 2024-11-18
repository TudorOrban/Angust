&nbsp;

# For Directive

The for directive is another structural directive. Its function is to add a HTML element for each item in an array.

&nbsp;

## Basic Usage

Suppose you have a list of navigation items that you want to display in the Header:

```rust
#[component_state]
struct HeaderComponentState {
    navigation_items: Vec<String>,
}
```

You can easily do so by using the for directive:

```html
<div @for="let item of navigation_items">
    {{ item }}
</div>
```

What Angust does with this is loop through `navigation_items`, and for each item add the above `div`, replacing the `{{ item }}` placeholder with the corresponding item value, thus displaying the list of navigation items.

Moreover, you can think of the `div` having the `@for` directive as a *scope* for the `item` variable. So you can do for instance:

```html
<div @for="let item of navigation_items">
    {{ item }}

    <div @if="item == 'Home'">
        This is Home
    </div>
</div>
```

and the "This is Home" element will only show up for an item equal to 'Home'.

**Note**: If you add styles to the above `div`, the styles will be added to each item's `div`, *not* to the parent container.

&nbsp;

## Complex Structs

The for directive supports complex structs. For instance, if you have:

```rust
#[component_state]
struct UIItem {
    label: String,
    value: String,
}

#[component_state]
struct HeaderComponentState {
    navigation_items: Vec<UIItem>,
}
```

you can display the labels like this:

```html
<div @for="let item of navigation_items">
    {{ item.label }}
</div>
```

Also, the array you want to iterate over can be further nested down in the component state. In that case, you can access it like you would normally do in Rust: `let item of header_data.navigation_items` for instance.

&nbsp;

## Next step

Lastly, check out the [onclick directive](https://tudororban.github.io/Angust/v0/user-guide/directives/onclick-directive).

&nbsp;