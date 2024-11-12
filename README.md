## Angust

Angust is a Rust GUI framework designed to bridge the gap between the safety and performance of Rust and the simplicity and scalability of web development. It provides an Angular-style API to develop dynamic UI applications, through reusable Components and associated HTML templates.

![Screenshots](/screenshots/Compilation.png)

### Features
* Rendering of standard **HTML - CSS**
* **Custom Components** with: HTML templates, reflective and reactive Component State, inputs and outputs, lifetime hooks
* HTML **Directives**: @if, @for, @onclick, supporting complex Rust expressions
* **Custom Services**: injectable singleton objects, supporting async operations and return to GUI thread for state updates
* Pre-made **Router**: object encapsulating common navigation functionality (route configuration, history, page caching etc.)
* **CLI tool**: provides commands for generating boilerplate Angust code for a smooth development experience

### How to use
To use Angust for your next app, follow these steps:
1. Ensure you have a newer version of Rust and Cargo installed.
2. Download the [Angust CLI tool](TBA) and add the path to environment variables.
2. Open your terminal in an appropiate folder and run:

`angust_cli create_project <your_project_name>`

That's it, you can now run the app with `cd <your_project_name>`, `cargo run`. Head over to the [Docs](TBA) to learn how to use Angust's features.

### Status
In mid stages of development.

### Contributing
All contributions are warmly welcomed. Head over to [CONTRIBUTING.md](https://github.com/TudorOrban/ChainOptim-backend/blob/main/CONTRIBUTING.md) for details.
