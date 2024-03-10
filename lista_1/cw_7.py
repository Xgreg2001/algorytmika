def RabinKarp(string, pattern):
    n = len(string)
    m = len(pattern)

    hpattern = hash(pattern)

    for i in range(n-m):

        hs = hash(string[i:i+m])

        if hs == hpattern:
            if string[i:i+m] == pattern:
                return i

    return None


def main():
    pat = "aba"
    s = "hgdafklsdgabafashkjdfa"

    print("Pattern found at:", RabinKarp(s, pat))


if __name__ == "__main__":
    main()
