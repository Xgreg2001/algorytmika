BASE = 256
MOD = 101


def RabinKarp(string, pattern):
    n = len(string)
    m = len(pattern)

    hpattern = first_hash(pattern)
    hs = first_hash(string[0:m])

    if hs == hpattern:
        if string[0:m] == pattern:
            return 0

    for i in range(1, n-m):
        hs = rolling_hash(hs, string[i - 1], string[i + m - 1], m)

        if hs == hpattern:
            if string[i:i+m] == pattern:
                return i

    return None


def first_hash(string):
    result = ord(string[0])

    for c in string[1:]:
        result = ((result * BASE) % 101 + ord(c)) % MOD

    return result


def rolling_hash(prev_hash, remove, add, length):
    left_base_offset = 1

    for _ in range(length-1):
        left_base_offset = (left_base_offset * BASE) % MOD

    return ((prev_hash + MOD - ord(remove) * left_base_offset) * BASE + ord(add)) % MOD


def main():
    pat = "aba"
    s = "hgdafklsdgabafashkjdfa"

    assert (first_hash("abr") == 4)
    assert (first_hash("bra") == 30)
    assert (rolling_hash(first_hash("abr"), 'a', 'a', 3))

    print("Pattern found at:", RabinKarp(s, pat))


if __name__ == "__main__":
    main()
