use std::fs;
use std::error::Error;

pub fn run(program_settings: ProgramSettings) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(&program_settings.filename)?;

	println!("FILE HEAD: {}...", &contents[..12]);
	Ok(())
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