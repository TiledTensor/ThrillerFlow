
import context
import thriller_flow

import pythriller

if __name__ == '__main__':
    thriller_flow.initialize_thriller_flow()
    g_a = pythriller.create_buffer("g_a")
    g_b = pythriller.create_buffer("g_b")
    g_c = pythriller.create_buffer("g_c")

    print(g_a, g_b, g_c)
