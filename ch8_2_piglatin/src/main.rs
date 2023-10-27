use std::io::stdin;

fn pig_latin(input:&str) -> &str {
	
	let mut output:String = String::new();
	let mut word:String=String::with_capacity(15);
	let mut ending:String=String::with_capacity(15);
	let mut force_uppercase:bool=false;
	
	for x in input.chars() {
		
		if x.is_alphabetic() {
			if ending.is_empty() {
				let lower_char = x.to_lowercase().to_string();
				let first_char = match lower_char { 
				
					'a' | 'e' | 'i' | 'o' | 'u' => {
						word.push(x);
						'h'
					}
					,
					_ => {
						force_uppercase = x.is_uppercase();
						lower_char
					}
					
				
				};
				ending.push(first_char);
				ending.push_str("ay");
			} else {
				if force_uppercase {
					word.push_str(x.to_uppercase().to_string());
					force_uppercase = false;
				} else {
					word.push(x);
				}
			}
			
		} else {
			if ! word.is_empty() {
				output.push_str(word);
				output.push_str(ending);
				word.clear();
				ending.clear();
			}
			output.push(x);
		}
	
	}
	output.push(word);
	output.push(ending);
	
	return output.as_str();
	
}

fn main() {
    
	let mut user_input=String::new();
	
	println!("Piglatin translator: write a phrase to be converted into piglatin");
	_ = stdin().read_line(&mut user_input);
	
	
}



#[cfg(test)]
mod tests {

	use crate::*;	
	#[test]
	fn test_pig_latin() {
		
		
		assert_eq!("Hetay_uickqay rownbay oxfay umpsjay overhay hetay azylay ogday",pig_latin("The_quick brown fox jumps over the lazy dog"));
		assert_eq!("Hohay! Ueqay iaday alavilhosomay!",pig_latin("Oh! Que dia malavilhoso!"));
		assert_eq!("Ehay aihay eivay! Elezabay??","E ai vei! Beleza??");
		
		
	}
	
}
