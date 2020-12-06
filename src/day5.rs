#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
	input.lines().map(String::from).collect()
}

fn get_seat_ids(tickets: &Vec<String>) -> Vec<i32> {
	let mut seat_ids = vec![];
	for ticket in tickets {
		let mut row = (0, 127);
		let mut column = (0, 7);

		for c in ticket.chars() {
			match c {
				'F' => row = (row.0, ((row.1 - row.0) as f32 / 2.0).floor() as i32 + row.0),
				'B' => row = (((row.1 - row.0) as f32 / 2.0).ceil() as i32 + row.0, row.1),
				'L' => column = (column.0, ((column.1 - column.0) as f32 / 2.0).floor() as i32 + column.0),
				'R' => column = (((column.1 - column.0) as f32 / 2.0).ceil() as i32 + column.0, column.1),
				_ => unreachable!(),
			}
		}

		seat_ids.push(row.0 * 8 + column.0);
	}
	seat_ids.sort();

	seat_ids
}

#[aoc(day5, part1)]
pub fn part1(tickets: &Vec<String>) -> i32 {
	*get_seat_ids(tickets).last().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(tickets: &Vec<String>) -> i32 {
	let seats = get_seat_ids(tickets);
	let mut last_seat = *seats.first().unwrap();
	for seat in seats.iter().skip(1) {
		if last_seat + 1 != *seat {
			return *seat - 1;
		}
		last_seat = *seat;
	}
	unreachable!();
}
