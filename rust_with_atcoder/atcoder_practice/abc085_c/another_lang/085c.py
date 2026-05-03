import sys


def main() -> None:
    n, y = map(int, sys.stdin.read().split())

    for a in range(n + 1):
        for b in range(n - a + 1):
            c = n - a - b
            if 10000 * a + 5000 * b + 1000 * c == y:
                print(f"{a} {b} {c}")
                return

    print("-1 -1 -1")


if __name__ == "__main__":
    main()
