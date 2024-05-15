use std::fs;
use std::io::Write;
use std::process::Command;
use std::{env::temp_dir, fs::File};

use crate::{Task, ThrillerBlock, ThrillerError, ThrillerResult};

/// `ThrillerEngine` is the main entry point for the ThrillerFlow framework.
pub struct ThrillerEngine {
    dataflow_block: ThrillerBlock,
}

impl ThrillerEngine {
    /// Create a new ThrillerEngine with the given dataflow block.
    pub fn new(dataflow_block: ThrillerBlock) -> Self {
        ThrillerEngine { dataflow_block }
    }

    /// Emit the function signature for the given dataflow block.
    pub(crate) fn emit_function_signature(&self) -> ThrillerResult<String> {
        todo!()
    }

    /// Generate the ThrillerFlow code for the given dataflow block.
    pub fn dataflow_generate(&self) -> ThrillerResult<String> {
        let mut code = String::new();
        code += self.emit_function_signature()?.as_str();
        code += "{\n";
        let dataflow_code = self.dataflow_block.emit()?;
        let lines = dataflow_code.lines().collect::<Vec<_>>();
        let indient = 4;
        for line in lines {
            code.push_str(&format!(
                "{indent}{line}\n",
                indent = " ".repeat(indient),
                line = line
            ));
        }
        code += "}\n";
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

    /// Install macro kernel for the given dataflow block.
    pub fn install_library(&self) -> ThrillerResult<()> {
        let temp_dir = temp_dir().join("thriller");
        fs::create_dir(temp_dir.clone()).map_err(|_| ThrillerError::FailedFileOp)?;

        std::env::set_current_dir(&temp_dir).map_err(|_| ThrillerError::FailedFileOp)?;

        let clone = Command::new("git")
            .args(["clone", "git@github.com:TiledTensor/TiledCUDA.git"])
            .output()
            .map_err(|_| ThrillerError::FailedFileOp)?;

        if !clone.status.success() {
            return Err(ThrillerError::FailedFileOp);
        }

        let repo_dir = temp_dir.join("TiledCUDA");
        std::env::set_current_dir(repo_dir).map_err(|_| ThrillerError::FailedFileOp)?;

        let submodule = Command::new("git")
            .args(["submodule", "update", "--init", "--recursive"])
            .output()
            .map_err(|_| ThrillerError::FailedFileOp)?;

        if !submodule.status.success() {
            return Err(ThrillerError::FailedFileOp);
        }

        Ok(())
    }
}
