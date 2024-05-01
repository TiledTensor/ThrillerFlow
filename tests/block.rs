use std::rc::Rc;

use thriller_flow::{
    initialize, AttachedEdge, BlockType, Buffer, MemoryLevel, Task, ThrillerBlock, ThrillerGraph,
};

#[test]
fn test_block() {
    initialize();

    let s_a = Rc::new(Buffer::new("sA"));
    let r_a = Rc::new(Buffer::new("rA"));
    let in_edge = AttachedEdge::new(s_a, r_a, None);

    let acc = Rc::new(Buffer::new("acc"));
    let s_c = Rc::new(Buffer::new("sC"));
    let out_edge = AttachedEdge::new(acc, s_c, None);

    let block = ThrillerBlock::new(
        vec![Rc::new(in_edge)],
        vec![Rc::new(out_edge)],
        MemoryLevel::Register,
        Rc::new(ThrillerGraph::new(MemoryLevel::Register)),
        BlockType::Map,
    );

    let code = block.emit();

    println!("{}", code);
}
