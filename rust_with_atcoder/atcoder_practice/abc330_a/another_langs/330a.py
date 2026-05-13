import sys


def main() -> None:
    data = list(map(int, sys.stdin.buffer.read().split()))
    n = data[0]
    l = data[1]
    a_list = data[2:2 + n]

    count = sum(1 for a in a_list if a >= l)
    print(count)


if __name__ == "__main__":
    main()
