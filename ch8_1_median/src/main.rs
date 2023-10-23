use std::io::stdin;

fn push_number(s:&String , mut v:Vec<u32>) -> Vec<u32> {
	
	if s.len() > 0 {
		let value:u32 = s.parse().unwrap();
		v.push(value);
	}
	v
}

fn main() {
    
	let mut user_input=String::new();
	
	println!("Provide a list of numbers separated by anything that is not 0 to 9");
	_ = stdin().read_line(&mut user_input);

	
	
	let mut the_vector:Vec<u32>=Vec::new();
	
	let mut a_number=String::with_capacity(12);
	
	for c in user_input.chars() {
		if c >= '0' && c <= '9' {
			a_number.push(c);
			continue;
		};
		the_vector = push_number(&a_number,the_vector);
		a_number.clear();
		
	}
	println!("You have chosen the numbers: ");
	for n in &the_vector {
		print!("{n} ");
	}
	println!("");
	
	
	
	
}
