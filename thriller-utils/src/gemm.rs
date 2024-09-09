use std::cell::RefCell;
use std::rc::Rc;

use thriller_core::{
    AccessMap, AccessMatrix, AccessOffset, AttachedEdge, BlockType, Buffer, Gemm, IterationBound,
    IterationVar, MemoryLevel, ThrillerBlock, ThrillerEdge, ThrillerGraph, ThrillerNode,
    ThrillerNodeInner,
};

use crate::BufBuilder;

use crate::ThrillerUtils;

impl ThrillerUtils {
    /// Build a RF level GEMM graph.
    pub fn build_gemm_rf_block(s_a: Rc<Buffer>, s_b: Rc<Buffer>, s_c: Rc<Buffer>) -> ThrillerBlock {
        let r_a = Rc::new(BufBuilder::row_major_reg_tile("rA", &[64, 64]));
        let r_b = Rc::new(BufBuilder::col_major_reg_tile("rB", &[64, 64]));

        let mut in_edge0 = AttachedEdge::new(s_a, r_a.clone(), None);
        let mut in_edge1 = AttachedEdge::new(s_b, r_b.clone(), None);

        let acc = Rc::new(BufBuilder::row_major_reg_tile("acc", &[64, 64]));

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
            Rc::new(subgraph),
            BlockType::Reduce,
        );

        block.merge_access_map();

        block
    }

    /// Build a shared level GEMM graph.
    pub fn build_shared_gemm_block(
        g_a: Rc<Buffer>,
        g_b: Rc<Buffer>,
        g_c: Rc<Buffer>,
    ) -> ThrillerBlock {
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

        let s_a = Rc::new(BufBuilder::row_major_shared_tile("sA", &[256, 256]));
        let s_b = Rc::new(BufBuilder::col_major_shared_tile("sB", &[256, 256]));
        let s_c = Rc::new(BufBuilder::row_major_shared_tile("sC", &[256, 256]));

        let in_edge0 = AttachedEdge::new(g_a.clone(), s_a.clone(), Some(access_map.clone()));
        let in_edge1 = AttachedEdge::new(g_b.clone(), s_b.clone(), Some(access_map.clone()));
        let out_edge = AttachedEdge::new(g_c.clone(), s_c.clone(), None);

        let rf_gemm_graph =
            ThrillerUtils::build_gemm_rf_block(s_a.clone(), s_b.clone(), s_c.clone());

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

        shared_block
    }
}
