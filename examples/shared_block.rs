use std::vec;
use std::{cell::RefCell, rc::Rc};

use thriller_core::{
    initialize, AccessMap, AccessMatrix, AccessOffset, AttachedEdge, BlockType, Buffer,
    IterationBound, IterationVar, MemoryLevel, Task, ThrillerBlock, ThrillerEdge, ThrillerGraph,
    ThrillerNode, ThrillerNodeInner,
};
use thriller_utils::*;

fn main() {
    initialize();

    let iter_var = Rc::new(IterationVar::new(
        "j",
        (IterationBound::Fixed(0), IterationBound::Fixed(10)),
    ));

    let mut access_map = AccessMap::new(1, vec![1]);
    access_map.add_iter_var(iter_var);

    access_map.add_access_matrix(AccessMatrix(vec![vec![1]]));
    access_map.add_access_matrix(AccessMatrix(vec![vec![1]]));

    access_map.add_access_offset(AccessOffset(vec![0]));
    access_map.add_access_offset(AccessOffset(vec![0]));

    let access_map = Rc::new(access_map);

    let s_a = Rc::new(Buffer::new("sA"));
    let s_b = Rc::new(Buffer::new("sB"));
    let s_c = Rc::new(Buffer::new("sC"));
    let g_a = Rc::new(Buffer::new("gA"));
    let g_b = Rc::new(Buffer::new("gB"));
    let g_c = Rc::new(Buffer::new("gC"));

    let in_edge0 = AttachedEdge::new(g_a.clone(), s_a.clone(), Some(access_map.clone()));
    let in_edge1 = AttachedEdge::new(g_b.clone(), s_b.clone(), Some(access_map.clone()));
    let out_edge = AttachedEdge::new(g_c.clone(), s_c.clone(), None);

    let rf_gemm_graph = build_gemm_rf_graph(s_a.clone(), s_b.clone(), s_c.clone());

    // let rf_code = rf_gemm_graph.emit().unwrap();

    // println!("{}", rf_code);

    let s_a_node = Rc::new(RefCell::new(ThrillerNode::new(ThrillerNodeInner::Buffer(
        s_a.clone(),
    ))));

    let s_b_node = Rc::new(RefCell::new(ThrillerNode::new(ThrillerNodeInner::Buffer(
        s_b.clone(),
    ))));

    let s_c_node = Rc::new(RefCell::new(ThrillerNode::new(ThrillerNodeInner::Buffer(
        s_c.clone(),
    ))));

    let rf_block_node = Rc::new(RefCell::new(ThrillerNode::new(ThrillerNodeInner::Block(
        Rc::new(rf_gemm_graph),
    ))));

    let sa_block_edge = Rc::new(ThrillerEdge::new(s_a_node.clone(), rf_block_node.clone()));

    let sb_block_edge = Rc::new(ThrillerEdge::new(s_b_node.clone(), rf_block_node.clone()));

    let block_sc_edge = Rc::new(ThrillerEdge::new(rf_block_node.clone(), s_c_node.clone()));

    let mut subgraph = ThrillerGraph::new(MemoryLevel::Shared);

    subgraph.add_nodes(vec![
        s_a_node.clone(),
        s_b_node.clone(),
        s_c_node.clone(),
        rf_block_node.clone(),
    ]);

    subgraph.add_edges(vec![
        sa_block_edge.clone(),
        sb_block_edge.clone(),
        block_sc_edge.clone(),
    ]);

    subgraph.connect();

    let mut shared_block = ThrillerBlock::new(
        vec![Rc::new(in_edge0), Rc::new(in_edge1)],
        vec![Rc::new(out_edge)],
        MemoryLevel::Shared,
        Rc::new(subgraph),
        BlockType::Map,
    );

    shared_block.merge_access_map();

    let code = shared_block.emit().unwrap();

    println!("{}", code);
}
