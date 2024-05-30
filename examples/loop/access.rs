use std::{rc::Rc, vec};

use thriller_core::{
    initialize, AccessMap, AccessMatrix, AccessOffset, IterationBound, IterationVar, ThrillerError,
    ThrillerResult, Var,
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

    let iter_var1 = Rc::new(IterationVar::new(
        "i1",
        (IterationBound::Fixed(0), IterationBound::Fixed(10)),
    ));
    let iter_var2 = Rc::new(IterationVar::new(
        "i2",
        (IterationBound::Fixed(0), IterationBound::Fixed(10)),
    ));
    let iter_var3 = Rc::new(IterationVar::new(
        "i3",
        (IterationBound::Fixed(0), IterationBound::Fixed(10)),
    ));

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

    let mul_add_op = |access_map: &AccessMap| -> ThrillerResult<String> {
        let access_matrixs = access_map.get_access_matrixs();
        let access_offsets = access_map.get_access_offsets();
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
                        offset = access_offsets[0].0[j]
                    )
                    .as_str();
                }

                if access[1] != 0 {
                    access_codes[i] += format!(
                        "[{access} * {iter_var} + {offset}]",
                        access = access[1],
                        iter_var = iter_vars[j].get_name(),
                        offset = access_offsets[1].0[j]
                    )
                    .as_str();
                }

                if access[2] != 0 {
                    access_codes[i] += format!(
                        "[{access} * {iter_var} + {offset}]",
                        access = access[2],
                        iter_var = iter_vars[2].get_name(),
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

    let mul_add_code = mul_add_op(&access_map).unwrap();

    let code = access_map.gen_loop_access(mul_add_code).unwrap();

    println!("{}", code);
}
