use std::{cell::RefCell, rc::Rc};

use thriller_core::{
    initialize, AccessMap, AccessMatrix, AccessOffset, Gemm, IterationBound, IterationVar, Task,
    ThrillerNode, ThrillerNodeInner,
};

use thriller_utils::BufBuilder;

fn main() {
    initialize();

    let iter_var = Rc::new(IterationVar::new(
        "i",
        (IterationBound::Fixed(0), IterationBound::Fixed(10)),
    ));

    let mut access_map = AccessMap::new(1, vec![1]);
    access_map.add_iter_var(iter_var);

    access_map.add_access_matrix(AccessMatrix(vec![vec![1]]));
    access_map.add_access_matrix(AccessMatrix(vec![vec![1]]));

    access_map.add_access_offset(AccessOffset(vec![0]));
    access_map.add_access_offset(AccessOffset(vec![0]));

    let buf_a = Rc::new(BufBuilder::row_major_reg_tile("rA", &[64, 64]));
    let buf_b = Rc::new(BufBuilder::row_major_reg_tile("rB", &[64, 64]));
    let buf_acc = Rc::new(BufBuilder::row_major_reg_tile("rC", &[64, 64]));

    let node_a = Rc::new(RefCell::new(ThrillerNode::new(ThrillerNodeInner::Buffer(
        buf_a.clone(),
    ))));
    let node_b = Rc::new(RefCell::new(ThrillerNode::new(ThrillerNodeInner::Buffer(
        buf_b.clone(),
    ))));
    let node_acc = Rc::new(RefCell::new(ThrillerNode::new(ThrillerNodeInner::Buffer(
        buf_acc.clone(),
    ))));
    let gemm = Gemm::new(
        vec![node_a.clone(), node_b.clone()],
        node_acc.clone(),
        Rc::new(access_map),
    );
    let gemm_node = Rc::new(ThrillerNode::new(ThrillerNodeInner::Op(Box::new(gemm))));

    let gemm_code = gemm_node.emit().unwrap();

    println!("{}", gemm_code);
}
