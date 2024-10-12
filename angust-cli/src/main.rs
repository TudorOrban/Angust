use clap::{Arg, Command};

fn main() {
    let matches = Command::new("angust-cli")
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
        .get_matches();

    match matches.subcommand() {
        Some(("new", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            println!("Creating a new Angust project with name: {}", name);
            create_project(&name);
        },
        _ => {}
    }
}


fn create_project(name: &str) {

}