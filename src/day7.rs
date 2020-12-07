use std::collections::HashMap;

#[derive(Debug)]
pub struct Bag {
	pub color: String,
	pub holds_gold: Option<bool>,
	pub total_bags: Option<i32>,
	pub contains: Vec<(String, i32)>,
}

pub fn parse_input(input: &str) -> HashMap<String, Bag> {
	let mut bags = HashMap::new();
	let input = input.replace(" bags contain ", ", ").replace(".", "");
	let inputs = input
		.lines()
		.map(|v| v.split(", ").map(String::from).collect::<Vec<String>>());
	for p in inputs {
		let mut bag = Bag {
			color: p[0].clone(),
			holds_gold: None,
			total_bags: None,
			contains: vec![],
		};
		if p[1] != "no other bags" {
			for c in p.iter().skip(1) {
				let b = c
					.replace(" bags", "")
					.replace(" bag", "")
					.splitn(2, ' ')
					.map(String::from)
					.collect::<Vec<String>>();
				let count = b[0].parse::<i32>().unwrap_or(0);
				let color = b[1].clone();
				bag.contains.push((color, count));
			}
		}
		bags.insert(p[0].clone(), bag);
	}
	bags
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
	let mut input = parse_input(input);
	let mut stack = input.keys().map(String::from).collect::<Vec<String>>();

	while stack.len() > 0 {
		let color_name = String::from(stack.last().unwrap());
		let color = input.get(&color_name).unwrap();

		if color.holds_gold.is_some() {
			stack.pop();
			continue;
		}

		let mut holds_gold = false;
		let mut all_resolved = true;
		for contain in &color.contains {
			// direct contains
			if contain.0 == "shiny gold" {
				holds_gold = true;
			}
			// child bag
			else {
				let c = input.get(&contain.0).unwrap();
				if let Some(h) = c.holds_gold {
					if h {
						holds_gold = true;
					}
				}
				else {
					all_resolved = false;
					stack.push(c.color.clone());
				}
			}
		}

		if all_resolved {
			input.get_mut(&color_name).unwrap().holds_gold = Some(holds_gold);
		}
	}

	let mut total = 0;
	for v in input.values() {
		if let Some(h) = v.holds_gold {
			if h {
				total += 1;
			}
		}
	}

	total
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i32 {
	let mut input = parse_input(input);
	let mut stack = vec![String::from("shiny gold")];

	while stack.len() > 0 {
		let color_name = String::from(stack.last().unwrap());
		let color = input.get(&color_name).unwrap();

		if color.total_bags.is_some() {
			stack.pop();
			continue;
		}

		let mut subtotal = 0;
		let mut all_resolved = true;
		for contain in &color.contains {
			subtotal += contain.1;
			let c = input.get(&contain.0).unwrap();
			if let Some(t) = c.total_bags {
				subtotal += contain.1 * t;
			}
			else {
				all_resolved = false;
				stack.push(c.color.clone());
			}
		}

		if all_resolved {
			input.get_mut(&color_name).unwrap().total_bags = Some(subtotal);
		}
	}

	input.get("shiny gold").unwrap().total_bags.unwrap()
}
