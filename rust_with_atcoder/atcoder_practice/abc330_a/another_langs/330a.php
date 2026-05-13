<?php
$tokens = preg_split('/\s+/', trim(stream_get_contents(STDIN)));
$tokens = array_map('intval', $tokens);

$n = $tokens[0];
$l = $tokens[1];
$aList = array_slice($tokens, 2, $n);

$count = 0;
foreach ($aList as $a) {
    if ($a >= $l) {
        $count++;
    }
}

echo $count . PHP_EOL;
