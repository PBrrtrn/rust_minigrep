use std::fs;
use std::env;
use std::error::Error;

pub fn run(program_settings: ProgramSettings) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(&program_settings.filename)?;

	let results = if program_settings.case_insensitive {
		search(&program_settings.query, &contents)
	} else {
		case_sensitive_search(&program_settings.query, &contents)
	};

	println!("{:?}", results);

	for line in results {
		println!("{}", line);
	}

	Ok(())
}

pub fn case_sensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}

	results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}

	results
}

pub struct ProgramSettings {
	pub query: String,
	pub filename: String,
	pub case_insensitive: bool,
}
impl ProgramSettings {
	pub fn new(args: &[String]) -> Result<ProgramSettings, &'static str> {
		if args.len() < 3 { return Err("Not enough parameters were passed") }
		let query = args[1].clone();
		let filename = args[2].clone();

		let case_insensitive = env::var("CASE_SENSITIVE").is_err();

		Ok(ProgramSettings { query, filename, case_insensitive })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn valid_program_settings() {
		let args = vec![String::from(""), String::from("test_query"), String::from("test_filename.txt")];

		let program_settings = ProgramSettings::new(&args).unwrap();

		assert_eq!(&program_settings.query, "test_query");
		assert_eq!(&program_settings.filename, "test_filename.txt");
	}

	#[test]
	fn search_without_matches() {
		let query = "test";
		let contents = "I don't contain any matches";

		assert_eq!(0, search(query, contents).len());
	}

	#[test]
	fn search_with_one_match() {
		let query = "test";
		let contents = "\
			I'm running a test,
			shall it debug my program, 
			then, refactor it.";

		assert_eq!(vec!["I'm running a test,"], search(query, contents));
	}

	#[test]
	fn case_sensitive() {
		let query = "TeSt";
		let contents = "\
			I'm running a test,
			shall it debug my program, 
			then, refactor it.

			'i'M rUnNinG a TeSt'";

		assert_eq!(vec!["\t\t\t'i'M rUnNinG a TeSt'"], case_sensitive_search(query, contents));
	}

/*	#[test]
	fn invalid_program_settings() {
		let args = vec![String::from("")];
		let program_settings = ProgramSettings::new(&args);

		assert_eq!(program_settings, Err("Not enough parameters were passed") );
	}*/
}