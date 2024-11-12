use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref ID_GENERATOR: Mutex<ElementIDGenerator> = Mutex::new(ElementIDGenerator::new());
}

pub struct ElementIDGenerator {
    current_id: usize,
}

impl ElementIDGenerator {
    pub fn new() -> Self {
        ElementIDGenerator { current_id: 0 }
    }

    pub fn generate(&mut self) -> String {
        let id = self.current_id;
        self.current_id += 1;
        format!("id_{}", id)
    }

    pub fn get() -> String {
        let mut generator = ID_GENERATOR.lock().unwrap();
        generator.generate()
    }
}
