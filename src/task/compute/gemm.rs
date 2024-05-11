use std::rc::Rc;

use crate::{AccessMap, Task, ThrillerError, ThrillerNode, ThrillerResult, Var};

/// Gemm is a task that performs matrix multiplication.
// #[derive(Clone, Copy)]
pub struct Gemm {
    // inputs: Vec<Rc<ThrillerEdge>>,
    // output: Rc<ThrillerEdge>,
    prevs: Vec<Rc<ThrillerNode>>,
    next: Rc<ThrillerNode>,
    access_map: AccessMap,
}

impl Gemm {
    /// Create a new GEMM task.
    pub fn new(
        prevs: Vec<Rc<ThrillerNode>>,
        next: Rc<ThrillerNode>,
        access_map: AccessMap,
    ) -> Self {
        Gemm {
            prevs,
            next,
            access_map,
        }
    }
}

impl Task for Gemm {
    fn emit(&self) -> ThrillerResult<String> {
        if self.prevs.len() != 2 {
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
                buf_a = self.prevs[0].get_name(),
                buf_b = self.prevs[1].get_name(),
                buf_c = self.next.get_name()
            )
            .as_str();

            Ok(code)
        };

        code += self.access_map.gen_loop_access(gemm)?.as_str();

        Ok(code)
    }
}
