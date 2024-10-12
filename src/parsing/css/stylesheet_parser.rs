
#[derive(Debug)]
pub struct Stylesheet {
    pub classes: Vec<StyleClass>,
}

#[derive(Debug)]
pub struct StyleClass {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
}

pub fn parse_stylesheet(stylesheet: &String) -> Stylesheet {
    let mut classes = Vec::new();
    let mut current_class = None;

    for line in stylesheet.lines() {
        let line = line.trim();
        if line.starts_with(".") {
            if let Some(class) = current_class.take() {
                classes.push(class);
            }

            // Remove leading left brace
            let class_name = line[1..]
                .split('{')
                .next()
                .unwrap_or("")
                .trim();

            current_class = Some(StyleClass {
                name: class_name.to_string(),
                properties: Vec::new(),
            });
        } else if let Some(class) = &mut current_class {
            let parts: Vec<&str> = line.split(':').map(str::trim).collect(); 

            if parts.len() == 2 {
                class.properties.push(Property {
                    name: parts[0].trim().to_string(),
                    value: parts[1].split(';').next().unwrap_or("").trim().to_string(), // Remove trailing semicolon
                });
            }
        }
    }

    if let Some(class) = current_class {
        classes.push(class);
    }

    Stylesheet {
        classes,
    }

}
