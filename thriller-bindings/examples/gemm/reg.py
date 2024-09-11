import context

import pythriller

if __name__ == '__main__':
    pythriller.initialize_thriller_flow()

    LayoutA = pythriller.PyLayout.RowMajor
    LayoutB = pythriller.PyLayout.RowMajor
    LayoutC = pythriller.PyLayout.RowMajor

    GlobalLayoutA = pythriller.PyLayout.RowMajor
    GlobalLayoutB = pythriller.PyLayout.ColMajor
    GlobalLayoutC = pythriller.PyLayout.RowMajor

    BufTypeA = pythriller.PyBufType.RegTile
    BufTypeB = pythriller.PyBufType.RegTile
    BufTypeC = pythriller.PyBufType.RegTile

    GlobalTypeA = pythriller.PyBufType.GlobalTile
    GlobalTypeB = pythriller.PyBufType.GlobalTile
    GlobalTypeC = pythriller.PyBufType.GlobalTile

    DimA = [64, 64]
    DimB = [64, 64]
    DimC = [64, 64]

    GlobalDimA = [256, 256]
    GlobalDimB = [256, 256]
    GlobalDimC = [256, 256]

    rA = pythriller.PyBuffer("rA", DimA, LayoutA, BufTypeA)
    rB = pythriller.PyBuffer("rB", DimB, LayoutB, BufTypeB)
    acc = pythriller.PyBuffer("acc", DimC, LayoutC, BufTypeC)

    gA = pythriller.PyBuffer("gA", GlobalDimA, GlobalLayoutA, GlobalTypeA)
    gB = pythriller.PyBuffer("gB", GlobalDimB, GlobalLayoutB, GlobalTypeB)
    gC = pythriller.PyBuffer("gC", GlobalDimC, GlobalLayoutC, GlobalTypeC)

    print(rA)
    print(rB)
    print(acc)

    print(gA)
    print(gB)
    print(gC)

    MemoryLevel = pythriller.PyMemoryLevel.Register
    RegGraph = pythriller.PyGraph(MemoryLevel)

    NodeA = pythriller.PyNode(rA)
    NodeB = pythriller.PyNode(rB)
    NodeAcc = pythriller.PyNode(acc)

    GemmNode = pythriller.PyNode.gemm(NodeA, NodeB, NodeAcc)

    EdgeA_Gemm = pythriller.PyEdge(NodeA, GemmNode)
    EdgeB_GEMM = pythriller.PyEdge(NodeB, GemmNode)
    EdgeGemm_Acc = pythriller.PyEdge(GemmNode, NodeAcc)

    RegGraph.add_nodes([NodeA, NodeB, NodeAcc, GemmNode])
    RegGraph.add_edges([EdgeA_Gemm, EdgeB_GEMM, EdgeGemm_Acc])

    RegGraph.connect()

    LoadGlobalToRegEdgeA = pythriller.AttachedEdge(gA, rA)
    LoadGlobalToRegEdgeB = pythriller.AttachedEdge(gB, rB)
    StoreRegToGlobalEdgeC = pythriller.AttachedEdge(acc, gC)
    G2RBlockMemLevel = pythriller.PyMemoryLevel.Register

    G2RBlockType = pythriller.BlockType.Reduce

    GlobalToRegBlock = pythriller.Block(
        [LoadGlobalToRegEdgeA, LoadGlobalToRegEdgeB], [StoreRegToGlobalEdgeC], G2RBlockMemLevel, RegGraph, G2RBlockType)

    code = GlobalToRegBlock.codegen()

    print("================Codegen=================")
    print(code)
