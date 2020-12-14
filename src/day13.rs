#[derive(Debug)]
pub struct BusRoute {
	id: Option<u64>,
}

pub struct Notes {
	target: u64,
	bus_routes: Vec<BusRoute>,
}

pub fn parse_input(input: &str) -> Notes {
	let mut lines = input.lines();

	let target = lines.next().unwrap().parse::<u64>().unwrap();

	let bus_routes = lines
		.next()
		.unwrap()
		.split(",")
		.map(|v| {
			if v == "x" {
				BusRoute { id: None }
			}
			else {
				BusRoute {
					id: Some(v.parse::<u64>().unwrap()),
				}
			}
		})
		.collect::<Vec<BusRoute>>();

	Notes { target, bus_routes }
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> u64 {
	let notes = parse_input(input);
	let mut best_id = 0;
	let mut best_difference = u64::max_value();

	for route in notes.bus_routes {
		if let Some(id) = route.id {
			let value = (f64::ceil(notes.target as f64 / id as f64) * id as f64) as u64 - notes.target;

			if value < best_difference {
				best_difference = value;
				best_id = id;
			}
		}
	}

	best_id * best_difference
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> u64 {
	let notes = parse_input(input);

	let mut t = 1;
	let mut m = 1;
	for (i, route) in notes.bus_routes.iter().enumerate() {
		if let Some(id) = route.id {
			while (t + i as u64) % id != 0 {
				t += m;
			}
			m *= id;
		}
	}

	t
}
