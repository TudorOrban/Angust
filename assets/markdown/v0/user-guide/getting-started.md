&nbsp;

# Getting Started

This section will guide you through the steps needed to get your first Angust project going. Throughout the assume familiarity with the basics of Rust and HTML-CSS.

&nbsp;

## Set up a new project

### Prerequisites

- **Rust**: Make sure you have Rust installed on your system. You can download it from [rust-lang.org](https://www.rust-lang.org/).
- **Text editor**: We recommend [Visual Studio Code](https://code.visualstudio.com).
- **Terminal**: Required for running Angust CLI commands.

#### Install Angust CLI

For ease of development, we highly recommend installing the [Angust CLI](TBA). After download, add the angust_cli executable to your environment variables.

#### Create a new Angust project

To create a new Angust project, open a terminal window in an appropriate directory and run the `create_project` command with the desired project name:

```
angust_cli create_project <project-name>
```

This will create a new Cargo project, add all the necessary dependencies and all the configuration files needed by Angust. Open the created folder in your IDE. The project structure should look like this:

![Project structure](assets/images/screenshots/InitialProjectStructure.png)

We will explain the role of each file as we move through the guide. For now, you can just run the application with:

```
cargo run
```

&nbsp;

## Next Step
Ready for the next step? Head over to the [HTML and CSS](https://tudorban.github.io/Angust/v0/user-guide/html-and-css) section.

&nbsp;