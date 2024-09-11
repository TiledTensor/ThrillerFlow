use std::vec;
use std::{cell::RefCell, rc::Rc};

use thriller_core::{
    initialize, AccessMap, AccessMatrix, AccessOffset, AttachedEdge, BlockType, Gemm,
    IterationBound, IterationVar, MemoryLevel, Task, ThrillerBlock, ThrillerEdge, ThrillerGraph,
    ThrillerNode, ThrillerNodeInner,
};

use thriller_utils::BufBuilder;

fn main() {
    initialize();
    let s_a = Rc::new(BufBuilder::row_major_shared_tile("sA", &[256, 256]));
    let r_a = Rc::new(BufBuilder::row_major_reg_tile("rA", &[64, 64]));
    let s_b = Rc::new(BufBuilder::col_major_shared_tile("sB", &[256, 256]));
    let r_b = Rc::new(BufBuilder::row_major_reg_tile("rB", &[64, 64]));
    let mut in_edge0 = AttachedEdge::new(s_a, r_a.clone(), None);
    let mut in_edge1 = AttachedEdge::new(s_b, r_b.clone(), None);

    let acc = Rc::new(BufBuilder::row_major_reg_tile("rC", &[64, 64]));
    let s_c = Rc::new(BufBuilder::row_major_shared_tile("sC", &[256, 256]));
    let out_edge = AttachedEdge::new(acc.clone(), s_c, None);

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

    in_edge0.replace_access_map(access_map.clone());
    in_edge1.replace_access_map(access_map.clone());

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

    let mut block = ThrillerBlock::new(
        vec![Rc::new(in_edge0), Rc::new(in_edge1)],
        vec![Rc::new(out_edge)],
        MemoryLevel::Register,
        Rc::new(RefCell::new(subgraph)),
        BlockType::Reduce,
    );

    block.merge_access_map();

    let code = block.emit().unwrap();

    println!("{}", code);
}
