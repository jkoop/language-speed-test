<?php 

function factorsOfN(int $n): int {
	$factors = 1;
	$sqrtN = sqrt($n);
	$n2 = (int) $sqrtN;

	for ($i = 1; $i <= $n2; $i++) {
		if ($n % $i == 0) {
			$factors++;
		}
	}

	$factors *= 2;

	if ($sqrtN == $n2) {
		$factors--;
	}

	return $factors;
}

$number = 0;
$maxFactors = 0;
$factors;

for ($i = 0; true; $i++) {
	$number += $i;
	$factors = factorsOfN($number);

	if ($factors > $maxFactors) {
		$maxFactors = $factors;
		echo($number . ' ' . $factors . "\n");
	}

	if ($factors > 500) {
		break;
	}
}
