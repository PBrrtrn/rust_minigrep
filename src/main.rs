use std::env;
use std::process;

use minigrep::ProgramSettings;

fn main() {
	let args: Vec<String> = env::args().collect();

	let program_settings = ProgramSettings::new(&args).unwrap_or_else( |err| {
		println!("Problem parsing arguments: {}", err);
		process::exit(1);
	});

	if let Err(e) = minigrep::run(program_settings) {
		println!("Application error: {}", e);

		process::exit(1);
	}
}