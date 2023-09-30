
// This is like a java import directive. I can comment this out and then use the fully qualified name.
//use std::io;

fn main() {

    println! ["Hello chapter 3.2"] ;
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

    // if I comment the use `std::io` out , I may still use the io method by full invocation.
    //io::stdin()
    std::io::stdin()
		// Hm... parameter by reference? So the content can be changed? 
		// OH. It DOES make sense, right? guess is used for STORING
		// user input. 
        .read_line(&mut guess)
		
		// read_line returns a "Result" enumeration. https://doc.rust-lang.org/std/result/enum.Result.html
		// that tells if the result is ok or failure.
		
		// https://doc.rust-lang.org/std/result/enum.Result.html#method.expect 
		// .expect raises a panic.
		//
		// That is... in java this would be like a 
		// 
		
		// try { x = in.read() } catch (Exception x) { throw x }...
		// which means that I can simply ignore the expect and the default behavior is a silent crash??
		// TODO: test this tomorrow... But I think this is quite untestable...
        .expect("Failed to read line");
		

		// WOW: if you comment the .expect part out, rust gives a warning!!!! You MUST use the function output.
			// abpicoli@DESKTOP-EPFPMPH:/mnt/e/projetos/my_rust_journal/chapter3_guessing_game$ cargo run
			   // Compiling chapter3_guessing_game v0.1.0 (/mnt/e/projetos/my_rust_journal/chapter3_guessing_game)
			// warning: unused `Result` that must be used
			  // --> src/main.rs:36:5
			   // |
			// 36 | /     std::io::stdin()
			// 37 | |         // Hm... parameter by reference? So the content can be changed?
			// 38 | |         // OH. It DOES make sense, right? guess is used for STORING
			// 39 | |         // user input.
			// 40 | |         .read_line(&mut guess)
			   // | |______________________________^
			   // |
			   // = note: this `Result` may be an `Err` variant, which should be handled
			   // = note: `#[warn(unused_must_use)]` on by default
			// help: use `let _ = ...` to ignore the resulting value
			   // |
			// 36 |     let _ = std::io::stdin()
			   // |     +++++++

			// warning: `chapter3_guessing_game` (bin "chapter3_guessing_game") generated 1 warning
				// Finished dev [unoptimized + debuginfo] target(s) in 2.02s
				 // Running `target/debug/chapter3_guessing_game`
			// Hello chapter 3.2
			// Guess the number!
			// Please input your guess.


    println!("You guessed: {guess}");

}
