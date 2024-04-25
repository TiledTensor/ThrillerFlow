use std::cell::OnceCell;

pub static mut ID_COUNTER: OnceCell<IdCounter> = OnceCell::new();

pub struct IdCounter {
    id: usize,
}

impl IdCounter {
    pub fn new() -> Self {
        IdCounter { id: 0 }
    }

    pub fn next(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        id
    }
}
