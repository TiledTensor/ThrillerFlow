use std::rc::Rc;

use thriller_flow::{
    initialize, AccessMap, AccessMatrix, AccessOffset, Buffer, Gemm, IterationVar, ThrillerEdge,
    ThrillerNode, ThrillerNodeInner,
};

fn main() {
    initialize();
    let gemm = Gemm;
    let buf_a = Rc::new(Buffer::new("rA"));
    let buf_b = Rc::new(Buffer::new("rB"));
    let buf_acc = Rc::new(Buffer::new("acc"));

    let node_a = Rc::new(ThrillerNode::new(ThrillerNodeInner::Buffer(buf_a.clone())));
    let node_b = Rc::new(ThrillerNode::new(ThrillerNodeInner::Buffer(buf_b.clone())));
    let node_acc = Rc::new(ThrillerNode::new(ThrillerNodeInner::Buffer(
        buf_acc.clone(),
    )));
    let gemm_node = Rc::new(ThrillerNode::new(ThrillerNodeInner::Op(Box::new(gemm))));

    let edge_a = Rc::new(ThrillerEdge::new(node_a.clone(), gemm_node.clone()));
    let edge_b = Rc::new(ThrillerEdge::new(node_b.clone(), gemm_node.clone()));
    let edge_acc = Rc::new(ThrillerEdge::new(gemm_node.clone(), node_acc.clone()));

    let iter_var = Rc::new(IterationVar::new("i", (0, 10)));

    let mut access_map = AccessMap::new(1, vec![1]);
    access_map.add_iter_var(iter_var);

    access_map.add_access_matrix(AccessMatrix(vec![vec![1]]));
    access_map.add_access_matrix(AccessMatrix(vec![vec![1]]));

    access_map.add_access_offset(AccessOffset(vec![0]));
    access_map.add_access_offset(AccessOffset(vec![0]));

    let gemm_code = gemm
        .emit_(&[edge_a, edge_b], &edge_acc, &access_map)
        .unwrap();

    println!("{}", gemm_code);
}
