use std::env;
use std::fs;
use std::path::Path;

mod interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
	let input: String;
	if args.len() > 1 {
		input = args[1].clone();
	}
	else {
		return;
	}

	let is_file = Path::new(&input).is_file();
	let mut content = input.clone();
	if is_file {
		content = match fs::read_to_string(input) {
			Err(why) => panic!("couldnt read file! reason: {}", why),
			Ok(content) => content,
		};
	}
	
	interpreter::interpret(content);

}
