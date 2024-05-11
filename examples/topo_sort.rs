use std::vec;
use std::{cell::RefCell, rc::Rc};

use thriller_flow::{
    initialize, AccessMap, AccessMatrix, AccessOffset, Buffer, Gemm, IterationBound, IterationVar,
    MemoryLevel, ThrillerEdge, ThrillerGraph, ThrillerNode, ThrillerNodeInner,
};

fn main() {
    initialize();
    let r_a = Rc::new(Buffer::new("rA"));
    let r_b = Rc::new(Buffer::new("rB"));

    let acc = Rc::new(Buffer::new("acc"));

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

    let access_map = Rc::new(access_map);

    let mut subgraph = ThrillerGraph::new(MemoryLevel::Register);

    let r_a_node = Rc::new(RefCell::new(ThrillerNode::new(ThrillerNodeInner::Buffer(
        r_a.clone(),
    ))));
    let r_b_node = Rc::new(RefCell::new(ThrillerNode::new(ThrillerNodeInner::Buffer(
        r_b.clone(),
    ))));
    let acc_node = Rc::new(RefCell::new(ThrillerNode::new(ThrillerNodeInner::Buffer(
        acc.clone(),
    ))));

    let gemm = Gemm::new(
        vec![r_a_node.clone(), r_b_node.clone()],
        acc_node.clone(),
        access_map.clone(),
    );

    let gemm_node = Rc::new(RefCell::new(ThrillerNode::new(ThrillerNodeInner::Op(
        Box::new(gemm),
    ))));

    let ra_gemm_edge = ThrillerEdge::new(r_a_node.clone(), gemm_node.clone());
    let rb_gemm_edge = ThrillerEdge::new(r_b_node.clone(), gemm_node.clone());
    let gemm_acc_edge = ThrillerEdge::new(gemm_node.clone(), acc_node.clone());

    let ra_gemm_edge_ref = Rc::new(ra_gemm_edge);
    let rb_gemm_edge_ref = Rc::new(rb_gemm_edge);
    let gemm_acc_edge_ref = Rc::new(gemm_acc_edge);

    subgraph.add_nodes(vec![
        r_a_node.clone(),
        r_b_node.clone(),
        acc_node.clone(),
        gemm_node.clone(),
    ]);
    subgraph.add_edges(vec![
        ra_gemm_edge_ref.clone(),
        rb_gemm_edge_ref.clone(),
        gemm_acc_edge_ref.clone(),
    ]);

    subgraph.connect();

    let sort_nodes = subgraph.topo_sort();

    for node in sort_nodes {
        println!("Node: {:?}", node.borrow().get_node_name());
    }
}
