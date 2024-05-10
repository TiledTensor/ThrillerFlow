use std::rc::Rc;

use crate::{AccessMap, Task, ThrillerEdge, ThrillerError, ThrillerResult, Var};

/// Gemm is a task that performs matrix multiplication.
#[derive(Clone, Copy)]
pub struct Gemm;

impl Gemm {
    /// Generate the GEMM task.
    pub fn emit_(
        &self,
        inputs: &[Rc<ThrillerEdge>],
        output: &Rc<ThrillerEdge>,
        access_map: &AccessMap,
    ) -> ThrillerResult<String> {
        if inputs.len() != 2 {
            return Err(ThrillerError::WrongInputsNum);
        }

        let mut code = String::new();

        let gemm = |access_map: &AccessMap| -> ThrillerResult<String> {
            let mut code = String::new();

            let access_matrixs = access_map.get_access_matrixs();
            let access_offsets = access_map.get_access_offsets();
            let iter_vars = access_map.get_iter_vars();
            // (A, B, C)
            let mut access_codes = vec![String::new(); 3];

            for (i, matrix) in access_matrixs.iter().enumerate() {
                for (j, access) in matrix.0.iter().enumerate() {
                    for (iter_index, &access_index) in access.iter().enumerate() {
                        if access_index != 0 {
                            let iter_var = iter_vars[iter_index].get_name();
                            access_codes[i] += format!(
                                "[{access} * {iter_var} + {offset}]",
                                access = access_index,
                                iter_var = iter_var,
                                offset = access_offsets[0].0[j]
                            )
                            .as_str();
                        }
                    }
                }
            }

            code += format!(
                "cute::gemm(mma, {buf_a}{a}, {buf_b}{b}, {buf_c}{c});\n",
                a = access_codes[0],
                b = access_codes[1],
                c = access_codes[2],
                buf_a = inputs[0].get_src_name(),
                buf_b = inputs[1].get_src_name(),
                buf_c = output.get_dst_name()
            )
            .as_str();

            Ok(code)
        };

        code += access_map.gen_loop_access(gemm)?.as_str();

        Ok(code)
    }
}

impl Task for Gemm {
    fn emit(&self) -> ThrillerResult<String> {
        todo!()
    }
}
