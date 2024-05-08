use crate::{AccessMap, ThrillerResult, Var};

#[allow(dead_code)]
/// Compute Type.
pub enum Compute {
    Gemm,
    Axpby,
}

#[allow(dead_code)]
pub(crate) fn gen_gemm(access_map: &AccessMap) -> ThrillerResult<String> {
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
        "cute::gemm(mma, {a}, {b}, {c});\n",
        a = access_codes[0],
        b = access_codes[1],
        c = access_codes[2],
    )
    .as_str();

    Ok(code)
}
