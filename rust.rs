fn factors_of_n(n: i32) -> i32 {
	let mut factors: i32 = 1;
	let sqrt_n: f32 = (n as f32).sqrt();
	let n2: i32 = sqrt_n as i32;

	for i in 1..=n2 {
		if n % i == 0 {
			factors += 1;
		}
	}

	factors *= 2;

	if sqrt_n == n2 as f32 {
		factors -= 1;
	}

	return factors;
}

fn main() -> () { // () is like void
	let mut number: i32 = 0;
	let mut max_factors: i32 = 0;
	let mut factors: i32;
	let mut i: i32 = -1;

	loop {
		i += 1;
		number += i;
		factors = factors_of_n(number);

		if factors > max_factors {
			max_factors = factors;
			println!("{} {}", number, factors);
		}

		if factors > 500 {
			break;
		}
	}
}
