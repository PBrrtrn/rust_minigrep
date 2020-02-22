use std::fs;
use std::error::Error;

pub fn run(program_settings: ProgramSettings) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(&program_settings.filename)?;

	for line in search(&program_settings.query, &contents){
		println!("{}", line);
	}
	Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}

	results
}

pub struct ProgramSettings {
	pub query: String,
	pub filename: String,
}
impl ProgramSettings {
	pub fn new(args: &[String]) -> Result<ProgramSettings, &'static str> {
		if args.len() < 3 { return Err("Not enough parameters were passed") }
		let query = args[1].clone();
		let filename = args[2].clone();

		Ok(ProgramSettings { query, filename })
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
	fn search_with_one_result() {
		let query = "test";
		let contents = "\
			I'm running a test,
			shall it debug my program, 
			then, refactor it.";

		assert_eq!(vec!["I'm running a test,"], search(query, contents));
	}

/*	#[test]
	fn invalid_program_settings() {
		let args = vec![String::from("")];
		let program_settings = ProgramSettings::new(&args);

		assert_eq!(program_settings, Err("Not enough parameters were passed") );
	}*/
}