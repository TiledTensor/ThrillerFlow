use crate::Var;

pub struct RegularVar {
    name: String,
    id: usize,
}

impl Var for RegularVar {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
