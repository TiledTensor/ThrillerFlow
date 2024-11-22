import context

from pythriller import initialize_thriller_flow, Layout, Tensor, TensorType
from pythriller import Graph, Node, Edge, AttachedEdge, IterationVar, AccessMap
from pythriller import Block


if __name__ == '__main__':
    # Initialize runtime.
    initialize_thriller_flow()

    # Define reg layout for A, B, C.
    RegLayoutA = Layout.RowMajor
    RegLayoutB = Layout.RowMajor
    RegLayoutC = Layout.RowMajor

    # Define shared layout for A, B, C.
    SharedLayoutA = Layout.RowMajor
    SharedLayoutB = Layout.ColMajor
    SharedLayoutC = Layout.RowMajor

    # Define global layout for A, B, C.
    GlobalLayoutA = Layout.RowMajor
    GlobalLayoutB = Layout.ColMajor
    GlobalLayoutC = Layout.RowMajor

    # Define Reg Dim for A, B, C.
    RegDimA = [64, 64]
    RegDimB = [64, 64]
    RegDimC = [64, 64]

    # Define Shared Dim for A, B, C.
    SharedDimA = [64, 64]
    SharedDimB = [64, 64]
    SharedDimC = [64, 64]

    # Define Global Dim for A, B, C.
    GlobalDimA = [256, 256]
    GlobalDimB = [256, 256]
    GlobalDimC = [256, 256]

    # Define Reg Tensor for A, B, C.
    rA = Tensor("rA", RegDimA, RegLayoutA, TensorType.RegTile)
    rB = Tensor("rB", RegDimB, RegLayoutB, TensorType.RegTile)
    acc = Tensor("acc", RegDimC, RegLayoutC, TensorType.RegTile)

    # Define Shared Tensor for A, B, C.
    sA = Tensor("sA", SharedDimA, SharedLayoutA, TensorType.SharedTile)
    sB = Tensor("sB", SharedDimB, SharedLayoutB, TensorType.SharedTile)
    sC = Tensor("sC", SharedDimC, SharedLayoutC, TensorType.SharedTile)

    # Define Global Tensor for A, B, C.
    gA = Tensor("gA", GlobalDimA, GlobalLayoutA, TensorType.GlobalTile)
    gB = Tensor("gB", GlobalDimB, GlobalLayoutB, TensorType.GlobalTile)
    gC = Tensor("gC", GlobalDimC, GlobalLayoutC, TensorType.GlobalTile)

    # Define Reg Node for A, B, C.
    NodeRA = Node.tensor(rA)
    NodeRB = Node.tensor(rB)
    NodeRC = Node.tensor(acc)

    # Define Reg GEMM Node.
    RegGemmNode = Node.gemm(NodeRA, NodeRB, NodeRC)

    # Define Reg Edge for A, B, C, GEMM.
    RegEdgeA = Edge(NodeRA, RegGemmNode)
    RegEdgeB = Edge(NodeRB, RegGemmNode)
    RegEdgeC = Edge(RegGemmNode, NodeRC)

    # Define Shared Node for A, B, C.
    NodeSA = Node.tensor(sA)
    NodeSB = Node.tensor(sB)
    NodeSC = Node.tensor(sC)

    # Define Global Node for A, B, C.
    NodeGA = Node.tensor(gA)
    NodeGB = Node.tensor(gB)
    NodeGC = Node.tensor(gC)

    # Define loop iter from shared to register
    LoopIterS2R = IterationVar('j', (0, 1))

    # Define loop iter from global to shared
    LoopIterG2S = IterationVar('i', (0, 4))

    # Build AccessMap from Shared to Register.
    AccessMapSA2RA = AccessMap(
        [0], [[[1]], [[0]]], [[0], [0]], [LoopIterS2R])
    AccessMapSB2RB = AccessMap(
        [0], [[[1]], [[0]]], [[0], [0]], [LoopIterS2R])
    AccessMapRC2SC = AccessMap([0], [[[]], [[]]], [[], []], [])

    # Build AccessMap from Global to Shared.
    AccessMapGA2SA = AccessMap(
        [0], [[[1]], [[0]]], [[0], [0]], [LoopIterG2S])
    AccessMapGB2SB = AccessMap(
        [0], [[[1]], [[0]]], [[0], [0]], [LoopIterG2S])
    AccessMapSC2GC = AccessMap([0], [[[]], [[]]], [[], []], [])

    # Build Attached Edge from Shared to Register.
    AttachedEdgeSA2RA = AttachedEdge(sA, rA, AccessMapSA2RA)
    AttachedEdgeSB2RB = AttachedEdge(sB, rB, AccessMapSB2RB)
    AttachedEdgeSC2RC = AttachedEdge(acc, sC, AccessMapRC2SC)

    # Build Attached Edge from Global to Shared.
    AttachedEdgeGA2SA = AttachedEdge(gA, sA, AccessMapGA2SA)
    AttachedEdgeGB2SB = AttachedEdge(gB, sB, AccessMapGB2SB)
    AttachedEdgeSC2GC = AttachedEdge(sC, gC, AccessMapSC2GC)

    # Build Register Level ETDG.
    RegGraph = Graph()

    # Add Reg Nodes into Reg Graph.
    RegGraph.add_nodes([NodeRA, NodeRB, NodeRC, RegGemmNode])
    # Add Reg Edges into Reg Graph.
    RegGraph.add_edges([RegEdgeA, RegEdgeB, RegEdgeC])
    # Connect Reg Graph.
    RegGraph.connect()

    # Print codegen for Reg Graph.
    reg_code = RegGraph.codegen()

    # Build Block for Shared to Register.
    SharedToRegBlock = Block(
        [AttachedEdgeSA2RA, AttachedEdgeSB2RB], [AttachedEdgeSC2RC], RegGraph, [LoopIterS2R])

    # Print codegen for Shared to Register Block.
    shared_to_reg_code = SharedToRegBlock.codegen()

    # Define BlockNode for SharedToRegBlock
    SharedBlockNode = Node.block(SharedToRegBlock)

    # Define Edge for SA, SB, SC, SharedBlockNode.
    EdgeSA2Block = Edge(NodeSA, SharedBlockNode)
    EdgeSB2Block = Edge(NodeSB, SharedBlockNode)
    EdgeBlock2SC = Edge(SharedBlockNode, NodeSC)

    # Build Shared Level ETDG.
    SharedGraph = Graph()
    # Add Shared Nodes into Shared Graph.
    SharedGraph.add_nodes([NodeSA, NodeSB, NodeSC, SharedBlockNode])
    # Add Shared Edges into Shared Graph.
    SharedGraph.add_edges([EdgeSA2Block, EdgeSB2Block, EdgeBlock2SC])
    # Connect Shared Graph.
    SharedGraph.connect()

    allocate_vars = SharedGraph.allocate_var()
    print(allocate_vars)
