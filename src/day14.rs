use std::collections::HashMap;

#[derive(Debug)]
pub struct Instruction {
	masks: Vec<(u64, u64)>,
	p_mask: u64,
	n_mask: u64,
	mem: Vec<(u64, u64)>,
}

fn mask_from_str(i: &str) -> (u64, u64) {
	let s = i.replace("mask = ", "");
	(
		u64::from_str_radix(s.replace('X', "0").as_str(), 2).unwrap(),
		u64::from_str_radix(s.replace('X', "1").as_str(), 2).unwrap(),
	)
}

pub fn parse_input_1(input: &str) -> Vec<Instruction> {
	let mut lines = input.lines();

	let mut mem = vec![];
	let mut mask = mask_from_str(lines.next().unwrap());

	let mut instructions = vec![];
	loop {
		if let Some(value) = lines.next() {
			if value.starts_with("mask") {
				instructions.push(Instruction {
					masks: vec![],
					p_mask: mask.0,
					n_mask: mask.1,
					mem: mem.clone(),
				});
				mask = mask_from_str(value);
				mem.clear();
				continue;
			}

			let v = value
				.replace("mem[", "")
				.replace("]", "")
				.split(" = ")
				.map(String::from)
				.collect::<Vec<String>>();

			mem.push((v[0].parse::<u64>().unwrap(), v[1].parse::<u64>().unwrap()));
		}
		else {
			break;
		}
	}
	instructions.push(Instruction {
		masks: vec![],
		p_mask: mask.0,
		n_mask: mask.1,
		mem: mem.clone(),
	});

	instructions
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u64 {
	let ins = parse_input_1(input);
	let mut mem = HashMap::new();

	for i in ins {
		for (a, v) in i.mem {
			mem.insert(a, (v | i.p_mask) & i.n_mask);
		}
	}

	mem.values().sum()
}

fn masks_from_str(i: &str) -> Vec<(u64, u64)> {
	let mut masks = vec![];
	let s = i.replace("mask = ", "");
	let c = s.chars().filter(|c| *c == 'X').count() as u32;
	let m = s.chars().rev().collect::<Vec<char>>();

	for i in 0..(2_u32.pow(c) as u32) {
		let mut p = 1;
		let mut index_mask = 1_u64;
		let mut p_mask = 0_u64;
		let mut n_mask = u64::max_value();
		for k in 0..36 {
			if m[k] == 'X' {
				if i & p == p {
					p_mask = p_mask | index_mask;
				}
				else {
					n_mask = n_mask & !index_mask;
				}
				p = p << 1;
			}
			else if m[k] == '1' {
				p_mask = p_mask | index_mask;
			}

			index_mask = index_mask << 1;
		}
		masks.push((p_mask, n_mask));
	}

	masks
}

pub fn parse_input_2(input: &str) -> Vec<Instruction> {
	let mut lines = input.lines();

	let mut mem = vec![];
	let mut masks = masks_from_str(lines.next().unwrap());

	let mut instructions = vec![];
	loop {
		if let Some(value) = lines.next() {
			if value.starts_with("mask") {
				instructions.push(Instruction {
					masks: masks.clone(),
					p_mask: 0,
					n_mask: 0,
					mem: mem.clone(),
				});
				masks = masks_from_str(value);
				mem.clear();
				continue;
			}

			let v = value
				.replace("mem[", "")
				.replace("]", "")
				.split(" = ")
				.map(String::from)
				.collect::<Vec<String>>();

			mem.push((v[0].parse::<u64>().unwrap(), v[1].parse::<u64>().unwrap()));
		}
		else {
			break;
		}
	}
	instructions.push(Instruction {
		masks: masks.clone(),
		p_mask: 0,
		n_mask: 0,
		mem: mem.clone(),
	});

	instructions
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> u64 {
	let ins = parse_input_2(input);
	let mut mem = HashMap::new();

	for i in ins {
		for (target, v) in i.mem {
			for (p_m, n_m) in i.masks.iter() {
				mem.insert((target | *p_m) & *n_m, v);
			}
		}
	}

	mem.values().sum()
}
