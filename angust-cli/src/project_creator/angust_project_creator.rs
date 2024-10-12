
pub fn create_project(name: &str) {
    println!("Creating a new Angust project with name: {}", name);

    let project_root_path = format!("");
    create_angust_configuration_file(&project_root_path);

}

fn create_angust_configuration_file(project_root_path: &str) {
    println!("Creating an Angust configuration file");
}