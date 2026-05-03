<?php

$k = intval(trim(fgets(STDIN)));
$ans_sum = 0;

for ($a = 1; $a <= $k; $a++) {
	for ($b = 1; $b <= $k; $b++) {
		for ($c = 1; $c <= $k; $c++) {
			$ans_sum += gcd(gcd($a, $b), $c);
		}
	}
}

echo $ans_sum . PHP_EOL;

function gcd(int $a, int $b): int {
	if ($b === 0) {
		return $a;
	}
	return gcd($b, $a % $b);
}

