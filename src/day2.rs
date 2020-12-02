#[derive(Debug)]
pub struct Password {
	pub min: usize,
	pub max: usize,
	pub check: char,
	pub password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
	input
		.lines()
		.map(|input| {
			let mut a = input.split(':');
			let mut b = a.next().unwrap().split(' ');
			let mut c = b.next().unwrap().split('-');
			Password {
				min: c.next().unwrap().parse().unwrap(),
				max: c.next().unwrap().parse().unwrap(),
				check: b.next().unwrap().chars().next().unwrap(),
				password: String::from(a.next().unwrap().trim()),
			}
		})
		.collect::<Vec<Password>>()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Password>) -> i32 {
	let mut total = 0;
	'p: for password in input.iter() {
		let mut count = 0;
		for p in password.password.chars() {
			if p == password.check {
				count += 1;
				if count > password.max {
					continue 'p;
				}
			}
		}
		if count >= password.min {
			total += 1;
		}
	}
	total
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Password>) -> i32 {
	let mut total = 0;
	for password in input.iter() {
		let char_1 = password.password.chars().skip(password.min - 1).next().unwrap();
		let char_2 = password.password.chars().skip(password.max - 1).next().unwrap();

		if char_1 == password.check && char_2 != password.check {
			total += 1;
		}
		else if char_1 != password.check && char_2 == password.check {
			total += 1;
		}
	}
	total
}
