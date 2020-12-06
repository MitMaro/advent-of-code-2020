#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<String> {
	let input = input.replace("\n\n", "\t").replace("\n", " ");
	input.split("\t").map(String::from).collect::<Vec<String>>()
}

#[aoc(day6, part1)]
pub fn part1(groups: &Vec<String>) -> usize {
	let mut total = 0;
	for group in groups {
		let mut answers = vec![];
		for answer in group.chars() {
			if answer != ' ' && !answers.contains(&answer) {
				answers.push(answer);
			}
		}
		total += answers.len();
	}

	total
}

#[aoc(day6, part2)]
pub fn part2(groups: &Vec<String>) -> usize {
	let mut total = 0;
	for group in groups {
		let mut it = group.split(" ");
		let mut answers: Vec<char> = it.next().unwrap().chars().collect::<Vec<char>>();
		for a in it {
			answers.retain(|&v| a.contains(v));
		}
		total += answers.len();
	}

	total
}
