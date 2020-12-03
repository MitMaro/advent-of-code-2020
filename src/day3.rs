fn count_trees(lines: &Vec<Vec<char>>, delta_x: usize, delta_y: usize) -> i32 {
	let lines_length = lines[0].len();
	let mut total = 0;
	let mut x = 0;
	let mut y = 0;

	loop {
		if lines[y][x] == '#' {
			total += 1;
		}
		x += delta_x;
		y += delta_y;

		if y >= lines.len() {
			break;
		}

		if x >= lines_length {
			x = x - lines_length;
		}
	}
	total
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
	let lines = input
		.lines()
		.map(|l| l.chars().collect::<Vec<char>>())
		.collect::<Vec<Vec<char>>>();
	count_trees(&lines, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
	let lines = input
		.lines()
		.map(|l| l.chars().collect::<Vec<char>>())
		.collect::<Vec<Vec<char>>>();
	count_trees(&lines, 1, 1)
		* count_trees(&lines, 3, 1)
		* count_trees(&lines, 5, 1)
		* count_trees(&lines, 7, 1)
		* count_trees(&lines, 1, 2)
}
