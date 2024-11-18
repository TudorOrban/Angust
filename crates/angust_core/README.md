## Angust

Angust is a Rust GUI framework designed to bridge the gap between the safety and performance of Rust and the simplicity and scalability of web development. It provides an Angular-style API to develop dynamic UI applications, through reusable Components and associated HTML templates.

This crate contains the core features provided by Angust. See also the [Angust CLI tool](TBA) and the [Angust Macros](TBA). 

### Features
* Declaration of UI through standard **HTML - CSS**
* **Custom Components** with: HTML templates, reflective and reactive Component State, inputs and outputs, lifetime hooks
* HTML **Directives**: @if, @for, @onclick, supporting complex Rust expressions
* **Custom Services**: injectable singleton objects, supporting async operations and return to GUI thread for state updates
* Pre-made **Router**: object encapsulating common navigation functionality (route configuration, history, page caching etc.)
* **CLI tool**: provides commands for generating boilerplate Angust code for a smooth development experience

 ## Documentation

For comprehensive and up-to-date documentation, please visit our [official documentation website](https://TudorOrban.github.io/Angust). This website is the primary source for all documentation related to Angust, including API references, tutorials, and examples.

### Status
In mid stages of development.

### Contributing
All contributions are warmly welcomed. Head over to [CONTRIBUTING.md](https://github.com/TudorOrban/Angust/blob/main/CONTRIBUTING.md) for details.

### License

Angust is licensed under the MIT License. See the [LICENSE](https://github.com/TudorOrban/Angust/blob/main/LICENSE) file for more details.