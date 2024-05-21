use std::env::current_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::rc::Rc;

// use thriller_kernels::Memory;
use crate::kernels::memory::Memory;

use crate::{RegularVar, Task, ThrillerBlock, ThrillerError, ThrillerResult, Var};

mod layout;

pub use layout::{BlockLayout, BlockShape};

/// `ThrillerEngine` is the main entry point for the ThrillerFlow framework.
pub struct ThrillerEngine {
    dataflow_block: ThrillerBlock,
    inputs: Vec<Rc<RegularVar>>,
    outputs: Vec<Rc<RegularVar>>,
    input_blocks: Vec<Rc<BlockLayout>>,
}

impl ThrillerEngine {
    /// Create a new ThrillerEngine with the given dataflow block.
    pub fn new(dataflow_block: ThrillerBlock) -> Self {
        ThrillerEngine {
            dataflow_block,
            inputs: vec![],
            outputs: vec![],
            input_blocks: vec![],
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

    /// Add input blocks into the ThrillerEngine.
    pub fn add_input_blocks(&mut self, input_blocks: Vec<Rc<BlockLayout>>) {
        self.input_blocks.extend(input_blocks);
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

    /// Emit include headers for the dataflow code.
    pub fn emit_header(&self) -> ThrillerResult<String> {
        let mut code = String::new();
        // code += "#pragma once\n";
        // code += "#include \"cuda_utils.hpp\"\n";
        // code += "#include <torch/script.h>\n";
        code += "#include \"cell/mod.hpp\"\n";
        code += "#include \"layout.hpp\"\n";
        code += "\n\n";
        Ok(code)
    }

    /// Generate the ThrillerFlow code for the given dataflow block.
    pub fn emit_dataflow<T: AsRef<str>>(&self, sig: T) -> ThrillerResult<String> {
        let mut code = String::new();
        code += self.emit_function_signature(sig)?.as_str();

        code += "{\n";
        code += "// Declare shared memory buffer\n";
        code += Memory::emit_shared_buf_decl().as_str();
        code += "\n";

        // println!("Inner buffers: {:?}", self.dataflow_block.get_inner_bufs());

        // Add block layouts mappings
        assert!(self.input_blocks.len() == self.inputs.len());
        // assert!(self.dataflow_block.get_inner_bufs().len() == self.inputs.len());

        for (input, input_block) in self.inputs.iter().zip(self.input_blocks.iter()) {
            // code += format!("auto {} = {};", input.get_name(), input_block.get_name()).as_str();
            code += format!(
                "Element* {} = const_cast<Element*>({}) + blockIdx.x * {} + blockIdx.y * {} + blockIdx.z * {};\n",
                "todo",
                input.get_name(),
                input_block.get_dim_x(),
                input_block.get_dim_y(),
                input_block.get_dim_z()
            )
            .as_str();
        }

        code += "// Emit dataflow code.\n";
        code += self.dataflow_block.emit()?.as_str();
        code += "}\n";
        Ok(code)
    }

    /// Persist the generated ThrillerFlow code to the given file.
    pub fn persist<T: AsRef<str>>(&self, file_name: T, sig: T) -> ThrillerResult<()> {
        let mut code = self.emit_header()?;
        code += "namespace tiledcuda::kernels {\n\n";
        code += self.emit_dataflow(sig)?.as_str();
        code += "\n}  // namespace tiledcuda::kernels\n";

        let file_name = Path::new(file_name.as_ref());
        let mut file = File::create(file_name).unwrap();
        // file.write_all(code.as_bytes()).unwrap();
        file.write_all(code.as_bytes())
            .map_err(|_| ThrillerError::FailedFileOp)?;

        Ok(())
    }

    /// Install macro kernel for the given dataflow block.
    pub fn install_library(&self) -> ThrillerResult<String> {
        let temp_dir = current_dir().unwrap();
        std::env::set_current_dir(&temp_dir).map_err(|_| ThrillerError::FailedFileOp)?;

        let repo_dir = temp_dir.join("TiledCUDA");

        if !repo_dir.exists() {
            let clone = Command::new("git")
                .args(["clone", "git@github.com:TiledTensor/TiledCUDA.git"])
                .output()
                .map_err(|_| ThrillerError::FailedFileOp)?;

            if !clone.status.success() {
                return Err(ThrillerError::FailedFileOp);
            }

            std::env::set_current_dir(repo_dir.clone()).map_err(|_| ThrillerError::FailedFileOp)?;

            let submodule = Command::new("git")
                .args(["submodule", "update", "--init", "--recursive"])
                .output()
                .map_err(|_| ThrillerError::FailedFileOp)?;

            if !submodule.status.success() {
                return Err(ThrillerError::FailedFileOp);
            }
        }

        let repo_path = repo_dir
            .as_path()
            .to_str()
            .map(|s| s.to_string())
            .ok_or(ThrillerError::FailedFileOp)?;

        Ok(repo_path)
    }
}
