use regex::Regex;

fn parse_number(i: &str) -> i32 {
	i.parse().unwrap_or(0)
}

#[derive(Debug)]
pub struct Passport {
	byr: Option<i32>,
	iyr: Option<i32>,
	eyr: Option<i32>,
	hgt: Option<(i32, String)>,
	hcl: Option<String>,
	ecl: Option<String>,
	pid: Option<String>,
	cid: Option<String>,
	hair_regex: Regex,
	pid_regex: Regex,
}

impl Passport {
	fn new() -> Self {
		Self {
			byr: None,
			iyr: None,
			eyr: None,
			hgt: None,
			hcl: None,
			ecl: None,
			pid: None,
			cid: None,
			hair_regex: Regex::new(r"^#[0-9a-f]{6}$").unwrap(),
			pid_regex: Regex::new(r"^[0-9]{9}$").unwrap(),
		}
	}

	fn set_field(&mut self, value: &str) {
		let v = value.split(':').collect::<Vec<&str>>();
		match v[0] {
			"byr" => self.byr = Some(parse_number(v[1])),
			"iyr" => self.iyr = Some(parse_number(v[1])),
			"eyr" => self.eyr = Some(parse_number(v[1])),
			"hgt" => {
				self.hgt = {
					if v[1].contains("cm") {
						Some((parse_number(v[1].replace("cm", "").as_str()), String::from("cm")))
					}
					else {
						Some((parse_number(v[1].replace("in", "").as_str()), String::from("in")))
					}
				}
			},
			"hcl" => self.hcl = Some(String::from(v[1])),
			"ecl" => self.ecl = Some(String::from(v[1])),
			"pid" => self.pid = Some(String::from(v[1])),
			"cid" => self.cid = Some(String::from(v[1])),
			_ => unimplemented!(),
		}
	}

	fn has_required(&self) -> bool {
		self.byr.is_some()
			&& self.iyr.is_some()
			&& self.eyr.is_some()
			&& self.hgt.is_some()
			&& self.hcl.is_some()
			&& self.ecl.is_some()
			&& self.pid.is_some()
	}

	fn is_valid(&self) -> bool {
		if !self.has_required() {
			return false;
		}

		let byr = self.byr.unwrap();
		let iyr = self.iyr.unwrap();
		let eyr = self.eyr.unwrap();
		let hgt = self.hgt.as_ref().unwrap();
		let hcl = self.hcl.as_ref().unwrap();
		let ecl = self.ecl.as_ref().unwrap();
		let pid = self.pid.as_ref().unwrap();

		if !(byr >= 1920 && byr <= 2002 && iyr >= 2010 && iyr <= 2020 && eyr >= 2020 && eyr <= 2030) {
			return false;
		}

		if !((hgt.1 == "cm" && hgt.0 >= 150 && hgt.0 <= 193) || (hgt.1 == "in" && hgt.0 >= 59 && hgt.0 <= 76)) {
			return false;
		}

		if !self.hair_regex.is_match(hcl.as_str()) {
			return false;
		}

		if !(ecl == "amb"
			|| ecl == "blu"
			|| ecl == "brn"
			|| ecl == "gry"
			|| ecl == "grn"
			|| ecl == "hzl"
			|| ecl == "oth")
		{
			return false;
		}

		if !self.pid_regex.is_match(pid.as_str()) {
			return false;
		}

		true
	}
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
	let input = input.replace("\n\n", "\t").replace("\n", " ");

	input
		.split("\t")
		.map(|p| p.split(" ").map(String::from).collect::<Vec<String>>())
		.collect::<Vec<Vec<String>>>()
}

#[aoc(day4, part1)]
pub fn part1(passports: &Vec<Vec<String>>) -> i32 {
	let mut total = 0;
	for passport in passports {
		let mut p = Passport::new();
		for field in passport {
			p.set_field(field);
		}
		if p.has_required() {
			total += 1;
		}
	}
	total
}

#[aoc(day4, part2)]
pub fn part2(passports: &Vec<Vec<String>>) -> i32 {
	let mut total = 0;
	for passport in passports {
		let mut p = Passport::new();
		for field in passport {
			p.set_field(field);
		}
		if p.is_valid() {
			total += 1;
		}
	}
	total
}
