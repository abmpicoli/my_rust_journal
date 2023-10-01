
// This is like a java import directive. I can comment this out and then use the fully qualified name.
//use std::io;

use rand::Rng

fn main() {


    // At first glance, invoking rand without changing cargo.toml to include the 
    // library will fail.
    //
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
	
	// ## Output WITHOUT changing cargo.toml:
	   // Compiling chapter3_guessing_game v0.1.0 (/mnt/e/projetos/my_rust_journal/chapter3_guessing_game)
	// error[E0433]: failed to resolve: use of undeclared crate or module `rand`
	  // --> src/main.rs:13:25
	   // |
	// 13 |     let secret_number = rand::Rng::rand::thread_rng().gen_range(1..=100);
	   // |                         ^^^^ use of undeclared crate or module `rand`

	// For more information about this error, try `rustc --explain E0433`.
	// error: could not compile `chapter3_guessing_game` (bin "chapter3_guessing_game") due to previous error
	
	
	
	// ## INTERESTING. I've added the cargo dependency to cargo.toml and now I see this:
	
    // Updating crates.io index
  // Downloaded rand_core v0.6.4
  // Downloaded cfg-if v1.0.0
  // Downloaded rand v0.8.5
  // Downloaded rand_chacha v0.3.1
  // Downloaded getrandom v0.2.10
  // Downloaded ppv-lite86 v0.2.17
  // Downloaded libc v0.2.148
  // Downloaded 7 crates (881.1 KB) in 3.07s
   // Compiling libc v0.2.148
   // Compiling cfg-if v1.0.0
   // Compiling ppv-lite86 v0.2.17
   // Compiling getrandom v0.2.10
   // Compiling rand_core v0.6.4
   // Compiling rand_chacha v0.3.1
   // Compiling rand v0.8.5
   // Compiling chapter3_guessing_game v0.1.0 (/mnt/e/projetos/my_rust_journal/chapter3_guessing_game)
// error[E0223]: ambiguous associated type
  // --> src/main.rs:13:25
   // |
// 13 |     let secret_number = rand::Rng::rand::thread_rng().gen_range(1..=100);
   // |                         ^^^^^^^^^^^^^^^
   // |
// help: if there were a trait named `Example` with associated type `rand` implemented for `dyn Rng`, you could use the fully-qualified path
   // |
// 13 |     let secret_number = <dyn Rng as Example>::rand::thread_rng().gen_range(1..=100);
   // |                         ~~~~~~~~~~~~~~~~~~~~~~~~~~

// error[E0782]: trait objects must include the `dyn` keyword
  // --> src/main.rs:13:25
   // |
// 13 |     let secret_number = rand::Rng::rand::thread_rng().gen_range(1..=100);
   // |                         ^^^^^^^^^
   // |
// help: add `dyn` keyword before this trait
   // |
// 13 |     let secret_number = <dyn rand::Rng>::rand::thread_rng().gen_range(1..=100);
//   |                         ++++          +
	
	
// The error start getting more and more esoteric as I try to find the recommendations... So I'm sticking with the starting example.

// BABY STEPS, RIGHT?? Falling back to exactly the example shown.


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

	// println experiments. Does println has an enhanced parser inside? I guess not. 
	// If true, the output would be `x = 5 ; x + 2 = 7 y + 2 = 12`
	let x = 5 ;
	let y = 10 ; 

	
	// println!("x = {x} ; x + 2 = {x + 2} y + 2 = {}",y+2 )

	// $ cargo run : NOPE: it would be too much :D 
	   // Compiling chapter3_guessing_game v0.1.0 (/mnt/e/projetos/my_rust_journal/chapter3_guessing_game)
	// error: invalid format string: expected `'}'`, found `'+'`
	  // --> src/main.rs:92:33
	   // |
	// 92 |     println!("x = {x} ; x + 2 = {x + 2} y + 2 = {}",y+2 )
	   // |                                 -  ^ expected `}` in format string
	   // |                                 |
	   // |                                 because of this opening brace
	   // |
	   // = note: if you intended to print `{`, you can escape it using `{{`

	// error: could not compile `chapter3_guessing_game` (bin "chapter3_guessing_game") due to previous error
	// $	
	
	println ! ("x = {x} ; x + 2 = {} ; y + 2 = {} ",x+2,y+2)
	

}
