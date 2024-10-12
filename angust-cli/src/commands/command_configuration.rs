use clap::{Arg, Command};


pub fn get_command_configuration() -> Command {
    Command::new("angust-cli")
        .version("0.1.0")
        .author("Tudor Andrei Orban <tudororban2@gmail.com>")
        .about("A CLI tool for managing Angust projects")
        .subcommand(
            Command::new("new")
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
                .about("Generate components, services, etc.")
                .arg(Arg::new("type")
                    .help("Type of object to generate (e.g., component, service)")
                    .required(true)
                    .index(1))
                .arg(Arg::new("name")
                    .help("Name of the object")
                    .required(true)
                    .index(2)),
        )
}