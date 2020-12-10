#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<i32> {
	let mut input = input.lines().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
	input.sort();
	input
}

#[aoc(day10, part1)]
pub fn part1(input: &Vec<i32>) -> i32 {
	let mut one_dif = 0;
	let mut three_dif = 0;
	let mut rating = 0;

	for i in input {
		let dif = i - rating;
		if dif == 3 {
			three_dif += 1;
		}
		else if dif == 1 {
			one_dif += 1;
		}
		rating += dif;
	}

	(three_dif + 1) * one_dif
}

#[derive(Debug)]
pub struct Adapter {
	pub jolts: usize,
	pub comb: Option<u64>,
}

#[aoc(day10, part2)]
pub fn part2(input: &Vec<i32>) -> u64 {
	let device_jolts = *input.last().unwrap() + 3;
	let mut input = input
		.iter()
		.map(|jolts| {
			Adapter {
				jolts: *jolts as usize,
				comb: None,
			}
		})
		.collect::<Vec<Adapter>>();
	input.insert(0, Adapter { jolts: 0, comb: None });
	input.push(Adapter {
		jolts: device_jolts as usize,
		comb: Some(1),
	});

	let mut stack = vec![0 as usize];

	's: while stack.len() > 0 {
		let i = *stack.last().unwrap();

		if input[i].comb.is_some() {
			stack.pop();
			continue;
		}

		let jolts = input[i].jolts;

		let mut j = i + 1;
		let mut comb = 0;
		loop {
			if j >= input.len() || input[j].jolts > jolts + 3 {
				break;
			}

			if let Some(c) = input[j].comb {
				comb += c;
			}
			else {
				stack.push(j);
				continue 's;
			}
			j += 1;
		}
		input[i].comb = Some(comb);
	}

	input[0].comb.unwrap()
}
