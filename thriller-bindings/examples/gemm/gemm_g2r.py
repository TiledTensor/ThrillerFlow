import context

from pythriller import initialize_thriller_flow
from pythriller import Tensor, Layout, TensorType, Graph, Node, Edge
from pythriller import AttachedEdge, Block, IterationVar, AccessMap

if __name__ == '__main__':
    initialize_thriller_flow()

    LayoutA = Layout.RowMajor
    LayoutB = Layout.RowMajor
    LayoutC = Layout.RowMajor

    GlobalLayoutA = Layout.RowMajor
    GlobalLayoutB = Layout.ColMajor
    GlobalLayoutC = Layout.RowMajor

    BufTypeA = TensorType.RegTile
    BufTypeB = TensorType.RegTile
    BufTypeC = TensorType.RegTile

    GlobalTypeA = TensorType.GlobalTile
    GlobalTypeB = TensorType.GlobalTile
    GlobalTypeC = TensorType.GlobalTile

    DimA = [64, 64]
    DimB = [64, 64]
    DimC = [64, 64]

    GlobalDimA = [256, 256]
    GlobalDimB = [256, 256]
    GlobalDimC = [256, 256]

    rA = Tensor("rA", DimA, LayoutA, BufTypeA)
    rB = Tensor("rB", DimB, LayoutB, BufTypeB)
    acc = Tensor("acc", DimC, LayoutC, BufTypeC)

    gA = Tensor("gA", GlobalDimA, GlobalLayoutA, GlobalTypeA)
    gB = Tensor("gB", GlobalDimB, GlobalLayoutB, GlobalTypeB)
    gC = Tensor("gC", GlobalDimC, GlobalLayoutC, GlobalTypeC)

    print(rA)
    print(rB)
    print(acc)

    print(gA)
    print(gB)
    print(gC)

    RegGraph = Graph()

    NodeA = Node(rA)
    NodeB = Node(rB)
    NodeAcc = Node(acc)

    GemmNode = Node.gemm(NodeA, NodeB, NodeAcc)

    LoopIter = IterationVar('i', (0, 4))

    access_dims = [1]

    AccessMap = AccessMap(
        access_dims, [[[1]], [[0]]], [[0], [10]], [LoopIter])

    EdgeA_Gemm = Edge(NodeA, GemmNode)
    EdgeB_GEMM = Edge(NodeB, GemmNode)
    EdgeGemm_Acc = Edge(GemmNode, NodeAcc)

    RegGraph.add_nodes([NodeA, NodeB, NodeAcc, GemmNode])
    RegGraph.add_edges([EdgeA_Gemm, EdgeB_GEMM, EdgeGemm_Acc])

    RegGraph.connect()

    LoadGlobalToRegEdgeA = AttachedEdge(gA, rA, AccessMap)
    LoadGlobalToRegEdgeB = AttachedEdge(gB, rB, AccessMap)
    StoreRegToGlobalEdgeC = AttachedEdge(acc, gC, AccessMap)

    GlobalToRegBlock = Block(
        [LoadGlobalToRegEdgeA, LoadGlobalToRegEdgeB], [StoreRegToGlobalEdgeC], RegGraph, [LoopIter])

    code = GlobalToRegBlock.codegen()

    print("================Codegen=================")
    print(code)
