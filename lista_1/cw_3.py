def find_k(x, y):
    m = len(y)
    result = 0
    for k in range(min(len(x),m)):
        print(f"k={k}, x={x[:k]}, y={y[-k:]}")
        if x[:k] == y[-k:]:
            result = k

    return result

if __name__ == "__main__":
    x = "abafaskjdhfkasjhdfkjah"
    y = "asdkjshdkjfhasdkaba"
    print(find_k(x, y))
