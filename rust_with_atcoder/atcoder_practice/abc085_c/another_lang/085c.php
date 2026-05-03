<?php

fscanf(STDIN, "%d %d", $n, $y);

for ($a = 0; $a <= $n; $a++) {
    for ($b = 0; $b <= $n - $a; $b++) {
        $c = $n - $a - $b;
        if (10000 * $a + 5000 * $b + 1000 * $c === $y) {
            echo "$a $b $c" . PHP_EOL;
            exit(0);
        }
    }
}

echo "-1 -1 -1" . PHP_EOL;
