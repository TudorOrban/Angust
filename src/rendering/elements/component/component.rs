use crate::rendering::elements::element::Element;

pub struct Component<State> {
    pub name: String,
    pub template_relative_path: String,
    pub content: Option<Box<dyn Element>>,
    pub state: State,
}

impl<State> Component<State> {
    pub fn new(name: String, template_relative_path: String, state: State) -> Self {
        Self::initialize(&template_relative_path);

        Self {
            name,
            template_relative_path,
            content: None,
            state,
        }
    }

    fn initialize(template_relative_path: &String) {
        // Load template
    }
}