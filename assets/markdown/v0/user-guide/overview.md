&nbsp;

# What is Angust?

Angust is a Rust GUI framework designed to bridge the gap between the safety and performance of Rust and the simplicity and scalability of web development. It provides an Angular-style API for developing dynamic UI applications, through reusable Components and associated HTML templates.


This [User Guide](https://tudororban.github.io/Angust/v0/user-guide/overview) aims to cover the essential features of Angust needed to build applications. For a thorough overview of the system and guidelines to contributing, see the [Contributor Guide](https://tudororban.github.io/Angust/v0/contributor-guide/overview). For a complete list of available features, see the [API Reference](https://tudororban.github.io/Angust/v0/api-reference/overview).

&nbsp;

> **Important Note**: Angust is currently in mid stages of development. As such, it still misses many core features, contains many bugs and optimization issues, and it has an unstable API. We plan to release a production-grade initial version of the framework in the near future.  Stay tuned for updates and consider contributing to the project to help shape its future.

&nbsp;

## Features

- Rendering of standard [HTML - CSS](https://tudororban.github.io/Angust/v0/user-guide/html-and-css)
- [Custom Components](https://tudororban.github.io/Angust/v0/user-guide/components/overview) with: HTML templates, reflective and reactive Component State, inputs and outputs, lifetime hooks
- [HTML Directives](https://tudororban.github.io/Angust/v0/user-guide/directives/overview): @if, @for, @onclick, supporting complex Rust expressions
- [Services](https://tudororban.github.io/Angust/v0/user-guide/services/overview): injectable singleton objects, supporting async operations and return to GUI thread for state updates
- Pre-made [Router](https://tudororban.github.io/Angust/v0/user-guide/routing/overview): object encapsulating common navigation functionality (route configuration, history, page caching etc.)
- [CLI tool](https://tudororban.github.io/Angust/v0/user-guide/angust-cli/overview): provides commands for generating boilerplate Angust code for a smooth development experience

&nbsp;

## Next Step
Ready? Jump into the [Getting Started](https://tudororban.github.io/Angust/v0/user-guide/getting-started) tutorial!
