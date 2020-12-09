use itertools::Itertools;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<u64> {
	input.lines().map(|v| v.parse::<u64>().unwrap()).collect::<Vec<u64>>()
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<u64>) -> u64 {
	let mut start = 25;

	's: while start < input.len() {
		for v in input.iter().skip(start - 25).take(25).combinations(2) {
			if v[0] + v[1] == input[start] {
				start += 1;
				continue 's;
			}
		}
		break 's;
	}
	input[start]
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<u64>) -> u64 {
	let target = part1(input);

	's: for i in 0..input.len() {
		let mut sum = input[i];
		for j in i + 1..input.len() {
			sum += input[j];

			if sum == target {
				return input.iter().skip(i).take(j - i).min().unwrap()
					+ input.iter().skip(i).take(j - i).max().unwrap();
			}
			else if sum > target {
				continue 's;
			}
		}
	}
	unreachable!();
}
