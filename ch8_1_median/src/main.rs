use std::io::stdin;
use std::collections::HashMap;

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

pub fn median(the_vector:&mut Vec<u32>) -> f64 {
	the_vector.sort_unstable();
	let vec_len = the_vector.len();
	let is_odd = vec_len % 2 == 1 ;
	
	if is_odd {
		let median:f64 =  the_vector[vec_len / 2 ].into();
		return median;
	} 
	let middle = vec_len / 2; 
	let middle_right:f64 = the_vector[middle].into();
	let middle_left:f64 = the_vector[middle-1].into();
	let median:f64 = ( middle_left + middle_right)/2.0;
	median
}

fn main() {
    
	let mut user_input=String::new();
	
	println!("Provide a list of numbers separated by anything that is not 0 to 9");
	_ = stdin().read_line(&mut user_input);

	let mut the_vector = string_to_vector(&user_input);
	
	
	println!("You have chosen the numbers: ");
	print_vector(&the_vector);
	
	let the_median = median(&mut the_vector);
	println!("Median = {the_median}");
	println!("Mode:");
	print_vector(&(mode(&the_vector)));
}

fn mode(the_vector: &Vec<u32>)-> Vec<u32> {
	
	let mut frequencies_by_key:HashMap<u32,u32> = HashMap::new();
	let mut mode_frequency:u32 = 1;
	for key in the_vector {
		
		match frequencies_by_key.get_mut(&key) {
			Some(x) => {
				*x = *x + 1 ;
				if *x > mode_frequency {
					mode_frequency = *x;
				}
			}
			None => {
				frequencies_by_key.insert(*key,1);
			}
		}
		
	}
	let mut vec_out: Vec<u32> =Vec::new();
	for x in &frequencies_by_key {
		if *x.1 == mode_frequency {
			vec_out.push(*x.0);
		}
	}
	vec_out.sort_unstable();
	vec_out
	
}

#[cfg(test)]
mod tests {

	use crate::*;	
	#[test]
	fn test_median() {
		let mut the_vector:Vec<u32>=vec![1,2,3,4,5];
		
		let the_median = median(&mut the_vector);
		
		assert_eq!(3.0,the_median,"The median of 1,2,3,4,5 should be the middle value, 3");
		
		the_vector = vec![1,2,3,4,5,6];
		let the_median = median(&mut the_vector);
		assert_eq!(3.5,the_median,"The median of 1,2,3,4,5,6 should be the average of the central values: (3+4)/2 = 3.5");
		
		the_vector = vec![1,4,5,6,3,2];
		let the_median = median(&mut the_vector);
		assert_eq!(3.5,the_median,"The median of 1,2,3,4,5,6, even if out of order, should be the average of the central values: (3+4)/2 = 3.5");
	}
	#[test]
	fn test_mode() {
		assert_eq!(vec![2,3],mode(&vec![1,2,2,3,3,4]));
		assert_eq!(vec![2],mode(&vec![1,4,5,3,2,2]));
		assert_eq!(vec![1,2,3,4,5,6],mode(&vec![1,2,3,4,5,6]));
		
	}
}

