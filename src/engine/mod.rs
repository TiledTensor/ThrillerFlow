use std::{fs::File, io::Write};

use crate::{ThrillerBlock, ThrillerError, ThrillerResult};

/// `ThrillerEngine` is the main entry point for the ThrillerFlow framework.
pub struct ThrillerEngine {
    dataflow_block: ThrillerBlock,
}

impl ThrillerEngine {
    /// Create a new ThrillerEngine with the given dataflow block.
    pub fn new(dataflow_block: ThrillerBlock) -> Self {
        ThrillerEngine { dataflow_block }
    }

    /// Generate the ThrillerFlow code for the given dataflow block.
    pub fn dataflow_generate(&self) -> ThrillerResult<String> {
        let mut code = String::new();
        code += self.dataflow_block.gen_loop_load()?.as_str();
        Ok(code)
    }

    /// Persist the generated ThrillerFlow code to the given file.
    pub fn persist(&self, file_name: &str) -> ThrillerResult<()> {
        let code = self.dataflow_generate()?;
        let mut file = File::create(file_name).unwrap();
        // file.write_all(code.as_bytes()).unwrap();
        file.write_all(code.as_bytes())
            .map_err(|_| ThrillerError::FailedFileOp)?;

        Ok(())
    }
}
