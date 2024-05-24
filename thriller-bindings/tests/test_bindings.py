import thriller_flow
from thriller_flow import PyBuffer


if __name__ == '__main__':
    thriller_flow.initialize_thriller_flow()
    g_a = PyBuffer("g_a")
    g_b = PyBuffer("g_b")
    g_c = PyBuffer("g_c")

    print(g_a, g_b, g_c)
