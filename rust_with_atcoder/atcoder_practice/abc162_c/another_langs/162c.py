import sys


def gcd(a: int, b: int) -> int:
	if b == 0:
		return a
	return gcd(b, a % b)


def main() -> None:
	k = int(sys.stdin.readline().strip())
	ans_sum = 0

	for a in range(1, k + 1):
		for b in range(1, k + 1):
			for c in range(1, k + 1):
				ans_sum += gcd(gcd(a, b), c)

	print(ans_sum)


if __name__ == "__main__":
	main()

