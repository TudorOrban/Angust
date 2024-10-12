use clap::ArgMatches;

use crate::{object_generator::component_generator, project_creator::angust_project_creator::create_project};


pub fn dispatch_command(command_name: &str, arg_matches: &ArgMatches) {
    match command_name {
        "new" => {
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
    let obj_type = arg_matches.get_one::<String>("type").unwrap();
    let name = arg_matches.get_one::<String>("name").unwrap();
    println!("Generating a {} named '{}'", obj_type, name);

    match obj_type.as_str() {
        "component" => {
            component_generator::generate_component(name.as_str());
        },
        _ => {
            println!("Object type not found");
        }
    }
}