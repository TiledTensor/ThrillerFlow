'''
Whole GEMM is an example of GEMM that utilized all mempry hierarchies
of NVIDIA GPU.
'''
import context

from pythriller import initialize_thriller_flow, PyLayout, PyBuffer


if __name__ == '__main__':
    # Initialize runtime.
    initialize_thriller_flow()

    # Define layout for A, B, C.
    LayoutA = PyLayout.RowMajor
    LayoutB = PyLayout.ColMajor
    LayoutC = PyLayout.RowMajor

    # Build Register Level ETDG.
