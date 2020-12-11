#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
	Floor,
	EmptySeat,
	OccupiedSeat,
}
pub fn parse_input(input: &str) -> Vec<Vec<State>> {
	let mut grid = input
		.lines()
		.map(|v| {
			let mut line = v
				.chars()
				.map(|c| {
					match c {
						'.' => State::Floor,
						'L' => State::EmptySeat,
						'#' => State::OccupiedSeat,
						_ => unreachable!(),
					}
				})
				.collect::<Vec<State>>();
			line.insert(0, State::Floor);
			line.push(State::Floor);
			line
		})
		.collect::<Vec<Vec<State>>>();
	grid.insert(0, vec![State::Floor; grid[0].len()]);
	grid.push(vec![State::Floor; grid[0].len()]);
	grid
}

fn s(cell: &State) -> i32 {
	if *cell == State::OccupiedSeat {
		1
	}
	else {
		0
	}
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> i32 {
	let mut grid = parse_input(input);

	let grid_width = grid[0].len() - 1;
	let grid_height = grid.len() - 1;

	let mut changed = true;
	while changed {
		changed = false;
		let g = grid.clone();
		for r in 1..grid_height {
			for c in 1..grid_width {
				let neighbors = s(&g[r + 1][c - 1])
					+ s(&g[r + 1][c]) + s(&g[r + 1][c + 1])
					+ s(&g[r][c - 1]) + s(&g[r][c + 1])
					+ s(&g[r - 1][c - 1])
					+ s(&g[r - 1][c]) + s(&g[r - 1][c + 1]);

				match grid[r][c] {
					State::OccupiedSeat => {
						if neighbors >= 4 {
							changed = true;
							grid[r][c] = State::EmptySeat
						}
					},
					State::EmptySeat => {
						if neighbors == 0 {
							changed = true;
							grid[r][c] = State::OccupiedSeat
						}
					},
					_ => {},
				}
			}
		}
	}

	grid.iter().flatten().map(s).sum()
}

fn part2_neighbors(r: usize, c: usize, grid: &Vec<Vec<State>>) -> i32 {
	let mut total = 0;
	let mut row = r;
	let mut col = c;
	let grid_height = grid.len() - 1;
	let grid_width = grid[0].len() - 1;

	// &g[r + 1][c - 1]
	while row < grid_height && col > 0 {
		row += 1;
		col -= 1;
		if grid[row][col] == State::EmptySeat {
			break;
		}
		if grid[row][col] == State::OccupiedSeat {
			total += 1;
			break;
		}
	}

	row = r;
	col = c;
	// &g[r + 1][c]
	while row < grid_height {
		row += 1;
		if grid[row][col] == State::EmptySeat {
			break;
		}
		if grid[row][col] == State::OccupiedSeat {
			total += 1;
			break;
		}
	}

	row = r;
	col = c;
	// &g[r + 1][c + 1]
	while row < grid_height && col < grid_width {
		row += 1;
		col += 1;
		if grid[row][col] == State::EmptySeat {
			break;
		}
		if grid[row][col] == State::OccupiedSeat {
			total += 1;
			break;
		}
	}

	row = r;
	col = c;
	// &g[r][c - 1]
	while col > 0 {
		col -= 1;
		if grid[row][col] == State::EmptySeat {
			break;
		}
		if grid[row][col] == State::OccupiedSeat {
			total += 1;
			break;
		}
	}

	row = r;
	col = c;
	// &g[r][c + 1]
	while col < grid_width {
		col += 1;
		if grid[row][col] == State::EmptySeat {
			break;
		}
		if grid[row][col] == State::OccupiedSeat {
			total += 1;
			break;
		}
	}

	row = r;
	col = c;
	// &g[r - 1][c - 1]
	while row > 0 && col > 0 {
		row -= 1;
		col -= 1;
		if grid[row][col] == State::EmptySeat {
			break;
		}
		if grid[row][col] == State::OccupiedSeat {
			total += 1;
			break;
		}
	}

	row = r;
	col = c;
	// &g[r - 1][c]
	while row > 0 {
		row -= 1;
		if grid[row][col] == State::EmptySeat {
			break;
		}
		if grid[row][col] == State::OccupiedSeat {
			total += 1;
			break;
		}
	}

	row = r;
	col = c;
	// &g[r - 1][c + 1]
	while row > 0 && col < grid_width {
		row -= 1;
		col += 1;
		if grid[row][col] == State::EmptySeat {
			break;
		}
		if grid[row][col] == State::OccupiedSeat {
			total += 1;
			break;
		}
	}

	total
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> i32 {
	let mut grid = parse_input(input);

	let grid_width = grid[0].len() - 1;
	let grid_height = grid.len() - 1;

	let mut changed = true;
	while changed {
		changed = false;
		let g = grid.clone();
		for r in 1..grid_height {
			for c in 1..grid_width {
				match grid[r][c] {
					State::OccupiedSeat => {
						let neighbors = part2_neighbors(r, c, &g);
						if neighbors >= 5 {
							changed = true;
							grid[r][c] = State::EmptySeat
						}
					},
					State::EmptySeat => {
						let neighbors = part2_neighbors(r, c, &g);
						if neighbors == 0 {
							changed = true;
							grid[r][c] = State::OccupiedSeat
						}
					},
					_ => {},
				}
			}
		}
	}

	grid.iter().flatten().map(s).sum()
}
