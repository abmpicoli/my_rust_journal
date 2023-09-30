use std::io;

fn main() {

    println! ["Hello chapter 3.1"] ;
    println!("Guess the number!");

    println!("Please input your guess.");

	// comments in rust are like single line comments in java.
	// it seems there are no /* */ comments, though.
	
	// let : BASIC nostalgia here :D 
	// "mut": variables in RUST are immutable by default. so if we 
	// need something that receives user input (mutable data)
	// we need to place the mut prefix.
	
	// In rust: ::new() is only a METHOD to create an empty string.
	// it is a convention: not a constructor.
	
	// https://doc.rust-lang.org/reference/keywords.html shows what are the rust keywords.
	// "new" is not one of them, like in java where we would write (in this context, a writable string, it would 
	// a a stringbuidler. So `StringBuilder guess = new StringBuilder()`
	// 
	// So it is not like everything must be an object... like java. Rust is not an object oriented language
	// necessarily.
	//
	// Which also means that the String in rust is MUTABLE?
	
    let mut guess = String::new();

    io::stdin()

		// Hm... parameter by reference? So the content can be changed? 
		// OH. It DOES make sense, right? guess is used for STORING
		// user input. 
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

}
