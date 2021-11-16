#include <math.h>
#include <stdio.h>
#include <stdlib.h>

int factorsOfN(int n) {
	int factors = 1;
	double sqrtN = sqrt(n);
	int n2 = (int) sqrtN;

	for (int i = 1; i <= n2; i++) {
		if (n % i == 0) {
			factors++;
		}
	}

	factors *= 2;

	if (sqrtN == n2) {
		factors--;
	}

	return factors;
}

void main() {
	int number = 0;
	int maxFactors = 0;
	int factors;

	for (int i = 0; 1; i++) {
		number += i;
		factors = factorsOfN(number);

		if (factors > maxFactors) {
			maxFactors = factors;
			printf("%i %i\n", number, factors);
		}

		if (factors > 500) {
			break;
		}
	}
}
