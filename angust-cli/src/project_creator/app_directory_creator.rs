use std::{fs, path::PathBuf};


pub fn create_app_directory(project_root_path: &PathBuf, app_folder_path: &String) {
    let app_dir_path = project_root_path.join(app_folder_path);

    match fs::create_dir_all(&app_dir_path) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to create app directory: {}", e);
        }
    }

    create_app_component(&app_dir_path);
    create_app_template(&app_dir_path);
}

fn create_app_component(app_dir_path: &PathBuf) {
    let app_component_path = app_dir_path.join("app.component.rs");

    let app_component_contents = r#"
    
use angust::rendering::elements::component::component::Component;

pub struct AppComponent {
    component: Component<AppComponentState>,    
}

pub struct AppComponentState {
    content: String,
}

impl AppComponent {
    pub fn new() -> Self {
        let state = AppComponentState { content: String::from("Hello, App Component!") };

        let mut component = Component::new(
            "app-component".to_string(),
            "src/app/app.component.html".to_string(),
            state,
        );

        component.add_event_handler("toggle".to_string(), Box::new(|state: &mut AppComponentState| {
            Self::toggle_content(state);
        }));
        component.add_event_handler("delete".to_string(), Box::new(|state: &mut AppComponentState| {
            Self::delete_content(state);
        }));

        Self { component }
    }

    
    pub fn toggle_content(state: &mut AppComponentState) {
        if state.content == "Initial Content" {
            state.content = String::from("Updated Content");
            println!("Content updated: {}", state.content);
        } else {
            state.content = String::from("Initial Content");
        }
    }

    pub fn delete_content(state: &mut AppComponentState) {
        state.content = String::from("");
    }

}

    "#;

    fs::write(&app_component_path, app_component_contents)
        .expect("Failed to write app_component.rs file");
}

fn create_app_template(app_dir_path: &PathBuf) {
    let app_template_path = app_dir_path.join("app.component.html");

    let app_template_contents = r#"
<div style="background-color: rgb(255, 0, 0)">

    <h1>{{ content }}</h1>

    <button @onclick="toggle">Toggle Content</button>
</div>
    "#;

    fs::write(&app_template_path, app_template_contents)
        .expect("Failed to write app.component.html file");
}
