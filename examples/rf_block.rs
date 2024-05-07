use std::rc::Rc;

use thriller_flow::{
    initialize, AttachedEdge, BlockType, Buffer, MemoryLevel, Task, ThrillerBlock, ThrillerGraph,
};

fn main() {
    initialize();
    let s_a = Rc::new(Buffer::new("sA"));
    let r_a = Rc::new(Buffer::new("rA"));
    let s_b = Rc::new(Buffer::new("sB"));
    let r_b = Rc::new(Buffer::new("rB"));
    let in_edge0 = AttachedEdge::new(s_a, r_a, None);
    let in_edge1 = AttachedEdge::new(s_b, r_b, None);

    let acc = Rc::new(Buffer::new("acc"));
    let s_c = Rc::new(Buffer::new("sC"));
    let out_edge = AttachedEdge::new(acc, s_c, None);

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
