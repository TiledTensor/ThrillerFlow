use std::rc::Rc;

use thriller_flow::{
    initialize, AccessMap, AccessMatrix, AccessOffset, AttachedEdge, BlockType, Buffer,
    IterationBound, IterationVar, MemoryLevel, Task, ThrillerBlock, ThrillerGraph,
};

fn main() {
    initialize();
    let s_a = Rc::new(Buffer::new("sA"));
    let r_a = Rc::new(Buffer::new("rA"));
    let s_b = Rc::new(Buffer::new("sB"));
    let r_b = Rc::new(Buffer::new("rB"));
    let mut in_edge0 = AttachedEdge::new(s_a, r_a, None);
    let mut in_edge1 = AttachedEdge::new(s_b, r_b, None);

    let acc = Rc::new(Buffer::new("acc"));
    let s_c = Rc::new(Buffer::new("sC"));
    let out_edge = AttachedEdge::new(acc, s_c, None);

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

    let block = ThrillerBlock::new(
        vec![Rc::new(in_edge0), Rc::new(in_edge1)],
        vec![Rc::new(out_edge)],
        MemoryLevel::Register,
        Rc::new(ThrillerGraph::new(MemoryLevel::Register)),
        BlockType::Map,
    );

    let code = block.emit().unwrap();

    println!("{}", code);
}
