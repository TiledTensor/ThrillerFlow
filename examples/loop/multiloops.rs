use std::rc::Rc;

use thriller_core::{
    initialize, AccessMap, AccessMatrix, AccessOffset, AttachedEdge, BlockType, Buffer,
    IterationBound, IterationVar, MemoryLevel, ThrillerBlock, ThrillerGraph,
};

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

    let s_a = Rc::new(Buffer::new("sA"));
    let r_a = Rc::new(Buffer::new("rA"));
    let s_b = Rc::new(Buffer::new("sB"));
    let r_b = Rc::new(Buffer::new("rB"));

    let in_edge0 = AttachedEdge::new(s_a, r_a.clone(), Some(access_map1.clone()));
    let in_edge1 = AttachedEdge::new(s_b, r_b.clone(), Some(access_map2.clone()));

    let subgraph = ThrillerGraph::new(MemoryLevel::Register);

    let mut block = ThrillerBlock::new(
        vec![Rc::new(in_edge0), Rc::new(in_edge1)],
        vec![],
        MemoryLevel::Register,
        Rc::new(subgraph),
        BlockType::Reduce,
    );

    block.merge_loops();

    // let code = block.emit().unwrap();

    // println!("{}", code);
}
