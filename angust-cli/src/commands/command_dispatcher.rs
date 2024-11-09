use clap::ArgMatches;

use crate::{object_generator::{component_generator, service_generator}, project_creator::project_creation_manager::create_project};


pub fn dispatch_command(command_name: &str, arg_matches: &ArgMatches) {
    match command_name {
        "create_project" => {
            let name = arg_matches.get_one::<String>("name").unwrap();
            create_project(&name);
        },
        "generate" => {
            dispatch_generate_command(arg_matches);
        },
        _ => {
            println!("Command not found");
        }
    }
}

fn dispatch_generate_command(arg_matches: &ArgMatches) {
    if let Some(("component", component_matches)) = arg_matches.subcommand() {
        let path = component_matches.get_one::<String>("path").unwrap();
        component_generator::generate_component(&path);
    } else if let Some(("service", service_matches)) = arg_matches.subcommand() {
        let path = service_matches.get_one::<String>("path").unwrap();
        service_generator::generate_service(&path);
    }
}