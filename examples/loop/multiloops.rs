use std::{cell::RefCell, rc::Rc};

use thriller_core::{
    initialize, AccessMap, AccessMatrix, AccessOffset, AttachedEdge, BlockType, IterationBound,
    IterationVar, MemoryLevel, Task, ThrillerBlock, ThrillerGraph,
};

use thriller_utils::BufBuilder;

fn main() {
    initialize();

    let iter_var1 = Rc::new(IterationVar::new(
        "i1",
        (IterationBound::Fixed(0), IterationBound::Fixed(10)),
    ));

    let iter_var2 = Rc::new(IterationVar::new(
        "i2",
        (IterationBound::Fixed(0), IterationBound::Fixed(20)),
    ));

    let mut access_map1 = AccessMap::new(1, vec![1]);
    access_map1.add_iter_var(iter_var1);

    access_map1.add_access_matrix(AccessMatrix(vec![vec![1]]));
    access_map1.add_access_matrix(AccessMatrix(vec![vec![1]]));

    access_map1.add_access_offset(AccessOffset(vec![0]));
    access_map1.add_access_offset(AccessOffset(vec![0]));

    let access_map1 = Rc::new(access_map1);

    let mut access_map2 = AccessMap::new(1, vec![1]);
    access_map2.add_iter_var(iter_var2);

    access_map2.add_access_matrix(AccessMatrix(vec![vec![1]]));
    access_map2.add_access_matrix(AccessMatrix(vec![vec![1]]));

    access_map2.add_access_offset(AccessOffset(vec![0]));
    access_map2.add_access_offset(AccessOffset(vec![0]));

    let access_map2 = Rc::new(access_map2);

    let s_a = Rc::new(BufBuilder::row_major_shared_tile("sA", &[256, 256]));
    let r_a = Rc::new(BufBuilder::row_major_reg_tile("sB", &[64, 64]));
    let s_b = Rc::new(BufBuilder::col_major_shared_tile("sC", &[256, 256]));
    let r_b = Rc::new(BufBuilder::row_major_reg_tile("rB", &[64, 64]));

    let in_edge0 = AttachedEdge::new(s_a.clone(), r_a.clone(), Some(access_map1.clone()));
    let in_edge1 = AttachedEdge::new(s_b.clone(), r_b.clone(), Some(access_map2.clone()));
    let in_edge2 = AttachedEdge::new(s_a.clone(), r_b.clone(), Some(access_map1.clone()));

    let subgraph = ThrillerGraph::new(MemoryLevel::Register);

    let mut block = ThrillerBlock::new(
        vec![Rc::new(in_edge0), Rc::new(in_edge1), Rc::new(in_edge2)],
        vec![],
        MemoryLevel::Register,
        Rc::new(RefCell::new(subgraph)),
        BlockType::Reduce,
    );

    block.merge_loops();

    let code = block.emit().unwrap();

    println!("{}", code);
}
