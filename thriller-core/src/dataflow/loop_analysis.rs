use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

use crate::AccessMap;

use super::block::ThrillerBlock;

pub struct AccessMapPtr<'a>(&'a Rc<AccessMap>);

impl<'a> Hash for AccessMapPtr<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(Rc::as_ptr(self.0) as usize)
    }
}

impl ThrillerBlock {
    #[allow(dead_code)]
    pub(crate) fn merge_loop(&self) -> HashSet<AccessMapPtr> {
        let mut sets = HashSet::new();

        self.get_inputs().iter().for_each(|edge| {
            if let Some(access_map) = edge.get_access() {
                let ptr = AccessMapPtr(access_map);
                sets.insert(ptr);
            }
        });

        sets
    }
}

impl<'a> PartialEq for AccessMapPtr<'a> {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Implement this function.s
        Rc::ptr_eq(self.0, other.0)
    }
}

impl<'a> Eq for AccessMapPtr<'a> {
    fn assert_receiver_is_total_eq(&self) {
        todo!("Implement this function")
    }
}
