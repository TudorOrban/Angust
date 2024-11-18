use clap::{Arg, Command};


pub fn get_command_configuration() -> Command {
    Command::new("angust_cli")
        .version("0.1.1")
        .author("Tudor Andrei Orban <tudoraorban@protonmail.com>")
        .about("CLI tool for the Angust GUI framework")
        .subcommand(
            Command::new("create_project")
                .about("Creates a new Angust project")
                .arg(
                    Arg::new("name")
                        .help("The name of the project")
                        .required(true)
                        .index(1),
                ),   
        )
        .subcommand(
            Command::new("generate")
                .about("Generate components, services, etc. Should be run from the root of the project, with component name in PascalCase")
                .subcommand(
                    Command::new("component")
                        .about("Generate a new component")
                        .arg(
                            Arg::new("path")
                                .help("Path of the component")
                                .required(true)
                                .index(1),
                        ),
                )
                .subcommand(
                    Command::new("service")
                        .about("Generate a new service")
                        .arg(
                            Arg::new("path")
                                .help("Path of the service")
                                .required(true)
                                .index(1),
                        ),
                ),
        )
}