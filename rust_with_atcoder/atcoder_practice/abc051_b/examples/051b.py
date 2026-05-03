import sys


def main() -> None:
    k, s = map(int, sys.stdin.read().split())

    count = 0

    for x in range(k + 1):
        for y in range(k + 1):
            z = s - x - y
            if 0 <= z <= k:
                count += 1

    print(count)


if __name__ == "__main__":
    main()
