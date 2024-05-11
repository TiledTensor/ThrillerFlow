use std::rc::Rc;

use thriller_core::{initialize, Buffer, Task};
use thriller_utils::*;

fn main() {
    initialize();

    let s_a = Rc::new(Buffer::new("sA"));
    let s_b = Rc::new(Buffer::new("sB"));
    let s_c = Rc::new(Buffer::new("sC"));

    let rf_gemm_graph = build_gemm_rf_graph(s_a, s_b, s_c);

    let rf_code = rf_gemm_graph.emit().unwrap();

    println!("{}", rf_code);
}
