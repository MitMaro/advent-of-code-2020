fn calc(exp: &str) -> u64 {
	let p = exp.split(" ");
	let mut total = 0;
	let mut op = '+';
	for t in p {
		match t {
			"+" => op = '+',
			"*" => op = '*',
			_ => {
				let n = t.parse::<u64>().unwrap();
				if op == '+' {
					total += n;
				}
				else {
					total *= n;
				}
			},
		}
	}
	total
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u64 {
	let mut total = 0;
	'l: for line in input.lines() {
		let mut line = format!("({})", line);
		let mut s_i = 0;
		let mut e_i = 0;
		'o: loop {
			if s_i == 0 && e_i == line.len() - 1 {
				total += calc(&line[1..line.len() - 1]);
				continue 'l;
			}
			if s_i != e_i {
				let sub = calc(&line[s_i + 1..e_i]);
				line.replace_range(s_i..=e_i, format!("{}", sub).as_str());
			}
			s_i = 0;
			let mut it = line.chars();
			let mut index = 0;
			loop {
				if let Some(t) = it.next() {
					if t == '(' {
						s_i = index;
					}
					if t == ')' {
						e_i = index;
						continue 'o;
					}
				}
				else {
					break;
				}
				index += 1;
			}
		}
	}
	total
}

fn calc_2(exp: &str) -> u64 {
	let mut p = exp.split(" ");

	let mut tokens = vec![];
	let mut t_1 = p.next().unwrap().parse::<u64>().unwrap();
	while let Some(t) = p.next() {
		match t {
			"+" => {
				t_1 += p.next().unwrap().parse::<u64>().unwrap();
			},
			"*" => {
				tokens.push(t_1);
				t_1 = p.next().unwrap().parse::<u64>().unwrap();
			},
			_ => unreachable!(),
		}
	}
	tokens.push(t_1);

	let mut total = 1;
	for t in tokens {
		total *= t;
	}
	total
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> u64 {
	let mut total = 0;
	'l: for line in input.lines() {
		eprintln!("{}", line);
		let mut line = format!("({})", line);
		let mut s_i = 0;
		let mut e_i = 0;
		'o: loop {
			if s_i == 0 && e_i == line.len() - 1 {
				total += calc_2(&line[1..line.len() - 1]);
				continue 'l;
			}
			if s_i != e_i {
				let sub = calc_2(&line[s_i + 1..e_i]);
				line.replace_range(s_i..=e_i, format!("{}", sub).as_str());
			}
			s_i = 0;
			let mut it = line.chars();
			let mut index = 0;
			loop {
				if let Some(t) = it.next() {
					if t == '(' {
						s_i = index;
					}
					if t == ')' {
						e_i = index;
						continue 'o;
					}
				}
				else {
					break;
				}
				index += 1;
			}
		}
	}
	total
}
