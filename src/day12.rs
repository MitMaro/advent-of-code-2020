#[derive(Debug)]
pub enum Action {
	North,
	South,
	East,
	West,
	Left,
	Right,
	Forward,
}

#[derive(Debug)]
pub struct Instruction {
	action: Action,
	amount: i32,
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
	input
		.lines()
		.map(|v| {
			let mut it = v.chars();
			let action = match it.next().unwrap() {
				'N' => Action::North,
				'S' => Action::South,
				'E' => Action::East,
				'W' => Action::West,
				'L' => Action::Left,
				'R' => Action::Right,
				'F' => Action::Forward,
				_ => unreachable!(),
			};

			let amount = it.collect::<String>().parse::<i32>().unwrap();

			Instruction { action, amount }
		})
		.collect::<Vec<Instruction>>()
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> i32 {
	let instructions = parse_input(input);
	let mut angle = 90;
	let mut x = 0;
	let mut y = 0;

	for inst in instructions {
		match inst.action {
			Action::North => y += inst.amount,
			Action::South => y -= inst.amount,
			Action::East => x += inst.amount,
			Action::West => x -= inst.amount,
			Action::Left => angle = ((angle - inst.amount) + 360) % 360,
			Action::Right => angle = (angle + inst.amount) % 360,
			Action::Forward => {
				match angle {
					0 => y += inst.amount,
					90 => x += inst.amount,
					180 => y -= inst.amount,
					270 => x -= inst.amount,
					_ => unimplemented!(),
				}
			},
		}
	}
	x.abs() + y.abs()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> i32 {
	let instructions = parse_input(input);
	let mut x = 10;
	let mut y = 1;

	let mut ship_x = 0;
	let mut ship_y = 0;

	for inst in instructions {
		match inst.action {
			Action::North => y += inst.amount,
			Action::South => y -= inst.amount,
			Action::East => x += inst.amount,
			Action::West => x -= inst.amount,
			Action::Left => {
				match inst.amount {
					90 => {
						let t = y;
						y = x;
						x = t * -1;
					},
					180 => {
						y = y * -1;
						x = x * -1;
					},
					270 => {
						let t = y;
						y = x * -1;
						x = t;
					},
					_ => unimplemented!(),
				}
			},
			Action::Right => {
				match inst.amount {
					90 => {
						let t = y;
						y = x * -1;
						x = t;
					},
					180 => {
						y = y * -1;
						x = x * -1;
					},
					270 => {
						let t = y;
						y = x;
						x = t * -1;
					},
					_ => unimplemented!(),
				}
			},
			Action::Forward => {
				ship_x += x * inst.amount;
				ship_y += y * inst.amount;
			},
		}
	}
	ship_x.abs() + ship_y.abs()
}
