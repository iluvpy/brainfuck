use std::io::*;

pub fn interpret(input: String) {
	println!("interpreter running!");
	let mut data: Vec<u8> = vec![0];
	let mut pointer = 0;
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
			if data.len() == pointer+1 {
				data.push(0);
				pointer += 1;
			}
		} 
		else if char_ == '<' {
			data.insert(0, 0);
		}
		else if char_ == ']' {
			println!("found ]");
			if data[pointer] != 0 {
				let mut pos: usize = char_index-1;
				let mut code = String::from("fdsfds ");
				println!("found ]");
				
				while input.chars().nth(pos).unwrap() != '[' {

					code.push(input.chars().nth(pos).unwrap());
					if pos == 0 {
						break;
					}
					pos -= 1;
				}
				println!("code: {}", code);
				let code_inv: String = code.chars().rev().collect();
				println!("code inv: {}", code_inv);
				char_index = pos;
			}
		} 
		else if char_ != '[' && char_ != '\n'{
			println!("An interpretation error occured!");
			break;
		}
		char_index += 1;
	}
}