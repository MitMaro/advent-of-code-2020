pub fn parse_input(input: &str) -> Vec<Vec<Vec<bool>>> {
	let pc = 7;
	let mut grid = input
		.lines()
		.map(|v| {
			let mut line = v.chars().map(|c| c == '#').collect::<Vec<bool>>();
			for _ in 0..pc {
				line.insert(0, false);
				line.push(false);
			}
			line
		})
		.collect::<Vec<Vec<bool>>>();
	for _ in 0..pc {
		grid.insert(0, vec![false; grid[0].len()]);
		grid.push(vec![false; grid[0].len()]);
	}

	let grid_width = grid[0].len();
	let grid_height = grid.len();

	let mut cube = vec![grid];
	for _ in 0..pc {
		cube.insert(0, vec![vec![false; grid_width]; grid_height]);
		cube.push(vec![vec![false; grid_width]; grid_height]);
	}

	cube
}

fn count_neighbours(col: usize, row: usize, dep: usize, cube: &Vec<Vec<Vec<bool>>>) -> u32 {
	let mut total = 0;
	for d in dep - 1..=dep + 1 {
		for r in row - 1..=row + 1 {
			for c in col - 1..=col + 1 {
				total += if cube[d][c][r] { 1 } else { 0 };
			}
		}
	}

	total - if cube[dep][col][row] { 1 } else { 0 }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> i32 {
	let mut cube = parse_input(input);

	let grid_width = cube[0][0].len() - 1;
	let grid_height = cube[0].len() - 1;
	let grid_depth = cube.len() - 1;

	for _ in 0..6 {
		let g = cube.clone();
		for d in 1..grid_depth {
			for r in 1..grid_height {
				for c in 1..grid_width {
					let neighbors = count_neighbours(c, r, d, &g);
					cube[d][c][r] = if g[d][c][r] {
						neighbors == 2 || neighbors == 3
					}
					else {
						neighbors == 3
					}
				}
			}
		}
	}

	cube.iter()
		.flatten()
		.flatten()
		.map(|v| {
			if *v {
				1
			}
			else {
				0
			}
		})
		.sum()
}

fn count_neighbours_2(col: usize, row: usize, dep: usize, ww: usize, www: &Vec<Vec<Vec<Vec<bool>>>>) -> u32 {
	let mut total = 0;
	for w in ww - 1..=ww + 1 {
		for d in dep - 1..=dep + 1 {
			for r in row - 1..=row + 1 {
				for c in col - 1..=col + 1 {
					total += if www[w][d][c][r] { 1 } else { 0 };
				}
			}
		}
	}

	total - if www[ww][dep][col][row] { 1 } else { 0 }
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> i32 {
	let cube = parse_input(input);

	let grid_width = cube[0][0].len() - 1;
	let grid_height = cube[0].len() - 1;
	let grid_depth = cube.len() - 1;

	let mut altcube = vec![cube];
	for _ in 0..7 {
		altcube.insert(0, vec![
			vec![vec![false; grid_width + 1]; grid_height + 1];
			grid_depth + 1
		]);
		altcube.push(vec![vec![vec![false; grid_width + 1]; grid_height + 1]; grid_depth + 1]);
	}

	let grid_w = altcube.len() - 1;

	for _ in 0..6 {
		let g = altcube.clone();
		for w in 1..grid_w {
			for d in 1..grid_depth {
				for r in 1..grid_height {
					for c in 1..grid_width {
						let neighbors = count_neighbours_2(c, r, d, w, &g);
						altcube[w][d][c][r] = if g[w][d][c][r] {
							neighbors == 2 || neighbors == 3
						}
						else {
							neighbors == 3
						}
					}
				}
			}
		}
	}

	altcube
		.iter()
		.flatten()
		.flatten()
		.flatten()
		.map(|v| {
			if *v {
				1
			}
			else {
				0
			}
		})
		.sum()
}
