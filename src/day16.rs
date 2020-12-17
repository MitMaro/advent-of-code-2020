use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Field {
	value: u32,
	valid: bool,
}

#[derive(Debug)]
pub struct Rule {
	name: String,
	ranges: Vec<RangeInclusive<u32>>,
}

#[derive(Debug)]
pub struct Notes {
	rules: Vec<Rule>,
	ticket: Vec<u64>,
	other_tickets: Vec<Vec<Field>>,
}

pub enum ParseState {
	Rules,
	Yours,
	Others,
}

pub fn parse_input(input: &str) -> Notes {
	let mut state = ParseState::Rules;
	let mut notes = Notes {
		rules: vec![],
		ticket: vec![],
		other_tickets: vec![],
	};

	for line in input.lines() {
		if line.is_empty() {
			continue;
		}
		match state {
			ParseState::Rules => {
				if line == "your ticket:" {
					state = ParseState::Yours;
					continue;
				}
				let mut p = line.split(": ");

				notes.rules.push(Rule {
					name: String::from(p.next().unwrap()),
					ranges: p
						.next()
						.unwrap()
						.split(" or ")
						.map(|s| {
							let mut r = s.split("-").map(|v| v.parse::<u32>().unwrap());
							r.next().unwrap()..=r.next().unwrap()
						})
						.collect::<Vec<RangeInclusive<u32>>>(),
				});
			},
			ParseState::Yours => {
				if line == "nearby tickets:" {
					state = ParseState::Others;
					continue;
				}
				notes.ticket = line.split(",").map(|v| v.parse::<u64>().unwrap()).collect::<Vec<u64>>();
			},
			ParseState::Others => {
				notes.other_tickets.push(
					line.split(",")
						.map(|v| {
							Field {
								value: v.parse::<u32>().unwrap(),
								valid: false,
							}
						})
						.collect::<Vec<Field>>(),
				);
			},
		}
	}

	notes
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> u32 {
	let mut notes = parse_input(input);

	let mut error_rate = 0;

	for ticket in notes.other_tickets.iter_mut() {
		for f in ticket.iter_mut() {
			let mut valid = false;
			for rule in &notes.rules {
				for r in &rule.ranges {
					if r.contains(&f.value) {
						valid = true;
					}
				}
			}
			if !valid {
				error_rate += f.value;
			}
			f.valid = valid;
		}
	}

	error_rate
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> u64 {
	let mut notes = parse_input(input);

	for ticket in notes.other_tickets.iter_mut() {
		for f in ticket.iter_mut() {
			let mut valid = false;
			for rule in &notes.rules {
				for r in &rule.ranges {
					if r.contains(&f.value) {
						valid = true;
					}
				}
			}
			f.valid = valid;
		}
	}
	let mut cdi = vec![];

	let tickets = notes
		.other_tickets
		.iter()
		.filter_map(|v| {
			if !v.iter().all(|v| v.valid) {
				None
			}
			else {
				Some(v.iter().map(|v| v.value).collect::<Vec<u32>>())
			}
		})
		.collect::<Vec<Vec<u32>>>();

	for i in 0..tickets[0].len() {
		let mut cd = vec![];
		for rule in &notes.rules {
			let v = tickets.iter().all(|t| rule.ranges.iter().any(|r| r.contains(&t[i])));
			if v {
				cd.push(rule.name.clone());
			}
		}
		cdi.push((i, cd));
	}

	let mut map = HashMap::new();

	loop {
		let mut done = true;
		for (i, c) in &cdi {
			let v = c
				.iter()
				.filter(|v| !map.contains_key(*v))
				.map(String::from)
				.collect::<Vec<String>>();

			if v.len() == 1 {
				done = false;
				map.insert(v[0].clone(), *i);
			}
		}

		if done {
			break;
		}
	}

	let mut total: u64 = 1;
	for (name, i) in map {
		if name.starts_with("departure") {
			total *= notes.ticket[i];
		}
	}

	total
}
