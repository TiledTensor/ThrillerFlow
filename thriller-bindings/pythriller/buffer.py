from .context import PyBuffer, PyLayout, PyBufType


def create_buffer(name, dim, layout, buf_type):
    return PyBuffer(name, dim, layout, buf_type)
