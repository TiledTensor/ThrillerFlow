use crate::{task::Task, ThrillerResult};

#[allow(dead_code)]
pub enum CopyType {
    GlobalToShared,
    SharedToGlobal,
    SharedToRF,
    RFToShared,
}

/// A task to copy data from one memory to another
pub struct CopyTask {
    copy_type: CopyType,
}

impl Task for CopyTask {
    fn emit(&self) -> ThrillerResult<String> {
        match self.copy_type {
            // TODO: Implement emit for each CopyType
            CopyType::GlobalToShared => Ok(format!("copy_tensor_g2s({}, {}, {})", "a", "b", "c")),
            CopyType::SharedToGlobal => Ok(format!("copy_tensor_s2g({}, {}, {})", "a", "b", "c")),
            CopyType::SharedToRF => Ok(format!("copy_tensor_s2rf({}, {}, {})", "a", "b", "c")),
            CopyType::RFToShared => Ok(format!("copy_tensor_rf2s({}, {}, {})", "a", "b", "c")),
        }
    }
}
