
import context

import pythriller

if __name__ == '__main__':
    pythriller.initialize_thriller_flow()

    LayoutA = pythriller.PyLayout.RowMajor
    LayoutB = pythriller.PyLayout.ColMajor
    LayoutC = pythriller.PyLayout.RowMajor

    BufTypeA = pythriller.PyBufType.GlobalTile
    BufTypeB = pythriller.PyBufType.GlobalTile
    BufTypeC = pythriller.PyBufType.GlobalTile

    DimA = [256, 256]
    DimB = [256, 256]
    DimC = [256, 256]

    gA = pythriller.create_buffer("gA", DimA, LayoutA, BufTypeA)
    gB = pythriller.create_buffer("gB", DimB, LayoutB, BufTypeB)
    gC = pythriller.create_buffer("gC", DimC, LayoutC, BufTypeC)

    print(gA)
    print(gB)
    print(gC)
