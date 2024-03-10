import sys


def fa_matcher(T, P):
    delta_dictionary = {}

    solution = []
    m = len(P)
    q = 0

    for (i, a) in enumerate(T):
        q = fa_delta(P, q, a, m, delta_dictionary)

        if q == m:
            solution.append(i - m + 1)

    return solution


def fa_delta(P, q, a, m, delta_dictionary):
    if (q, a) in delta_dictionary:
        return delta_dictionary[(q, a)]

    k = min(m, q + 1)

    while not (P[:q] + a).endswith(P[:k]):
        k = k - 1

    delta_dictionary[(q, a)] = k

    return k


def main():
    if (len(sys.argv) < 3):
        return -1

    pattern = sys.argv[1]
    string = sys.argv[2]

    print(pattern, string)

    sol = fa_matcher(string, pattern)

    print(sol)


if __name__ == "__main__":
    main()
