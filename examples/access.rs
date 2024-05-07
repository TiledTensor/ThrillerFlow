use std::{rc::Rc, vec};

use thriller_flow::{
    initialize, AccessMap, AccessMatrix, AccessOffset, IterationVar, ThrillerError, ThrillerResult,
    Var,
};

fn main() {
    initialize();

    /*
     * for(i1 = 0; i1 < M; i1++){
     *  for(i2 = 0; i2 < N; i2++){
     *     for(i3 = 0; i3 < K; i3++){
     *    }
     *  }
     * }
     */

    let iter_var1 = Rc::new(IterationVar::new("i1", (0, 10)));
    let iter_var2 = Rc::new(IterationVar::new("i2", (0, 10)));
    let iter_var3 = Rc::new(IterationVar::new("i3", (0, 10)));

    let mut access_map = AccessMap::new(3, vec![2, 2, 2]);
    access_map.add_iter_var(iter_var1);
    access_map.add_iter_var(iter_var2);
    access_map.add_iter_var(iter_var3);

    access_map.add_access_matrix(AccessMatrix(vec![vec![1, 0, 0], vec![0, 0, 1]]));
    access_map.add_access_matrix(AccessMatrix(vec![vec![0, 0, 1], vec![0, 1, 0]]));
    access_map.add_access_matrix(AccessMatrix(vec![vec![1, 0, 0], vec![0, 1, 0]]));

    access_map.add_access_offset(AccessOffset(vec![0, 1]));
    access_map.add_access_offset(AccessOffset(vec![1, 0]));
    access_map.add_access_offset(AccessOffset(vec![0, 0]));

    let mul_add_op = |access_matrixs: &Vec<AccessMatrix>,
                      access_offsets: &Vec<AccessOffset>|
     -> ThrillerResult<String> {
        if access_matrixs.len() != 3 || access_offsets.len() != 3 {
            return Err(ThrillerError::InvalidAccessPattern);
        }

        let mut access_codes = vec![String::new(); 3];
        let iter_vars = access_map.get_iter_vars();

        for (i, matrix) in access_matrixs.iter().enumerate() {
            for (j, access) in matrix.0.iter().enumerate() {
                if access[0] != 0 {
                    access_codes[i] += format!(
                        "[{access} * {iter_var} + {offset}]",
                        access = access[0],
                        iter_var = iter_vars[j].get_name(),
                        offset = access_offsets[i].0[j]
                    )
                    .as_str();
                }

                if access[1] != 0 {
                    access_codes[i] += format!(
                        "[{access} * {iter_var} + {offset}]",
                        access = access[1],
                        iter_var = iter_vars[j].get_name(),
                        offset = access_offsets[i].0[j]
                    )
                    .as_str();
                }

                if access[2] != 0 {
                    access_codes[i] += format!(
                        "[{access} * {iter_var} + {offset}]",
                        access = access[2],
                        iter_var = iter_vars[j].get_name(),
                        offset = access_offsets[i].0[j]
                    )
                    .as_str();
                }
            }
        }

        let mut code = String::new();

        code += format!(
            "C{c} += A{a} * B{b};\n",
            c = access_codes[2],
            a = access_codes[0],
            b = access_codes[1]
        )
        .as_str();

        Ok(code)
    };

    let code = access_map.gen_loop(mul_add_op).unwrap();

    println!("{}", code);
}
