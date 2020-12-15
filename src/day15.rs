use std::collections::HashMap;

pub fn parse_input(input: &str) -> HashMap<u32, (u32, u32)> {
	let nums = input
		.split(",")
		.map(|v| v.parse::<u32>().unwrap())
		.collect::<Vec<u32>>();
	let mut h = HashMap::new();
	for (r, n) in nums.iter().enumerate() {
		h.insert(*n, (r as u32, 0));
	}
	h
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> u32 {
	let mut numbers = parse_input(input);

	let mut next = 0;
	for round in numbers.len()..(2020 - 1) {
		let m = numbers.contains_key(&next);
		let e = numbers.entry(next).or_insert((0, 0));
		e.1 = e.0;
		e.0 = round as u32;
		if m {
			next = e.0 - e.1;
		}
		else {
			next = 0;
		}
	}

	next
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> u32 {
	let mut numbers = parse_input(input);

	let mut next = 0;
	for round in numbers.len()..(30000000 - 1) {
		let m = numbers.contains_key(&next);
		let e = numbers.entry(next).or_insert((0, 0));
		e.1 = e.0;
		e.0 = round as u32;
		if m {
			next = e.0 - e.1;
		}
		else {
			next = 0;
		}
	}

	next
}
