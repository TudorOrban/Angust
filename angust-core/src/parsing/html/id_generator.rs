use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref ID_GENERATOR: Mutex<IDGenerator> = Mutex::new(IDGenerator::new());
}

pub struct IDGenerator {
    current_id: usize,
}

impl IDGenerator {
    pub fn new() -> Self {
        IDGenerator { current_id: 0 }
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
