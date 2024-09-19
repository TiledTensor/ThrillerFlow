use std::cell::OnceCell;

pub static mut ID_COUNTER: OnceCell<IdCounter> = OnceCell::new();

/// [`IdCounter`] is a counter that generates unique IDs.
pub struct IdCounter {
    id: usize,
}

impl IdCounter {
    pub(crate) fn new() -> Self {
        IdCounter { id: 0 }
    }

    pub(crate) fn next(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        id
    }
}
