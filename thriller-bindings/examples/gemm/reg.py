import context

import pythriller

if __name__ == '__main__':
    pythriller.initialize_thriller_flow()

    LayoutA = pythriller.PyLayout.RowMajor
    LayoutB = pythriller.PyLayout.ColMajor
    LayoutC = pythriller.PyLayout.RowMajor

    BufTypeA = pythriller.PyBufType.RegTile
    BufTypeB = pythriller.PyBufType.RegTile
    BufTypeC = pythriller.PyBufType.RegTile

    DimA = [64, 64]
    DimB = [64, 64]
    DimC = [64, 64]

    rA = pythriller.PyBuffer("rA", DimA, LayoutA, BufTypeA)
    rB = pythriller.PyBuffer("rB", DimB, LayoutB, BufTypeB)
    acc = pythriller.PyBuffer("acc", DimC, LayoutC, BufTypeC)

    print(rA)
    print(rB)
    print(acc)

    print("================Codegen=================")

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
    code = RegGraph.codegen()

    print(code)
