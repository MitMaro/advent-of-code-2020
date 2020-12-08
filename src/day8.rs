#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Op {
	Nop,
	Acc,
	Jmp,
}

#[derive(Debug)]
pub struct Instruction {
	pub op: Op,
	pub call_count: i32,
	pub argument: i32,
}

impl Instruction {
	pub fn new(line: &str) -> Self {
		let i = line.split(' ').collect::<Vec<&str>>();

		let op = match i[0] {
			"nop" => Op::Nop,
			"acc" => Op::Acc,
			"jmp" => Op::Jmp,
			_ => unreachable!(),
		};

		Self {
			op,
			call_count: 0,
			argument: i[1].parse::<i32>().unwrap(),
		}
	}
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
	input.lines().map(Instruction::new).collect::<Vec<Instruction>>()
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i32 {
	let mut ptr = 0;
	let mut acc = 0;
	let mut ins = parse_input(input);

	loop {
		let mut i = &mut ins[ptr as usize];
		if i.call_count > 0 {
			break;
		}

		i.call_count += 1;

		match i.op {
			Op::Nop => ptr += 1,
			Op::Acc => {
				acc += i.argument;
				ptr += 1;
			},
			Op::Jmp => ptr += i.argument,
		}
	}
	acc
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i32 {
	let mut acc = 0;
	let mut ins = parse_input(input);

	'r: for change in 0..ins.len() {
		let mut ptr: i32 = 0;
		acc = 0;
		for mut i in &mut ins {
			i.call_count = 0;
		}
		if ins[change].op == Op::Acc {
			continue;
		}

		loop {
			if ptr as usize == ins.len() {
				break 'r;
			}
			let mut i = &mut ins[ptr as usize];
			let mut op = i.op;
			if ptr as usize == change {
				match op {
					Op::Nop => {
						op = Op::Jmp;
					},
					Op::Jmp => {
						op = Op::Nop;
					},
					_ => unreachable!(),
				}
			}

			if i.call_count > 0 {
				continue 'r;
			}

			i.call_count += 1;

			match op {
				Op::Nop => ptr += 1,
				Op::Acc => {
					acc += i.argument;
					ptr += 1;
				},
				Op::Jmp => ptr += i.argument,
			}
		}
	}
	acc
}
