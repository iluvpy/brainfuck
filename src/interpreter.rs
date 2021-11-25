use std::io::*;
use std::time::{Instant};


const DATA_SIZE: usize = 512000; 

pub fn interpret(input: String) {
	println!("interpreter running!");
	let start = Instant::now();
	let mut data: [u8; DATA_SIZE] = [0; DATA_SIZE];
	let mut pointer: usize = DATA_SIZE/2;
	let mut char_index = 0;
	while char_index < input.chars().count() {
		let char_ = input.chars().nth(char_index).unwrap(); 
		if char_ == '+' {
			if data[pointer] == 255 {
				data[pointer] = 0;
			} else {
				data[pointer] += 1;
			}
		} 
		else if char_ == '-' {
			if data[pointer] == 0 {
				data[pointer] = 255;
			} else {
				data[pointer] -= 1;
			}
		} 
		else if char_ == '.' {
			print!("{}", data[pointer] as char);
		} 
		else if char_ == ',' {
			let input: Option<u8> = std::io::stdin()
				.bytes() 
				.next()
				.and_then(|result| result.ok())
				.map(|byte| byte as u8);
			data[pointer] = input.unwrap();
		} 
		else if char_ == '>' {
			pointer += 1;
		} 
		else if char_ == '<' {
			pointer -= 1;
		}
		else if char_ == '[' {
			if data[pointer] == 0 {
				let mut pos = char_index;
				let mut left = 0; // number of left brackets
				let mut right = 0; // number of right brackets

				loop {
					let current_char = input.chars().nth(pos).unwrap();
					
					if current_char == ']' {
						right += 1;
					}
					else if current_char == '[' {
						left += 1;
					}

					if current_char == ']' && left == right {
						break;
					}
					pos += 1;
				}
				char_index = pos;
				//println!("jumped to {}", input.chars().nth(pos).unwrap());
			}
		}
		else if char_ == ']' {
			if data[pointer] != 0 {
				let mut pos = char_index;
				let mut left = 0; // number of left brackets
				let mut right = 0; // number of right brackets

				loop {
					let current_char = input.chars().nth(pos).unwrap();
					
					if current_char == ']' {
						right += 1;
					}
					else if current_char == '[' {
						left += 1;
					}

					if current_char == '[' && left == right {
						break;
					}
					pos -= 1;
				}
				char_index = pos;
				//println!("jumped to {}", input.chars().nth(pos).unwrap());
			}
		}
		char_index += 1;
	}
	println!("\ninterpreter finished in {:?}", start.elapsed());
}