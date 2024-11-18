use commands::command_dispatcher::dispatch_command;

pub mod project_creator;
pub mod commands;
pub mod object_generator;
pub mod shared;

fn main() {
    let angust_version = "0.1.1";
    let angust_macros_version = "0.1.0"; 

    let matches = 
        commands::command_configuration::get_command_configuration()
            .get_matches();
        
    match matches.subcommand() {
        Some((command_name, arg_matches)) => {
            dispatch_command(command_name, arg_matches, angust_version, angust_macros_version);
        },
        _ => {}
    }
}