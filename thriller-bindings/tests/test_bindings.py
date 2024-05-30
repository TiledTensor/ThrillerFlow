
import context

import pythriller

if __name__ == '__main__':
    pythriller.initialize_thriller_flow()
    g_a = pythriller.create_buffer("g_a")
    g_b = pythriller.create_buffer("g_b")
    g_c = pythriller.create_buffer("g_c")

    print(g_a)
    print(g_b)
    print(g_c)
