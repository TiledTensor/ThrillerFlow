use std::rc::Rc;

use crate::Buffer;
use crate::DataType;
use crate::Task;
use crate::ThrillerResult;

/// Convert a variable to a different type.
pub struct Convert {
    src_buf: Rc<Buffer>,
    dst_buf: Rc<Buffer>,
    src_type: DataType,
    dst_type: DataType,
}

impl Convert {
    /// Create a new `Convert` task.
    pub fn new(
        src_buf: Rc<Buffer>,
        dst_buf: Rc<Buffer>,
        src_type: DataType,
        dst_type: DataType,
    ) -> Self {
        // `src_buf` and `dst_buf` must have the same typing.
        // TODO(KuanjuX): Don't use assert here.
        assert_eq!(src_buf.get_typing(), dst_buf.get_typing());

        // `src_buf` and `dst_buf` must have the same shape.
        // TODO(KuanjuX): Don't use assert here.
        assert_eq!(src_buf.get_shape(), dst_buf.get_shape());

        Self {
            src_buf,
            dst_buf,
            src_type,
            dst_type,
        }
    }
}

impl Task for Convert {
    fn emit(&self) -> ThrillerResult<String> {
        Ok(format!(
            "cast_{src_type}_to_{dst_type}({src_buf}, {dst_buf});\n",
            src_type = self.src_type,
            dst_type = self.dst_type,
            src_buf = self.src_buf.get_name(),
            dst_buf = self.dst_buf.get_name(),
        ))
    }

    fn get_name(&self) -> String {
        todo!()
    }
}
