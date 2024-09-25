'''
Back-to-Back GEMM example.
'''
import context

from pythriller import initialize_thriller_flow, Layout, Tensor, TensorType
from pythriller import Graph, Node, Edge, AttachedEdge, IterationVar, AccessMap
from pythriller import Block

if __name__ == '__main__':
    # Initialize the Thriller flow runtime.
    initialize_thriller_flow()

    # Define reg layout for A, B, C, D, acc.
    RegLayoutA = Layout.RowMajor
    RegLayoutB = Layout.RowMajor
    RegLayoutC = Layout.RowMajor
    RegLayoutD = Layout.RowMajor
    RegLayoutAcc = Layout.RowMajor

    # Define shared layout for A, B, C, D.
    SharedLayoutA = Layout.RowMajor
    SharedLayoutB = Layout.ColMajor
    SharedLayoutC = Layout.ColMajor
    SharedLayoutD = Layout.RowMajor

    # Define global layout for A, B, C, D.
    GlobalLayoutA = Layout.RowMajor
    GlobalLayoutB = Layout.ColMajor
    GlobalLayoutC = Layout.ColMajor
    GlobalLayoutD = Layout.RowMajor

    # Define Reg Dim for A, B, C, D, acc.
    RegDimA = [64, 64]
    RegDimB = [64, 64]
    RegDimC = [64, 64]
    RegDimD = [64, 64]
    RegDimAcc = [64, 64]

    # Define Shared Dim for A, B, C, D.
    SharedDimA = [64, 64]
    SharedDimB = [64, 64]
    SharedDimC = [64, 64]
    SharedDimD = [64, 64]

    # Define Global Dim for A, B, C, D.
    GlobalDimA = [256, 256]
    GlobalDimB = [256, 256]
    GlobalDimC = [256, 256]
    GlobalDimD = [256, 256]

    # Define Reg Tensor for A, B, C, D, acc.
    rA = Tensor("rA", RegDimA, RegLayoutA, TensorType.RegTile)
    rB = Tensor("rB", RegDimB, RegLayoutB, TensorType.RegTile)
    rC = Tensor("rC", RegDimC, RegLayoutC, TensorType.RegTile)
    rD = Tensor("rD", RegDimD, RegLayoutD, TensorType.RegTile)
    rAcc = Tensor("rAcc", RegDimAcc, RegLayoutAcc, TensorType.RegTile)

    # Define Shared Tensor for A, B, C, D.
    sA = Tensor("sA", SharedDimA, SharedLayoutA, TensorType.SharedTile)
    sB = Tensor("sB", SharedDimB, SharedLayoutB, TensorType.SharedTile)
    sC = Tensor("sC", SharedDimC, SharedLayoutC, TensorType.SharedTile)
    sD = Tensor("sD", SharedDimD, SharedLayoutD, TensorType.SharedTile)

    # Define Global Tensor for A, B, C, D.
    gA = Tensor("gA", GlobalDimA, GlobalLayoutA, TensorType.GlobalTile)
    gB = Tensor("gB", GlobalDimB, GlobalLayoutB, TensorType.GlobalTile)
    gC = Tensor("gC", GlobalDimC, GlobalLayoutC, TensorType.GlobalTile)
    gD = Tensor("gD", GlobalDimD, GlobalLayoutD, TensorType.GlobalTile)

    # Define Reg Node for A, B, C, D, acc.
    NodeRA = Node.tensor(rA)
    NodeRB = Node.tensor(rB)
    NodeRC = Node.tensor(rC)
    NodeRD = Node.tensor(rD)
    NodeRAcc = Node.tensor(rAcc)
    # Define A, B, Acc to GEMM Node.
    RegABGemmCNode = Node.gemm(NodeRA, NodeRB, NodeRAcc)

    # Define Shared Node for A, B, C, D.
    NodeSA = Node.tensor(sA)
    NodeSB = Node.tensor(sB)
    NodeSC = Node.tensor(sC)
    NodeSD = Node.tensor(sD)

    # Define Global Node for A, B, C, D.
    NodeGA = Node.tensor(gA)
    NodeGB = Node.tensor(gB)
    NodeGC = Node.tensor(gC)
    NodeGD = Node.tensor(gD)

    # Define Edge for A, B, Acc, Gemm
    RegEdgeA = Edge(NodeRA, RegABGemmCNode)
    RegEdgeB = Edge(NodeRB, RegABGemmCNode)
    RegEdgeAcc = Edge(NodeRAcc, RegABGemmCNode)

    # Define iteration variable for A, B, Acc, Gemm loop.
    # Iterate over the register tiles along the kTK dimension.
    IterVarI = IterationVar("i", (0, 1))

    # Iterate over K.
    IterVarK = IterationVar("k", (0, 1))

    # Iterator over N.
    IterVarN = IterationVar("n", (0, 4))

    # Build AccessMap from sA, sB load into rA, rB.
    AccessMapSA2RA = AccessMap([0], [[[1]], [[0]]], [[0], [0]], [IterVarI])
    AccessMapSB2RB = AccessMap([0], [[[1]], [[0]]], [[0], [0]], [IterVarI])

    # Build AccessMap from sC load into rC.
    AccessMapSC2RC = AccessMap([0], [[[1]], [[0]]], [[0], [0]], [IterVarK])

    # Build AccessMap from gA, gB load into sA, sB.
    AccessMapGA2SA = AccessMap([0], [[[1]], [[0]]], [[0], [0]], [IterVarK])
    AccessMapGB2SB = AccessMap([0], [[[1, 0], [0, 1]], [[0, 0]]], [
                               [0, 0], [0]], [IterVarK, IterVarN])

    # Build AccessMap from gC load into sC.
    AccessMapGC2SC = AccessMap([0], [[[1]], [[0]]], [[0], [0]], [IterVarN])

    # Build AccessMap from rAcc store into sD.
    AccessMapRAcc2GD = AccessMap([0], [[[1]], [[0]]], [[0], [0]], [])

    # Build AccessMap from sD store into gD.
    AccessMapSD2GD = AccessMap([0], [[[1]], [[0]]], [[0], [0]], [])

    # Build Attached Edge for load sA, sB into rA, rB.
    AttachedEdgeSA2RA = AttachedEdge(sA, rA, AccessMapSA2RA)
    AttachedEdgeSB2RB = AttachedEdge(sB, rB, AccessMapSB2RB)

    # Build Attached Edge for load sC into rC.
    AttachedEdgeSC2RC = AttachedEdge(sC, rC, AccessMapSC2RC)

    # Build Attached Edge for load gA, gB into sA, sB.
    AttachedEdgeGA2SA = AttachedEdge(gA, sA, AccessMapGA2SA)
    AttachedEdgeGB2SB = AttachedEdge(gB, sB, AccessMapGB2SB)

    # Build Attached Edge for load gC into sC.
    AttachedEdgeGC2SC = AttachedEdge(gC, sC, AccessMapGC2SC)

    # Build Attached Edge for store rAcc into sD.
    # AttachedEdgeRAcc2SD = AttachedEdge(rAcc, sD, AccessMapRAcc2GD)

    # Build Attached Edge for store sD into gD.
    AttachedEdgeSD2GD = AttachedEdge(sD, gD, AccessMapSD2GD)

    # Build rA, rB, Acc, Gemm Graph.
    RegABGemmGraph = Graph()

    # Add Nodes to the Graph.
    RegABGemmGraph.add_nodes([NodeRA, NodeRB, NodeRAcc, RegABGemmCNode])
    # Add Edges to the Graph.
    RegABGemmGraph.add_edges([RegEdgeA, RegEdgeB, RegEdgeAcc])
    # Connect the Graph.
    RegABGemmGraph.connect()

    # Print codegen for Reg Graph.
    print(RegABGemmGraph.codegen())

    # Build Block for adding attached edge sA, sB into rA, rB.
    BlockRegABGemm = Block(
        [AttachedEdgeSA2RA, AttachedEdgeSB2RB], [], RegABGemmGraph, [IterVarI])
    # Print codegen for Block.
    print(BlockRegABGemm.codegen())

    # Define Block Node for `BlockRegABGemm`.
    BlockRegABGemmNode = Node.block(BlockRegABGemm)

    # Build Graph for BlockRegABGemm.
    BlockRegABGemmGraph = Graph()
    # Add Nodes to the Graph.
    BlockRegABGemmGraph.add_nodes([BlockRegABGemmNode])

    # Connect the Graph.
    BlockRegABGemmGraph.connect()
    # Print codegen for BlockRegABGemmGraph.
    print(BlockRegABGemmGraph.codegen())

    # Build Block for adding attached edge gA, gB into sA, sB.
    BlockSharedABGemm = Block(
        [AttachedEdgeGA2SA, AttachedEdgeGB2SB, AttachedEdgeSC2RC], [], BlockRegABGemmGraph, [IterVarK])
    # Print codegen for Block.
    print(BlockSharedABGemm.codegen())

    # Build Node for `BlockSharedABGemm`.
    BlockSharedABGemmNode = Node.block(BlockSharedABGemm)

    # Build Graph for BlockSharedABGemm.
    BlockSharedABGemmGraph = Graph()
    # Add Nodes to the Graph.
    BlockSharedABGemmGraph.add_nodes([BlockSharedABGemmNode])

    # Connect the Graph.
    BlockSharedABGemmGraph.connect()
    # Print codegen for BlockSharedABGemmGraph.
    print(BlockSharedABGemmGraph.codegen())

    # Build Block for adding attached edge gC into sC.
    BlockSharedCGemm = Block(
        [AttachedEdgeGC2SC], [AttachedEdgeSD2GD], BlockSharedABGemmGraph, [IterVarN])

    # Print codegen for Block.
    print(BlockSharedCGemm.codegen())
