use clap::{Arg, Command};


pub fn get_command_configuration() -> Command {
    Command::new("angust-cli")
        .version("0.1.0")
        .author("Tudor Andrei Orban <tudororban2@gmail.com>")
        .about("A CLI tool for managing Angust projects")
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
                            Arg::new("name")
                                .help("Name of the component")
                                .required(true)
                                .index(1),
                        ),
                )
                .subcommand(
                    Command::new("service")
                        .about("Generate a new service")
                        .arg(
                            Arg::new("name")
                                .help("Name of the service")
                                .required(true)
                                .index(1),
                        ),
                ),
        )
}