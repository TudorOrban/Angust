&nbsp;

# HTML and CSS

In Angust projects, the UI is declared through HTML and CSS. This approach allows you to create concise, customizable and scalable UIs. It also paves the way for the [Components](https://tudorban.github.io/Angust/v0/user-guide/components/overview) of the next section.

&nbsp;

## Example

Here is a simple example of how to declare UI elements in Angust:

```html
<div style="display: flex; flex-direction: row; align-items: center; spacing: 0px 20px;">
    <div style="font-size: 24px; font-weight: 600px;">
        Press this button to increase the count:
    </div>

    <button @onclick="increase_count()">
        Increase
    </button>
</div>
```

This defines a text element and a button, aligned horizontally with a space of 20 pixels in between. 

&nbsp;

## Stylesheets

You can also define CSS classes in separate stylesheet files, and use them like this:

```html
<div class="custom-page-title">
    Some Title
</div>
```

The standard project generated by the [Angust CLI](https://tudorban.github.io/Angust/v0/user-guide/angust-cli/overview) already contains a `styles.css` file at the root of the `src` folder, which is automatically loaded by Angust. You can define your classes there or in separate files, as long as you import them in `styles.css` with the `@import` directive, for instance:
```css
@import 'forms.css'
```
This will look for the `forms.css` file *in the `src/styles` directory*. You can change this in `angust.config.json`, although it's recommended to stick with that convention unless you have a specific need.

&nbsp;

## Supported Elements and Styles

As Angust is in the development phase, it supports only a limited part of the standard HTML elements and CSS styles. The supported HTML elements are:

- `div`: the basic element enabling declaration of complex UI layouts
- `button`: an element with an associated `onclick` handler, enabling dynamic behavior
- `img`: element that allows you to render any image within the UI
- `text`: you can render any text by just adding it in your HTML

The supported CSS styles are:

### Layout

- `display`: controls the basic layout behavior of a `div` element. It can be:
  - `block`: the element takes up the full width of its container, starting on a new line
  - `inline-block`: the element does not start on a new line but can have width and height set
  - `flex`: the element behaves as a flexible container that arranges its children dynamically

- `flex-direction`: used along with `flex`. It can be:
  - `row`: the element lays out children horizontally
  - `column`: the element lays out children vertically

- `justify-content`: controls how the children are distributed along the main axis. It can be:
  - `flex-start`: children are aligned at the start of the container
  - `flex-end`: children are aligned at the end of the container
  - `center`: children are centered within the container
  - `space-between`: children are evenly distributed, with the first child at the start of the container
  - `space-around`: children are evenly distributed, with equal space around each child

- `align-items`: controls how the children are aligned along the cross axis. It can be:
  - `flex-start`: children are aligned at the start of the cross axis
  - `flex-end`: children are aligned at the end of the cross axis
  - `center`: children are centered across the cross axis
  - `stretch`: children stretch to fill the container along the cross axis
  - `baseline`: children are aligned such as their baselines align

- `overflow`: controls how the container deals with the lack of space of its content. It can be:
  - `visible`: the content is not clipped and may be rendered outside of the container
  - `hidden`: the content is clipped and no scrollbars are provided
  - `scroll`: the content is clipped and scrollbars are added when necessary
  - `auto`: similar to `scroll` but may dependent on user agent

- `flex-grow`: controls to what extend the children can grow to fill the container. Can be any number from 0 to 1

- `flex-shrink`: controls to what extend the children can shrink when there's not enough space. Can be any number from 0 to 1

- `width`, `height`, `max-width`, `min-width`, `min-height`, `max-height`: control the dimensions of the container. Can be specified in pixels (eg. `width: 100px`), or in percentage of the total size of the container (eg. `height: 50%`).

- `spacing`: controls the spacing between the children of the container. Expects two values, vertical and horizontal spacing respectively

- `margin`: controls the space around the container. Expects four values, top, right, bottom and left margins respectively

- `padding`: controls the space within the container, affecting the children. Expects four values, left, right, bottom and left margins respectively

### Appearance

- `background-color`: sets the background color of the container. Expects three values between 0 and 255, for example: `background-color: rgb(255, 0, 0)`

- `color`: sets the color of the text within the container. Expects three values between 0 and 255
  
- `border`: sets the width and color of the border of the container. They can be provided like this: `border: 1px rgb(255, 0, 0)`

### Text
- `white-space`: controls whether the text breaks into multiple lines when it doesn't have enough space. It can be:
  - `normal`: text wraps to a new line
  - `no-wrap`: text doesn't wrap to a new line

- `font-size`: controls the font size of the text
- `font-family`: controls the font family of the text
- `font-weight`: controls the font weight of the text
- `font-style`: controls the font style of the text

&nbsp;

## Issues and Missing Styles

The layout algorithm still has a few issues. Notably:

- the percentage-based sizes are not always attributed accurately to the specifications
- the `justify-content` value `space-around` is not working properly yet
- the scrollbar on `overflow: auto` is not appearing properly in some cases

However, we expect that these bugs will be fixed before the release of the initial version. Moreover, there are several important CSS styles that are not currently implemented but will be soon. For example: `position`, `grid`, rounded containers, shadows, text truncation etc.

&nbsp;