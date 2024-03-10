def horner(poly, n, x):

    result = poly[0]

    # ((2x – 6)x + 2)x – 1
    for i in range(1, n):
        result = result*x + poly[i]

    return result


def main():
    # 2x3 - 6x2 + 2x - 1 for x = 3
    poly = [2, -6, 2, -1]
    x = 3
    n = len(poly)

    print("Value of polynomial is ", horner(poly, n, x))


if __name__ == "__main__":
    main()
