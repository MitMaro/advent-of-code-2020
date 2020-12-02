use std::str::FromStr;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
	input.lines().map(|v| i32::from_str(v).unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<i32>) -> i32 {
	for (index, a) in input.iter().enumerate() {
		for b in input.iter().skip(index) {
			if a + b == 2020 {
				return a * b;
			}
		}
	}
	unreachable!();
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<i32>) -> i32 {
	for (index_a, a) in input.iter().enumerate() {
		for (index_b, b) in input.iter().skip(index_a).enumerate() {
			for c in input.iter().skip(index_a + index_b) {
				if a + b + c == 2020 {
					return a * b * c;
				}
			}
		}
	}
	unreachable!();
}
