use std::fs;
use std::io::Write;
use std::process::Command;
use std::rc::Rc;
use std::{env::temp_dir, fs::File};

use crate::{RegularVar, Task, ThrillerBlock, ThrillerError, ThrillerResult, Var};

/// `ThrillerEngine` is the main entry point for the ThrillerFlow framework.
pub struct ThrillerEngine {
    dataflow_block: ThrillerBlock,
    inputs: Vec<Rc<RegularVar>>,
    outputs: Vec<Rc<RegularVar>>,
}

impl ThrillerEngine {
    /// Create a new ThrillerEngine with the given dataflow block.
    pub fn new(dataflow_block: ThrillerBlock) -> Self {
        ThrillerEngine {
            dataflow_block,
            inputs: vec![],
            outputs: vec![],
        }
    }

    /// Add inputs into the ThrillerEngine.
    pub fn add_inputs(&mut self, inputs: Vec<Rc<RegularVar>>) {
        self.inputs.extend(inputs);
    }

    /// Add outputs into the ThrillerEngine.
    pub fn add_outputs(&mut self, outputs: Vec<Rc<RegularVar>>) {
        self.outputs.extend(outputs);
    }

    /// Emit the function signature for the given dataflow block.
    pub(crate) fn emit_function_signature<T: AsRef<str>>(&self, sig: T) -> ThrillerResult<String> {
        let mut code = String::new();
        code += "template<typename Element, typename KeTraits>\n";
        code += format!("__global__ void {}(", sig.as_ref()).as_str();
        // TODO: Add function arguments.
        for (index, input) in self.inputs.iter().enumerate() {
            if index != 0 {
                code += format!(", const Element* {}", input.get_name()).as_str();
            } else {
                code += format!("const Element* {}", input.get_name()).as_str();
            }
        }

        for output in &self.outputs {
            code += format!(", Element* {}", output.get_name()).as_str();
        }
        code += ")";

        Ok(code)
    }

    /// Generate the ThrillerFlow code for the given dataflow block.
    pub fn emit_dataflow<T: AsRef<str>>(&self, sig: T) -> ThrillerResult<String> {
        let mut code = String::new();
        code += self.emit_function_signature(sig)?.as_str();
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
    pub fn persist<T: AsRef<str>>(&self, file_name: &str, sig: T) -> ThrillerResult<()> {
        let code = self.emit_dataflow(sig)?;
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
