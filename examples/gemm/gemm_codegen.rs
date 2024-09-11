use std::{cell::RefCell, rc::Rc};

use thriller_core::{
    initialize, BlockLayout, BlockShape, BlockType, MemoryLevel, RegularVar, ThrillerBlock,
    ThrillerEngine, ThrillerGraph, ThrillerNode, ThrillerNodeInner,
};
use thriller_utils::{BufBuilder, ThrillerUtils};

fn main() {
    initialize();

    let g_a = Rc::new(BufBuilder::row_major_global_tile("gA", &[256, 256]));
    let g_b = Rc::new(BufBuilder::col_major_global_tile("gB", &[256, 256]));
    let g_c = Rc::new(BufBuilder::row_major_global_tile("gC", &[256, 256]));

    let shared_block =
        ThrillerUtils::build_shared_gemm_block(g_a.clone(), g_b.clone(), g_c.clone());
    let shared_block = Rc::new(shared_block);

    let shared_block_node = Rc::new(RefCell::new(ThrillerNode::new(ThrillerNodeInner::Block(
        shared_block.clone(),
    ))));

    let mut global_graph = ThrillerGraph::new(MemoryLevel::Global);

    global_graph.add_nodes(vec![shared_block_node.clone()]);

    let global_graph = Rc::new(RefCell::new(global_graph));

    let global_block = ThrillerBlock::new(
        vec![],
        vec![],
        MemoryLevel::Global,
        global_graph.clone(),
        BlockType::Map,
    );

    let mut engine = ThrillerEngine::new(global_block);

    let var_a = Rc::new(RegularVar::new(String::from("A")));
    let var_b = Rc::new(RegularVar::new(String::from("B")));
    let var_c = Rc::new(RegularVar::new(String::from("C")));

    let block_layout_a = Rc::new(BlockLayout::new([
        BlockShape::Num(1),
        BlockShape::Num(1),
        BlockShape::Num(1),
    ]));

    let block_layout_b = Rc::new(BlockLayout::new([
        BlockShape::Num(1),
        BlockShape::Num(1),
        BlockShape::Num(1),
    ]));

    let block_layout_c = Rc::new(BlockLayout::new([
        BlockShape::Num(1),
        BlockShape::Num(1),
        BlockShape::Num(1),
    ]));

    engine.add_inputs(vec![
        (var_a.clone(), g_a.clone()),
        (var_b.clone(), g_b.clone()),
    ]);
    engine.add_outputs(vec![(var_c.clone(), g_c.clone())]);

    engine.add_input_blocks(vec![block_layout_a, block_layout_b]);
    engine.add_output_blocks(vec![block_layout_c]);

    let code = engine.emit_dataflow("thriller_gemm").unwrap();
    println!("{}", code);

    let repo_dir = engine.install_library().unwrap();
    println!("Library installed at: {}", repo_dir);

    engine
        .persist(
            format!("{}/{}", repo_dir, "src/kernels/thriller_gemm.cu"),
            "thriller_gemm".to_string(),
        )
        .unwrap();
}
