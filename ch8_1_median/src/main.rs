use std::io::stdin;

pub fn push_number(s:&String , mut v:Vec<u32>) -> Vec<u32> {
	
	if s.len() > 0 {
		let value:u32 = s.parse().unwrap();
		v.push(value);
	}
	v
}

pub fn print_vector(v:&Vec<u32>) {

	for n in v {
		print!("{n} ");
	}
	println!("");
	
}

pub fn string_to_vector(input:&String) -> Vec<u32> {
	
	let mut the_vector:Vec<u32>=Vec::new();
	
	let mut a_number=String::with_capacity(12);
	
	for c in input.chars() {
		if c >= '0' && c <= '9' {
			a_number.push(c);
			continue;
		};
		the_vector = push_number(&a_number,the_vector);
		a_number.clear();
		
	};
	the_vector
}

fn main() {
    
	let mut user_input=String::new();
	
	println!("Provide a list of numbers separated by anything that is not 0 to 9");
	_ = stdin().read_line(&mut user_input);

	let mut the_vector = string_to_vector(&user_input);
	
	
	println!("You have chosen the numbers: ");
	print_vector(&the_vector);
	the_vector.sort_unstable();
	println!("The sorted vector.");
	print_vector(&the_vector);
		
}

